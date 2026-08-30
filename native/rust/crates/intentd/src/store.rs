//! The ONLY way a request handler reaches a project's data.
//!
//! **THIS MODULE EXISTS TO MAKE `AC-08.11`'s STRUCTURAL OBLIGATION HOLD BY
//! CONSTRUCTION RATHER THAN BY DISCIPLINE.** The row requires that no blocking
//! store call ever occupies an async worker thread, and the reason is precise:
//! per-connection tasks order operations WITHIN a connection, while the
//! starvation that defeats the client's deadline happens BETWEEN them. The
//! accept loop is itself a task on the same runtime, so N concurrent blocking
//! `rusqlite` calls on N workers leave the probe's connection sitting in the
//! kernel backlog until its deadline expires -- and the client then routes
//! in-process against a store this daemon owns, which is the exact failure the
//! routing rule exists to prevent.
//!
//! **SO THE `Facade` NEVER LEAVES THIS FILE.** It is owned by a plain
//! `std::thread` and reached only through [`ProjectHandle`], whose methods are
//! all `async` and none of which can block a worker: the send is asynchronous,
//! the wait is a `oneshot`, and the blocking work happens on a thread the
//! runtime does not schedule. A handler cannot reach a blocking handle because
//! it is never given a type that has one.
//!
//! **THE COMPILER IS THEREFORE THE WITNESS, WHICH IS WHAT THE ROW ASKED FOR.**
//! A latency test would have measured the machine rather than the code and
//! passed with every line of this discipline deleted. What holds here is that
//! `Facade` does not appear in any other module of this crate -- checked
//! mechanically by `tests/one_store_door.rs`, in the shape `dep_graph_guard.rs`
//! uses for D06, so the arrangement cannot be quietly undone by someone who
//! reasonably wanted a store handle where they were working.
//!
//! **A THREAD PER PROJECT RATHER THAN `spawn_blocking`, AND THE REASON IS
//! OWNERSHIP.** `Facade`'s mutating methods take `&mut self`, so the handle
//! cannot be shared across tasks under any design -- something has to own it
//! exclusively. A dedicated thread makes that ownership the thread's, which
//! also serialises every operation on one project without a lock anywhere:
//! requests queue in the channel and are served in order. `spawn_blocking`
//! would have needed a mutex around the facade to achieve the same thing, and
//! a mutex held across a blocking call is the shape this module exists to avoid.

use std::path::{Path, PathBuf};

use intentsvcs::facade::{Facade, FacadeContext};
use intentsvcs::project::Project;
use intentsvcs::remedy::Remedy;
use intentsvcs::wire::{Op, Response, ThreadSummary};
use tokio::sync::{mpsc, oneshot};

/// How many requests may be queued for one project before callers wait.
///
/// **BOUNDED, SO A CLIENT CANNOT MAKE THE DAEMON BUY MEMORY ON ITS BEHALF.** An
/// unbounded queue turns a slow store into unbounded growth; a bounded one
/// turns it into backpressure, where the caller waits and the daemon stays the
/// size it was. The wait is asynchronous, so a full queue delays the requests
/// for THAT project and no others -- and never the accept loop.
const QUEUE_DEPTH: usize = 64;

/// One unit of work for a project's store thread.
struct Job {
  op: Op,
  reply: oneshot::Sender<Response>,
}

/// A handle to one project's store.
///
/// Cloneable and shareable: it is a channel sender, not a database handle. That
/// difference is the whole design -- see the module note.
#[derive(Debug, Clone)]
pub struct ProjectHandle {
  tx: mpsc::Sender<Job>,
  root: PathBuf,
}

