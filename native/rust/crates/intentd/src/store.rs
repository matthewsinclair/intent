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
use intentsvcs::wire::{Event, Op, Response, ThreadSummary};
use tokio::sync::{broadcast, mpsc, oneshot};

/// How many requests may be queued for one project before callers wait.
///
/// **BOUNDED, SO A CLIENT CANNOT MAKE THE DAEMON BUY MEMORY ON ITS BEHALF.** An
/// unbounded queue turns a slow store into unbounded growth; a bounded one
/// turns it into backpressure, where the caller waits and the daemon stays the
/// size it was. The wait is asynchronous, so a full queue delays the requests
/// for THAT project and no others -- and never the accept loop.
const QUEUE_DEPTH: usize = 64;

/// One unit of work for a project's store thread.
///
/// **TWO ARMS BECAUSE THERE ARE TWO ORIGINS, AND CONFLATING THEM WOULD BREAK
/// THE ONE DISCRIMINATOR `AC-08.2` RESTS ON.** A client request is counted,
/// answered and attributable; a watcher-driven ingest has no client, nobody to
/// answer, and must NOT move [`ProjectHandle::dispatched`] -- that counter says
/// *how many ops a CLIENT routed here*, and a background ingest landing inside
/// a harness's before-and-after bracket would report `+2` for a single verb and
/// make the conformance test flake against a mechanism it does not know about.
///
/// **THE SEPARATION IS STRUCTURAL RATHER THAN A BRANCH.** The count lives in
/// [`ProjectHandle::call`], which is the client door; the watcher reaches
/// [`ProjectHandle::ingest`], which is a different door and cannot increment
/// anything. A `skip if this was internal` branch inside one shared path is
/// exactly where such an exemption goes quietly wrong.
enum Work {
  /// A client asked something and is waiting for the answer.
  Client {
    op: Op,
    reply: oneshot::Sender<Response>,
  },
  /// The watcher saw the project's tree change (`AC-08.5`).
  Ingest,
  /// The backup sweep came round and this project should decide (`AC-08.8`).
  ///
  /// **IT IS *CONSIDER*, NOT *DO*, AND THE DIFFERENCE IS WHERE THE DECISION
  /// LIVES.** The sweeper knows only that some time has passed; whether a
  /// backup is due is a question about this project's configured period and
  /// the age of its newest good snapshot, and BOTH of those are readable only
  /// here. A sweeper that decided would have to open the store to find out,
  /// which is the second engine this whole module exists to prevent.
  ///
  /// **LIKE [`Work::Ingest`] IT IS AN INTERNAL DOOR AND NOT A WIRE `Op`.** No
  /// client asks for it, so putting it on the wire would publish a verb whose
  /// only caller is in this process -- and it must not move
  /// [`ProjectHandle::dispatched`], which counts what CLIENTS routed here.
  Backup,
}

