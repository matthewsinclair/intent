pub mod store;

pub struct Widget {
  pub size: usize,
}

impl Widget {
  pub fn new(size: usize) -> Self {
    Widget { size }
  }

  pub fn size(&self) -> usize {
    self.size
  }
}

pub fn assemble() -> usize {
  let widget = Widget::new(3);
  let opened = store::open("tiny");
  widget.size() + opened.len()
}
