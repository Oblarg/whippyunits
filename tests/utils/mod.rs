/// Macro for approximate equality assertions on floating-point numbers
/// 
/// Usage:
/// ```rust
/// assert_approx_eq!(actual, expected);                    // f64, default epsilon (1e-5)
/// assert_approx_eq!(actual, expected, epsilon);          // f64, custom epsilon
/// assert_approx_eq!(actual, expected, type = f32);       // f32, default epsilon
/// assert_approx_eq!(actual, expected, epsilon, type = f32); // f32, custom epsilon
/// ```
#[macro_export]
macro_rules! assert_approx_eq {
    // Base case: just left and right, default to f64 with default epsilon
    ($left:expr, $right:expr) => {
        assert_approx_eq!($left, $right, 1e-5);
    };
    
    // Case: left, right, and a numeric epsilon (f64)
    ($left:expr, $right:expr, $epsilon:expr) => {
        {
            let left_val = $left as f64;
            let right_val = $right as f64;
            let eps = $epsilon as f64;
            let diff = (left_val - right_val).abs();
            
            if diff > eps {
                panic!(
                    "assertion failed: `(left ≈ right)`\n  left:  {:?}\n right: {:?}\n  diff:  {:?} > epsilon: {:?}",
                    left_val, right_val, diff, eps
                );
            }
        }
    };
    
    // Case: left, right, type = f32 (default epsilon)
    ($left:expr, $right:expr, type = f32) => {
        assert_approx_eq!($left, $right, 1e-5f32, type = f32);
    };
    
    // Case: left, right, epsilon, type = f32
    ($left:expr, $right:expr, $epsilon:expr, type = f32) => {
        {
            let left_val = $left as f32;
            let right_val = $right as f32;
            let eps = $epsilon as f32;
            let diff = (left_val - right_val).abs();
            
            if diff > eps {
                panic!(
                    "assertion failed: `(left ≈ right)`\n  left:  {:?}\n right: {:?}\n  diff:  {:?} > epsilon: {:?}",
                    left_val, right_val, diff, eps
                );
            }
        }
    };
    
    // Case: left, right, type = f64 (explicit, default epsilon)
    ($left:expr, $right:expr, type = f64) => {
        assert_approx_eq!($left, $right, 1e-5);
    };
    
    // Case: left, right, epsilon, type = f64 (explicit)
    ($left:expr, $right:expr, $epsilon:expr, type = f64) => {
        assert_approx_eq!($left, $right, $epsilon);
    };
}

