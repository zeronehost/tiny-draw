#[derive(Clone, Debug)]
pub struct Offset {
  top: i32,
  left: i32,
}

impl Offset {
  pub fn new(top: i32, left: i32) -> Self {
    Self { top, left }
  }
}
