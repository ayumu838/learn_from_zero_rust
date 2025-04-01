struct H2O {}
struct O2 {}
struct H2 {}

fn born(_h2_1: H2, _h2_2: H2, _o2: O2) -> (H2O, H2O) {
  (H2O {}, H2O {})
}

fn main() {
  let h2_1 = H2 {};
  let h2_2 = H2 {};
  let o2 = O2 {};

  let (_h2o_1, _h2o_2) = born(h2_1, h2_2, o2);

  // 以下のはエラー
  // let (_h2o_3, _h2o_3) = born(h2_1, h2_2, o2);
}