/// A handle to one project's store.
///
/// Cloneable and shareable: it is a channel sender, not a database handle. That
/// difference is the whole design -- see the module note.
/// **NOT `Clone`, AND THE COUNTER IS WHY IT MUST NOT BE.** The handle is always
/// reached through an `Arc` -- the registry stores `Arc<ProjectHandle>` and hands
/// out clones of THAT -- so deriving `Clone` here was never used. It is removed
/// rather than left harmless: **a cloned handle would carry a fresh
/// [`ProjectHandle::dispatched`] while addressing the same store**, so one
/// project would have two independent counts and the registry would report
/// whichever copy it happened to hold. A discriminator that can be halved by an
/// `#[derive]` is not one.
#[derive(Debug)]
pub struct ProjectHandle {
  tx: mpsc::Sender<Work>,
  root: PathBuf,
  /// Ops this handle has dispatched to its store.
  ///
  /// **IT LIVES ON THE HANDLE AND IS INCREMENTED INSIDE [`ProjectHandle::call`],
  /// WHICH MAKES `counts dispatched ops and nothing else` STRUCTURAL RATHER THAN
  /// A RULE SOMEBODY FOLLOWS.** Counting in the connection handler would have
  /// needed a branch to skip liveness probes and the registry, and a branch is
  /// exactly where that exemption goes quietly wrong -- **if the probe
  /// incremented, every fallthrough would increment and the discriminator would
  /// be vacuous**, which is the defect the counter exists to catch, arriving
  /// through the mechanism built to avoid it.
  ///
  /// Nothing but a dispatched op reaches this line: probes are answered before
  /// dispatch, and `Op::Registry` never reaches a handle at all. See
  /// `intentsvcs::wire::UNCOUNTED`, which declares that set so the harness can
  /// form an expectation from it rather than from a list of its own.
  dispatched: std::sync::atomic::AtomicU64,
  /// Ingests this project's tree has driven (`AC-08.5`).
  ///
  /// **INCREMENTED WHERE THE WORK IS DONE, NOT WHERE IT IS REQUESTED, AND THAT
  /// IS THE OPPOSITE CHOICE FROM [`ProjectHandle::dispatched`].** The two
  /// counters answer different questions, so they are counted at different
  /// points on purpose. `dispatched` answers *did a client route here*, which
  /// is true the moment the request is sent and stays true if the store then
  /// fails. This one answers *how many times did the tree actually get
  /// re-read*, which is a claim about work performed -- and counting it at the
  /// send would make a full queue look like completed ingests.
  ingested: std::sync::Arc<std::sync::atomic::AtomicU64>,
  /// This project's identity, as `D20`'s events name it.
  ///
  /// **READ ONCE WHEN THE PROJECT OPENS, NEVER PER EVENT.** It comes off the
  /// project's config, which the store thread has already read to build its
  /// `FacadeContext` -- so asking again per event would be a second reader of
  /// one value, on the hot path, for a fact that cannot change while the handle
  /// lives.
  project_id: String,
  /// The live feed for this project (`AC-08.6`).
  ///
  /// **THE EVENT BUS SITS BESIDE THE STORE DOOR AND DOES NOT WIDEN IT.** This
  /// module's rule is that a `Facade` never leaves the file, and a
  /// `broadcast::Sender` is not one -- it reaches no store, blocks nothing, and
  /// is held here for the same reason `tx` is: **both the store thread and the
  /// watcher must publish, and a second home for the sender would be two feeds
  /// one subscriber could only be on one of.**
  events: broadcast::Sender<Event>,
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
  pub async fn open(
    root: PathBuf,
    events: broadcast::Sender<Event>,
  ) -> Result<ProjectHandle, Response> {
    let (tx, mut rx) = mpsc::channel::<Work>(QUEUE_DEPTH);
    let (ready_tx, ready_rx) = oneshot::channel::<Result<String, Response>>();
    let thread_root = root.clone();
    // **SHARED WITH THE STORE THREAD BECAUSE THAT IS WHERE THE WORK HAPPENS.**
    // The handle reports the count and the thread increments it, so the two
    // must be the same cell -- a counter on the handle alone could only be
    // incremented at the send, which is the meaning this one deliberately does
    // not have.
    let ingested = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let thread_ingested = std::sync::Arc::clone(&ingested);
    let thread_events = events.clone();
    // **CAPTURED HERE BECAUSE HERE IS INSIDE THE RUNTIME AND THE STORE THREAD
    // IS NOT.** `Handle::current()` panics off a runtime thread; this `async
    // fn` runs on one, so the handle is taken once and moved across. The store
    // thread uses it for exactly one thing: blocking on a GraphQL execution
    // (`Op::Graphql`) with THIS process's runtime rather than a second one.
    let runtime = tokio::runtime::Handle::current();

    std::thread::spawn(move || {
      // **THE ID IS LEARNED ON THIS THREAD AND KEPT HERE, RATHER THAN READ BACK
      // OFF THE HANDLE.** The handle does not exist yet -- it is built from what
      // this thread reports -- and reaching back for it per event would make the
      // publisher depend on the thing it is publishing to.
      //
      // **BOUND FROM THE MATCH RATHER THAN PRE-DECLARED EMPTY, AND THAT IS THE
      // POINT RATHER THAN TIDINESS.** It was `let mut thread_project_id =
      // String::new()` assigned in the `Ok` arm, and rustc reported the initial
      // value as never read -- correctly, since the other arm returns. **The
      // warning was cosmetic and the SHAPE was not**: an identity with an empty
      // default is the precise defect this daemon's events were found carrying,
      // where `""` stood in for *absent* and made two projects compare equal.
      // Binding it out of the match means the store thread cannot hold an empty
      // id even transiently, because there is no state in which it has one.
      let (mut facade, thread_project_id) = match open_facade(&thread_root) {
        Ok((facade, project_id)) => {
          if ready_tx.send(Ok(project_id.clone())).is_err() {
            // Nobody is waiting any more: the caller went away between the
            // spawn and the open. Drop the facade rather than serve a queue no
            // one will read.
            return;
          }
          (facade, project_id)
        }
        Err(refusal) => {
          let _ = ready_tx.send(Err(refusal));
          return;
        }
      };

      // **PER-PROJECT STATE FOR A PER-PROJECT REPORT, AND IT LIVES EXACTLY AS
      // LONG AS THE PROJECT DOES.** A project the daemon is not backing up is
      // reconsidered on every sweep and would otherwise say so on every sweep
      // -- 288 identical lines a day, which is how the one log line that
      // matters becomes unfindable.
      //
      // **ONE FLAG FOR BOTH REASONS, BECAUSE THEY CANNOT BOTH BE TRUE.**
      // `Disabled` is decided before `schedule` is read, so a project reaches
      // at most one of these arms and the flag means *I have already said why
      // this project is not being backed up*. That is also why its NAME had to
      // change: `said_it_cannot_be_scheduled` was accurate for the only case
      // that existed and would have been quietly wrong for the second. Saying it once per store thread is the
      // whole of what an operator needs, and the lifetime is right by
      // construction: a project reopened is a project reported about again.
      let mut said_why_it_is_not_backing_up = false;

      // **A PROJECT THAT OPENS CONSIDERS A BACKUP, WHICH IS WHAT MAKES A
      // RESTART HARMLESS** (`AC-08.8`). The sweep alone would leave a daemon
      // restarted at login up to one sweep behind on every project it holds,
      // and a machine rebooted more often than the sweep interval would never
      // reach one -- the same permanently-one-boot-away failure that
      // `backup::due` reading the STORE rather than a timer exists to remove,
      // arriving one layer out.
      //
      // **AFTER THE READY SIGNAL, DELIBERATELY**: the caller is already
      // unblocked, so a snapshot cannot delay the OPEN. It does sit in front of
      // the first queued op, which is a real cost and a small one -- a
      // `VACUUM INTO` on a project store, once per project per period, against
      // the alternative of a daemon that holds a project and has not backed it
      // up.
      consider_backup(
        &mut facade,
        &thread_root,
        &mut said_why_it_is_not_backing_up,
      );

      // `blocking_recv` is correct HERE and would be a defect anywhere else in
      // this crate: this is not a runtime worker, it is a thread whose entire
      // job is to block.
      while let Some(work) = rx.blocking_recv() {
        match work {
          Work::Client { op, reply } => {
            let response = serve(&mut facade, op, &runtime);
            // A dropped receiver means the client disconnected mid-request.
            // The work is already done and there is nobody to tell, which is a
            // state rather than a fault.
            let _ = reply.send(response);
          }
          Work::Ingest => {
            ingest(&mut facade, &thread_root);
            // **`projectChanged` IS EMITTED HERE AND NOWHERE ELSE, BECAUSE HERE
            // IS THE ONLY PLACE THAT KNOWS THE RE-READ FINISHED.** The watcher
            // knows a file moved and emits `fileChanged` for it; only the store
            // thread knows the MODEL moved. Emitting this from the watcher
            // would tell subscribers the project changed before it had, and a
            // subscriber redrawing on that reads the state it already had.
            //
            // **A SEND WITH NO SUBSCRIBERS IS AN `Err` AND IS NOT A FAILURE.**
            // `broadcast::Sender::send` refuses when nobody is listening, which
            // is the ordinary case for a daemon nobody has subscribed to.
            let _ = thread_events.send(Event::ProjectChanged {
              project_id: thread_project_id.clone(),
            });
            // **AFTER THE WORK, AND AFTER IT EVEN WHEN IT FAILED.** The
            // question this answers is *how many times did the watcher make
            // this store re-read the tree*, which is what `AC-08.5`'s
            // debounce and scope claims are about -- and a failed ingest was
            // still a trigger that fired. Counting only successes would make
            // a self-triggering loop over unreadable files invisible.
            thread_ingested.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
          }
          Work::Backup => {
            consider_backup(
              &mut facade,
              &thread_root,
              &mut said_why_it_is_not_backing_up,
            );
          }
        }
      }
    });

    match ready_rx.await {
      Ok(Ok(project_id)) => Ok(ProjectHandle {
        tx,
        root,
        dispatched: std::sync::atomic::AtomicU64::new(0),
        ingested,
        project_id,
        events,
      }),
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

  /// How many ops have been dispatched to this store.
  pub fn dispatched(&self) -> u64 {
    self.dispatched.load(std::sync::atomic::Ordering::Relaxed)
  }

  /// How many times this project's tree has been re-read after a change.
  pub fn ingested(&self) -> u64 {
    self.ingested.load(std::sync::atomic::Ordering::Relaxed)
  }

  /// This project's identity, as `D20`'s events name it.
  pub fn project_id(&self) -> &str {
    &self.project_id
  }

  /// A live feed of this project's changes (`AC-08.6`).
  ///
  /// **A `broadcast` RECEIVER, SO EVERY SUBSCRIBER SEES EVERY EVENT.** An mpsc
  /// fan-out would give each event to exactly one of them, which is the
  /// opposite of what a subscription is -- and the failure would be invisible
  /// with one subscriber and silently wrong with two.
  pub fn subscribe(&self) -> broadcast::Receiver<Event> {
    self.events.subscribe()
  }

  /// Publish one event to this project's subscribers.
  ///
  /// **THE WATCHER PUBLISHES THROUGH THE HANDLE RATHER THAN HOLDING ITS OWN
  /// SENDER**, so there is one feed per project and not one per publisher. A
  /// watcher with its own channel would deliver `fileChanged` to subscribers of
  /// a different channel from the one carrying `projectChanged`, and each half
  /// would look correct in isolation.
  ///
  /// The `Err` case is *nobody is subscribed*, which is the ordinary state of a
  /// daemon nobody has asked for a feed, so callers discard it deliberately
  /// rather than by omission.
  pub fn publish(&self, event: Event) -> Result<usize, broadcast::error::SendError<Event>> {
    self.events.send(event)
  }

  /// Ask the store, without occupying a worker while it answers.
  pub async fn call(&self, op: Op) -> Response {
    // Counted BEFORE the send, so a request the store never answers still
    // registers as having been dispatched: the harness's question is whether
    // the CLIENT routed, and a store that then failed does not un-route it.
    self
      .dispatched
      .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let (reply_tx, reply_rx) = oneshot::channel();
    if self
      .tx
      .send(Work::Client {
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

  /// Ask the store to re-read the project from disk (`AC-08.5`).
  ///
  /// **SYNCHRONOUS, BECAUSE ITS ONLY CALLER IS NOT ON THE RUNTIME.** The
  /// debouncer owns its own thread, so `blocking_send` is correct here for the
  /// same reason `blocking_recv` is correct on the store thread and would be a
  /// defect anywhere else in this crate: neither is a runtime worker.
  ///
  /// **IT DOES NOT TOUCH [`ProjectHandle::dispatched`], AND THAT IS THE POINT
  /// OF ITS BEING A SEPARATE DOOR.** See [`Work`].
  ///
  /// **BLOCKING RATHER THAN DROPPING WHEN THE QUEUE IS FULL.** A duplicate
  /// ingest costs a scan and writes nothing, because the sync is driven by
  /// content hashes; a DROPPED one leaves the store behind the disk until
  /// somebody happens to edit again. The wait delays the next batch for this
  /// project and nothing else.
  /// **THE REFUSAL IS A `Response`, NOT A `String`** (`IN-RS-CODE-004`, caught
  /// by the pre-commit critic). It is the same shape every other fallible entry
  /// in this crate returns, so the message and the remedy stay two fields
  /// rather than one string a caller has to take apart to report.
  pub fn ingest(&self) -> Result<(), Response> {
    self.tx.blocking_send(Work::Ingest).map_err(|_| {
      Response::error(
        format!(
          "the store for `{}` is no longer running, so external edits are not being ingested",
          self.root.display()
        ),
        "the project's store thread has ended, which it does only on a panic. Restart the daemon.",
      )
    })
  }

  /// Ask this project whether a scheduled backup is due, and take one if it is.
  ///
  /// **`async` WHERE [`ProjectHandle::ingest`] IS BLOCKING, AND THE SPLIT IS
  /// ABOUT THE CALLER RATHER THAN THE WORK.** The watcher owns its own thread,
  /// so blocking there costs nothing this runtime cares about; the backup sweep
  /// is a tokio task, and a `blocking_send` from one would park a runtime worker
  /// on a full queue -- the exact defect this module's whole arrangement exists
  /// to avoid, arrived at through the mechanism added to be careful.
  ///
  /// **IT DOES NOT WAIT FOR THE OUTCOME AND HAS NOTHING TO RETURN.** There is
  /// nobody to answer: the sweep is a timer. The store thread reports what
  /// happened where a daemon reports things, and what an OPERATOR meets is
  /// `intent doctor`, which reads the same rows the backup wrote.
  ///
  /// **A CLOSED CHANNEL IS SILENT HERE, DELIBERATELY, AND IT IS NOT A SWALLOWED
  /// ERROR.** A store thread ends only on a panic, which has already printed;
  /// the sweep re-sends every five minutes, so a second message per project per
  /// sweep would turn one panic into a permanent stream. The state is reported
  /// where it is actionable -- the project stops being backed up, and `doctor`
  /// says the store is stale.
  pub async fn consider_backup(&self) {
    let _ = self.tx.send(Work::Backup).await;
  }
}

/// Re-read the project from disk, on the store thread.
///
/// **THE SAME CALL `intent sync --to-store` MAKES, WHICH IS D32 APPLIED TO A
/// TRIGGER NOBODY TYPED.** A daemon with its own ingest path would be a second
/// sync engine in the literal sense the design warns about -- not two processes
/// racing, but two implementations that agree today.
///
/// **A FAILURE IS PRINTED BECAUSE THERE IS NOBODY TO RETURN IT TO, AND SILENCE
/// IS THE ONE THING IT MUST NOT BE** (`IN-AG-NO-SILENT-001`). An ingest that
/// failed quietly leaves a daemon that looks healthy and a store that is
/// behind the disk, and the operator's first symptom is a stale answer to an
/// unrelated question. `AC-08.4` names where daemon logs live; until that
/// lands this is stderr, which is where `intent daemon run` puts it in front
/// of whoever started it.
fn ingest(facade: &mut Facade, root: &Path) {
  if let Err(e) = facade.sync_from_disk(&intentsvcs::sync::Scope::All) {
    eprintln!(
      "intentd: ingesting `{}` after an external edit failed: {}\n  remedy: {}",
      root.display(),
      e.render(),
      e.remedy()
    );
  }
}

/// Take a scheduled backup if this project is due one, on the store thread.
///
/// **THE SAME CALL `intent backup` MAKES, WHICH IS D32 APPLIED TO A SECOND
/// TRIGGER NOBODY TYPED** -- the argument [`ingest`] above makes for sync,
/// made for backups, which is `AC-08.8`. The retention policy, the period and
/// the never-taken rule all live in `intentsvcs::backup`; this file holds none
/// of them and must not, or the scheduled path and the typed path become two
/// implementations that agree today.
///
/// **THE OUTCOME IS PRINTED BECAUSE THERE IS NOBODY TO RETURN IT TO**
/// (`IN-AG-NO-SILENT-001`). A success is one line a day on a daily schedule and
/// it is worth having: the alternative is a daemon that says nothing whether it
/// is working or not.
///
/// **AND A FAILURE IS ALREADY RECORDED IN THE STORE BEFORE THIS PRINTS IT.**
/// `backup::take` writes the failed attempt with its reason, so what an
/// operator meets is `intent doctor` reading those rows -- the log line is the
/// convenience and the row is the record. That ordering is the criterion's
/// *never only in a log nobody reads*, and it is the reason this function can
/// print without that being the whole of its reporting.
fn consider_backup(facade: &mut Facade, root: &Path, said_why_it_is_not_backing_up: &mut bool) {
  match intentsvcs::backup::due(facade.project(), facade.store()) {
    Ok(intentsvcs::backup::Due::Now) => {
      match intentsvcs::backup::cycle(facade.project(), facade.store()) {
        Ok(ran) => {
          let project = facade.project();
          println!(
            "intentd: backed up `{}` to {}{}",
            root.display(),
            project.relative(&ran.written),
            match ran.removed.len() {
              0 => String::new(),
              n => format!(" ({n} expired snapshot(s) removed)"),
            }
          );
        }
        Err(e) => eprintln!(
          "intentd: the scheduled backup of `{}` failed: {}\n  remedy: {}",
          root.display(),
          e,
          e.remedy()
        ),
      }
    }
    Ok(intentsvcs::backup::Due::NotYet) => {}
    // **ANNOUNCED, NOT SILENT, THOUGH IT IS NOT AN ERROR.** The operator asked
    // for this, so there is no remedy to offer and none is offered. What the
    // line buys is the answer to *why is my daemon not backing this up*
    // WITHOUT having to go and read a config file to find out -- and the
    // sentence deliberately says what remains true, because the setting stops
    // the sweep and stops nothing else.
    Ok(intentsvcs::backup::Due::Disabled) => {
      if !*said_why_it_is_not_backing_up {
        *said_why_it_is_not_backing_up = true;
        println!(
          "intentd: `{}` has backup.enabled = false, so no scheduled backup is being taken. `intent backup` still takes one when you ask, and `intent doctor` still reports a stale one.",
          root.display()
        );
      }
    }
    // **SAID ONCE, NOT ONCE A SWEEP.** See the flag's declaration on the store
    // thread; and `doctor` is what an operator actually meets, which reports
    // the same setting from the same config with the remedy attached.
    Ok(intentsvcs::backup::Due::Unschedulable(value)) => {
      if !*said_why_it_is_not_backing_up {
        *said_why_it_is_not_backing_up = true;
        eprintln!(
          "intentd: `{}` is NOT being backed up: backup.schedule is {value:?}, which is not one of hourly, daily, weekly\n  remedy: correct backup.schedule in the project's config.json. `intent doctor` reports this too, with the estate's other findings.",
          root.display()
        );
      }
    }
    // The store could not be asked whether a backup was due. Reported rather
    // than retried silently: the sweep comes round again, and a reader of this
    // log needs to know the decision was not made rather than made as `no`.
    Err(e) => eprintln!(
      "intentd: could not tell whether `{}` is due a backup: {}\n  remedy: {}",
      root.display(),
      e,
      e.remedy()
    ),
  }
}

/// Open the facade for a root, turning every failure into something sayable.
fn open_facade(root: &Path) -> Result<(Facade, String), Response> {
  let project = Project::open(root).map_err(|e| {
    Response::error(
      format!("{e}"),
      format!(
        "`{}` is registered with this daemon and is not an Intent project. Run `intent init` there, or the directory has been replaced since the daemon first opened it.",
        root.display()
      ),
    )
  })?;
  // **THE ID IS TAKEN HERE BECAUSE HERE IS WHERE THE CONFIG IS ALREADY OPEN**,
  // and `D20`'s events name a project by id rather than by root: a root is
  // where a project sits today and moves (`AC-08.1` exists because they do),
  // while the id is what it is.
  let project_id = project.config().project_id.clone().unwrap_or_default();
  let ctx = FacadeContext {
    // **THE DAEMON IS NOT THE AUTHOR AND MUST NOT CLAIM TO BE.** `principal`
    // reaches the event log, so a daemon writing "local" would attribute every
    // client's work to itself. Naming the daemon is honest today, when no
    // request carries an identity; when one does, it comes off the request.
    principal: "intentd".to_string(),
    project_id: project.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let facade =
    Facade::open(project, ctx).map_err(|e| Response::error(format!("{e}"), e.remedy()))?;
  Ok((facade, project_id))
}

/// Serve one project-scoped operation.
///
/// **THE ONLY FUNCTION IN THIS CRATE THAT TOUCHES A `Facade`, AND IT RUNS ON
/// THE STORE THREAD.** Adding an operation means adding an arm here; there is
/// nowhere else it could be added, which is the property the door is for.
///
/// `runtime` is the daemon's own tokio handle, and it is here for one arm:
/// the store thread is a plain `std::thread`, so a future produced by the
/// facade has to be driven by something, and the something must not be a
/// second executor.
fn serve(facade: &mut Facade, op: Op, runtime: &tokio::runtime::Handle) -> Response {
  match op {
    Op::ThreadList => Response::Threads {
      threads: facade
        .st_list()
        .into_iter()
        .map(|thread| ThreadSummary {
          id: thread.id.clone(),
          title: thread.title.clone(),
          status: thread.status,
          created: thread.created.clone(),
          completed: thread.completed.clone(),
        })
        .collect(),
    },
    // **THE STORE THREAD BLOCKS ON THIS, AND IT IS THE RIGHT THREAD TO DO IT
    // ON.** Executing a document is async because async-graphql's resolvers
    // are; the future awaits nothing outside itself -- the snapshot was taken
    // through the facade BEFORE the future existed -- so `block_on` costs this
    // thread exactly the resolver work and costs no worker anything. The
    // handle is tokio's own, taken in `open`: no second executor enters the
    // daemon, and none may enter the CLI, which is why the CLI ships the
    // document here at all (vc's ruling, 2026-08-31, recorded on the op).
    //
    // A refusal the SCHEMA makes -- a mutation against `EmptyMutation`, an
    // unknown field -- is inside `response`, on the spec's channel; the only
    // `Response::Error` this arm can produce is a serialisation fault, which
    // is intentd's and says so.
    Op::Graphql { query, variables } => match runtime.block_on(facade.graphql(&query, variables)) {
      Ok(response) => Response::Graphql { response },
      Err(e) => Response::error(
        format!("the GraphQL answer could not be serialised: {e}"),
        "this is a fault in intentd rather than in the document or the project; the daemon's log names it.",
      ),
    },
    // **UNREACHABLE BY DISPATCH AND ANSWERED ANYWAY.** The registry is not
    // scoped to a project, so the connection handler answers it before any
    // handle is involved. If that dispatch ever changes, a caller arriving here
    // gets a refusal that names the mistake rather than a panic or a wrong
    // answer -- the arm costs two lines and removes a whole class of silent
    // misrouting.
    // **UNREACHABLE FOR THE SAME REASON AND ANSWERED FOR THE SAME REASON.**
    // A subscription changes the CONNECTION's mode, so it is handled where
    // connections are, before any handle is involved. Reaching here would be a
    // routing fault, and it says so rather than answering something plausible.
    Op::Subscribe => Response::error(
      "a subscription changes the connection's mode and reached a project's store",
      "this is a routing fault inside intentd. `Op::Subscribe` is handled on the connection, not by any project's store thread.",
    ),
    // **UNREACHABLE FOR A THIRD TIME, AND THE ARM EARNS ITS LINES HERE MOST OF
    // ALL.** Stopping is a request about the DAEMON, so it is answered on the
    // connection before any project is opened. Reaching a store thread would
    // mean the daemon had been asked to stop and had instead dispatched the
    // question to one project -- which would consume the request, answer
    // something plausible, and leave the daemon running.
    Op::Shutdown => Response::error(
      "stopping the daemon is not a project-scoped operation and reached a project's store",
      "this is a routing fault inside intentd. `Op::Shutdown` is handled on the connection, not by any project's store thread, so the daemon is still running.",
    ),
    Op::Registry => Response::error(
      "the registry is not a project-scoped operation and reached a project's store",
      "this is a routing fault inside intentd. The registry is answered by the daemon itself, not by any project.",
    ),
  }
}
