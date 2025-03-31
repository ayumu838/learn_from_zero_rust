fn main() {
  m();
}

fn m() {
  let a = 10;
  let b = 20;

  {
    let c = 30;
    let d = 40;
    n()
  }
}

fn n() {
  let e = 50;
  let f = 60;
}
