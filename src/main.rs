#[derive(Debug)]
enum Stack {
  Empty,
  X(f64),
  XY(f64, f64, Vec<f64>),
}

#[derive(Debug)]
struct CalcState {
    stack: Stack
}

impl CalcState {
  fn exp(mut self) -> Self {
    self.stack = match self.stack {
        Stack::Empty => Stack::Empty,
        Stack::X(x) => Stack::X(x.exp()),
        Stack::XY(x, y, rest) => Stack::XY(x.exp(), y, rest)
    };

    self
  }
}

 fn main() {
    let foo  = CalcState { stack: Stack::X(1.295) };

    println!("{:#?}", foo.exp() );
 }
