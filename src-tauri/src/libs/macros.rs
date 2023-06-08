// fn approx(a: Self, b: Self, eps: Self) -> bool {
//   Self::abs(a - b) < eps
// }

// macro_rules! is_approx {
//   ($a: expr, $b: expr, $eps: expr) => {{
//     let diff = ($a - $b).abs();
//     if (diff <= $eps) {
//       return true;
//     }
//     return false;
//   }};
// }

// mod core_maccore_ros {
//   macro_rules! add {
//     ($a: expr, $b: expr) => {
//       $a + $b
//     };
//   }
// }

// #[derive(Debug, Default)]
// pub struct Test {
//   name: String,
// }

#[macro_export]
macro_rules! add {
  ($a: expr, $b: expr) => {
    $a + $b
  };
}

pub(crate)  use add;

// macro_rules! assert_f32_near {
//     // Explicit steps.
//     ($a:expr, $b:expr, $n:expr) => ({
//         let (a, b, n) = ($a, $b, $n);
//         let (r, previous, next) = afe_is_f32_near!(a, b, n);
//         assert!(r, afe_near_error_msg!(), a, b, n, previous, next)
//     });
//     // No explicit steps, use default.
//     ($a:expr, $b:expr) => (assert_f32_near!($a, $b, 4));
// }
