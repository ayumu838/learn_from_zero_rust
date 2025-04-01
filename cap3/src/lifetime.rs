fn main() {
  // 無効の場合 エラーになる
  // let a;

  // {
  //   let b = 10;
  //   a = &b;
  // }

  // println!("{a}");

  // 有効の場合
  let a;

  {
    let b = 10;
    a = &b;
    println!("{a}");
  }


  // ライフライム指定子
  let a: i32 = 10;
  let b: &i32 = &a;

  fn square<'a>(x: &'a i32) -> i32 {
    x * x
  }

  square(&b);

  struct Foo<'a> {
    x: &'a i32,
  }

  Foo { x: &a };
}




