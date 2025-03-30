struct XOR64 {
  x: u64
}

impl XOR64 {
  fn new(seed: u64) -> XOR64 {
    XOR64 { x: seed ^ 88172645463325252 }
  }

  fn next(&mut self) -> u64 {
    let x = self.x;
    let x = x ^ (x << 13);
    let x = x ^ (x >> 7);
    let x = x ^ ( x << 17);
    self.x = x;
    return x
  }
}

const NUM: usize = 200_000_000;
fn randomized_vec() -> (Vec<u64>, Vec<u64>) {
  let mut v1 = Vec::new();
  let mut v2 = Vec::new();

  let mut generator = XOR64::new(1234);

  for _ in 0..NUM {
    let x1 = generator.next();
    let x2 = generator.next();
    v1.push(x1);
    v2.push(x2);
  }

  (v1, v2)
}

fn single_thread() {
  let (mut v1, mut v2) = randomized_vec();

  let start = std::time::Instant::now();

  v1.sort();
  v2.sort();

  let end = start.elapsed();

  println!(
    "single thread: {}.{:03} 秒",
    end.as_secs(),
    end.subsec_nanos()
  );
}

fn multi_thread() {
  let (mut v1, mut v2) = randomized_vec();

  let start = std::time::Instant::now();

  let handler1 = std::thread::spawn(move || {
    v1.sort();
  });

  let handler2 = std::thread::spawn(move || {
    v2.sort();
  });

  let _v1 = handler1.join().unwrap();
  let _v2 = handler2.join().unwrap();
  let end = start.elapsed();

  println!(
    "multi thread: {}.{:03} 秒",
    end.as_secs(),
    end.subsec_nanos()
  );
}

fn main() {
  single_thread();
  multi_thread();
}