use std::collections::BTreeSet;
use std::collections::LinkedList;

fn main() {
  let mut v = Vec::new();
  v.push(10);
  v.push(5);

  let mut s = BTreeSet::new();

  s.insert(300);
  s.insert(400);

  let it = v.iter().chain(s.iter());

  for n in it.clone().map( |n| n * 2 ) {
    println!("n: {}", n);
  }

  let total = it.clone().fold(0, |acc, x| acc + x);
  println!("total: {}", total);


  let list: LinkedList<_> = it.filter(|n| *n % 2 == 0).collect();
  for i in list.iter() {
    println!("list: {}", i);
  }

  for (n, m) in v.iter().zip(s.iter()) {
    println!("n: {}, m: {}", n, m);
  }
}