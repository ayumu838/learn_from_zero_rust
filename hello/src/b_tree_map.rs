use std::collections::BTreeMap;

fn main() {
  let mut m = BTreeMap::new();

  m.insert(1, "apple");
  m.insert(2, "orange");
  m.insert(3, "banana");

  for (key, value) in m.iter() {
    println!("{}: {}", key, value);
  }

  if let Some(old) = m.remove(&2) {
    println!("removed {}", old);
  }

  if let Some(value) = m.get(&3) {
    println!("value {}", value);
  }

}