impl ProjectHandle {
  /// Open a project and start the thread that owns its store.
  ///
  /// **THE OPEN HAPPENS ON THE STORE THREAD, NOT HERE.** `Facade::open` runs
  /// migrations and reads canon; doing it on the caller's task would block a
  /// worker for exactly as long as the thing this module exists to keep off
  /// them. So the thread is started first and reports back whether it got a
  /// facade, which also means the failure path returns a `Response` a client
  /// can be told rather than an error only a log would see.
  pub async fn open(root: PathBuf) -> Result<ProjectHandle, Response> {
    let (tx, mut rx) = mpsc::channel::<Job>(QUEUE_DEPTH);
    let (ready_tx, ready_rx) = oneshot::channel::<Result<(), Response>>();
    let thread_root = root.clone();

    std::thread::spawn(move || {
      let mut facade = match open_facade(&thread_root) {
        Ok(facade) => {
          if ready_tx.send(Ok(())).is_err() {
            // Nobody is waiting any more: the caller went away between the
            // spawn and the open. Drop the facade rather than serve a queue no
            // one will read.
            return;
          }
          facade
        }
        Err(refusal) => {
          let _ = ready_tx.send(Err(refusal));
          return;
        }
      };

      // `blocking_recv` is correct HERE and would be a defect anywhere else in
      // this crate: this is not a runtime worker, it is a thread whose entire
      // job is to block.
      while let Some(job) = rx.blocking_recv() {
        let response = serve(&mut facade, job.op);
        // A dropped receiver means the client disconnected mid-request. The
        // work is already done and there is nobody to tell, which is a state
        // rather than a fault.
        let _ = job.reply.send(response);
      }
    });

    match ready_rx.await {
      Ok(Ok(())) => Ok(ProjectHandle { tx, root }),
      Ok(Err(refusal)) => Err(refusal),
      // The thread ended without answering: it panicked. Report it as a
      // refusal rather than letting the caller wait on a channel nothing will
      // ever write to.
      Err(_) => Err(Response::error(
        format!(
          "the store thread for `{}` stopped before it could open the project",
          root.display()
        ),
        "this is a fault in intentd rather than in the project. Restart the daemon; if it recurs, the daemon's log names the panic.",
      )),
    }
  }

  /// Ask the store, without occupying a worker while it answers.
  pub async fn call(&self, op: Op) -> Response {
    let (reply_tx, reply_rx) = oneshot::channel();
    if self
      .tx
      .send(Job {
        op,
        reply: reply_tx,
      })
      .await
      .is_err()
    {
      return Response::error(
        format!(
          "the store for `{}` is no longer running",
          self.root.display()
        ),
        "the project's store thread has ended, which it does only on a panic. Restart the daemon.",
      );
    }
    match reply_rx.await {
      Ok(response) => response,
      Err(_) => Response::error(
        format!(
          "the store for `{}` stopped while serving this request",
          self.root.display()
        ),
        "the request was accepted and the store thread ended before answering. Restart the daemon; the log names the panic.",
      ),
    }
  }
}

/// Open the facade for a root, turning every failure into something sayable.
fn open_facade(root: &Path) -> Result<Facade, Response> {
  let project = Project::open(root).map_err(|e| {
    Response::error(
      format!("{e}"),
      format!(
        "`{}` is registered with this daemon and is not an Intent project. Run `intent init` there, or the directory has been replaced since the daemon first opened it.",
        root.display()
      ),
    )
  })?;
  let ctx = FacadeContext {
    // **THE DAEMON IS NOT THE AUTHOR AND MUST NOT CLAIM TO BE.** `principal`
    // reaches the event log, so a daemon writing "local" would attribute every
    // client's work to itself. Naming the daemon is honest today, when no
    // request carries an identity; when one does, it comes off the request.
    principal: "intentd".to_string(),
    project_id: project.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  Facade::open(project, ctx).map_err(|e| Response::error(format!("{e}"), e.remedy()))
}

/// Serve one project-scoped operation.
///
/// **THE ONLY FUNCTION IN THIS CRATE THAT TOUCHES A `Facade`, AND IT RUNS ON
/// THE STORE THREAD.** Adding an operation means adding an arm here; there is
/// nowhere else it could be added, which is the property the door is for.
fn serve(facade: &mut Facade, op: Op) -> Response {
  match op {
    Op::ThreadList => Response::Threads {
      threads: facade
        .st_list()
        .into_iter()
        .map(|thread| ThreadSummary {
          id: thread.id.clone(),
          title: thread.title.clone(),
          status: thread.status,
        })
        .collect(),
    },
    // **UNREACHABLE BY DISPATCH AND ANSWERED ANYWAY.** The registry is not
    // scoped to a project, so the connection handler answers it before any
    // handle is involved. If that dispatch ever changes, a caller arriving here
    // gets a refusal that names the mistake rather than a panic or a wrong
    // answer -- the arm costs two lines and removes a whole class of silent
    // misrouting.
    Op::Registry => Response::error(
      "the registry is not a project-scoped operation and reached a project's store",
      "this is a routing fault inside intentd. The registry is answered by the daemon itself, not by any project.",
    ),
  }
}
