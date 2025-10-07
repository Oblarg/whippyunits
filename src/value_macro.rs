macro_rules! define_value_macro {
    ($macro_name:ident, $rescale_fn:ident, $T:ty) => {
        #[macro_export]
        macro_rules! $macro_name {
            ($quantity:expr, $unit:expr) => {{
                type TargetQuantity = $crate::unit!($unit, $T);
                
                let rescaled: TargetQuantity = $crate::api::$rescale_fn($quantity);
                rescaled.unsafe_value
            }};
        }
    };
}

// Generate value! macros for all supported backing types
define_value_macro!(value_f64, rescale, f64);
define_value_macro!(value_f32, rescale_f32, f32);
define_value_macro!(value_i8, rescale_i8, i8);
define_value_macro!(value_i16, rescale_i16, i16);
define_value_macro!(value_i32, rescale_i32, i32);
define_value_macro!(value_i64, rescale_i64, i64);
define_value_macro!(value_i128, rescale_i128, i128);
define_value_macro!(value_u8, rescale_u8, u8);
define_value_macro!(value_u16, rescale_u16, u16);
define_value_macro!(value_u32, rescale_u32, u32);
define_value_macro!(value_u64, rescale_u64, u64);
define_value_macro!(value_u128, rescale_u128, u128);

/// Unit-safe value extraction macro
/// 
/// Usage: `value!(quantity, unit_literal)` or `value!(quantity, unit_literal, type)`
/// 
/// This macro extracts the raw value from a quantity, rescaling it to the specified unit.
/// It's unit-safe because it uses the rescale function to ensure dimensional consistency.
/// The conversion is done statically at compile time using the type system.
/// 
/// The macro automatically infers the backing type from the input quantity and returns
/// the same type, maintaining zero-cost guarantees for integer types.
/// 
/// Examples:
/// ```rust
/// let distance_f64 = quantity!(1.0, m);
/// let val_f64: f64 = value!(distance_f64, m);   // 1.0
/// let val_f64: f64 = value!(distance_f64, mm);  // 1000.0
/// 
/// let distance_i32 = quantity!(1, m, i32);
/// let val_i32: i32 = value!(distance_i32, m, i32);   // 1
/// let val_i32: i32 = value!(distance_i32, mm, i32);  // 1000
/// ```
#[macro_export]
macro_rules! value {
    // f64 (default)
    ($quantity:expr, $unit:expr) => {
        $crate::value_f64!($quantity, $unit)
    };
    // f64 (explicit)
    ($quantity:expr, $unit:expr, f64) => {
        $crate::value_f64!($quantity, $unit)
    };
    // f32
    ($quantity:expr, $unit:expr, f32) => {
        $crate::value_f32!($quantity, $unit)
    };
    // i8
    ($quantity:expr, $unit:expr, i8) => {
        $crate::value_i8!($quantity, $unit)
    };
    // i16
    ($quantity:expr, $unit:expr, i16) => {
        $crate::value_i16!($quantity, $unit)
    };
    // i32
    ($quantity:expr, $unit:expr, i32) => {
        $crate::value_i32!($quantity, $unit)
    };
    // i64
    ($quantity:expr, $unit:expr, i64) => {
        $crate::value_i64!($quantity, $unit)
    };
    // i128
    ($quantity:expr, $unit:expr, i128) => {
        $crate::value_i128!($quantity, $unit)
    };
    // u8
    ($quantity:expr, $unit:expr, u8) => {
        $crate::value_u8!($quantity, $unit)
    };
    // u16
    ($quantity:expr, $unit:expr, u16) => {
        $crate::value_u16!($quantity, $unit)
    };
    // u32
    ($quantity:expr, $unit:expr, u32) => {
        $crate::value_u32!($quantity, $unit)
    };
    // u64
    ($quantity:expr, $unit:expr, u64) => {
        $crate::value_u64!($quantity, $unit)
    };
    // u128
    ($quantity:expr, $unit:expr, u128) => {
        $crate::value_u128!($quantity, $unit)
    };
}
