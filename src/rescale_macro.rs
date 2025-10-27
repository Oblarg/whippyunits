/// Rescale a quantity to a different unit of the same dimension.
///
/// This macro provides a convenient way to rescale quantities in inline contexts where
/// there is no easy place to insert a target type assertion via `unit!`.  Defaults to
/// `f64` storage type; due to generic resolution limitations, any other numeric type must be
/// explicitly specified.
///
/// ## Syntax
///
/// ```rust,ignore
/// rescale!(quantity, target_unit)                // Returns Quantity<target_unit, f64>
/// rescale!(quantity, target_unit, storage_type)  // Returns Quantity<target_unit, storage_type>
/// ```
///
/// where:
/// - `quantity`: A [quantity](crate::quantity!) to rescale.
/// - `unit_expression`: A "unit literal expression"
///     - A "unit literal expression" is either:
///         - An atomic unit (may include prefix):
///             - `m`, `kg`, `s`, `A`, `K`, `mol`, `cd`, `rad`
///         - An exponentiation of an atomic unit:
///             - `m2`, `m^2`
///         - A multiplication of two or more exponentiated atomic units:
///             - `kg.m2`, `kg * m2`
///         - A division of two such product expressions:
///             - `kg.m2/s2`, `kg * m2 / s^2`
///             - There may be at most one division expression in a unit literal expression
///             - All terms trailing the division symbol are considered to be in the denominator
/// - `storage_type`: An optional storage type for the quantity. Defaults to `f64`.
///
/// ## Examples
///
/// ```rust
/// # #[culit::culit(whippyunits::default_declarators::literals)]
/// # fn main() {
/// use whippyunits::rescale;
///
/// // Default f64 storage type
/// let distance = rescale!(1.0m, mm); // ✅ 1000.0 Quantity<mm, f64>
/// // let _distance = rescale!(1.0m, ms); // ❌ Compile error (dimension mismatch)
/// let distance = rescale!(1m, mm, i32); // ✅ 1000 Quantity<mm, i32>
///
/// // Use in expressions where type assertion is awkward
/// let total = rescale!(1.0m, mm) + 500.0mm; // 1500.0 mm
/// # }
/// ```
#[macro_export]
macro_rules! rescale {
    // f64 (default) - two arguments
    ($quantity:expr, $unit:expr) => {{
        type TargetQuantity = $crate::unit!($unit, f64);
        let result: TargetQuantity = $crate::rescale($quantity);
        result as $crate::unit!($unit, f64)
    }};
    // f64 (explicit) - three arguments with f64
    ($quantity:expr, $unit:expr, f64) => {{
        type TargetQuantity = $crate::unit!($unit, f64);
        let result: TargetQuantity = $crate::rescale($quantity);
        result as $crate::unit!($unit, f64)
    }};
    // f32
    ($quantity:expr, $unit:expr, f32) => {{
        type TargetQuantity = $crate::unit!($unit, f32);
        let result: TargetQuantity = $crate::rescale_f32($quantity);
        result as $crate::unit!($unit, f32)
    }};
    // i8
    ($quantity:expr, $unit:expr, i8) => {{
        type TargetQuantity = $crate::unit!($unit, i8);
        let result: TargetQuantity = $crate::rescale_i8($quantity);
        result as $crate::unit!($unit, i8)
    }};
    // i16
    ($quantity:expr, $unit:expr, i16) => {{
        type TargetQuantity = $crate::unit!($unit, i16);
        let result: TargetQuantity = $crate::rescale_i16($quantity);
        result as $crate::unit!($unit, i16)
    }};
    // i32
    ($quantity:expr, $unit:expr, i32) => {{
        type TargetQuantity = $crate::unit!($unit, i32);
        let result: TargetQuantity = $crate::rescale_i32($quantity);
        result as $crate::unit!($unit, i32)
    }};
    // i64
    ($quantity:expr, $unit:expr, i64) => {{
        type TargetQuantity = $crate::unit!($unit, i64);
        let result: TargetQuantity = $crate::rescale_i64($quantity);
        result as $crate::unit!($unit, i64)
    }};
    // i128
    ($quantity:expr, $unit:expr, i128) => {{
        type TargetQuantity = $crate::unit!($unit, i128);
        let result: TargetQuantity = $crate::rescale_i128($quantity);
        result as $crate::unit!($unit, i128)
    }};
    // u8
    ($quantity:expr, $unit:expr, u8) => {{
        type TargetQuantity = $crate::unit!($unit, u8);
        let result: TargetQuantity = $crate::rescale_u8($quantity);
        result as $crate::unit!($unit, u8)
    }};
    // u16
    ($quantity:expr, $unit:expr, u16) => {{
        type TargetQuantity = $crate::unit!($unit, u16);
        let result: TargetQuantity = $crate::rescale_u16($quantity);
        result as $crate::unit!($unit, u16)
    }};
    // u32
    ($quantity:expr, $unit:expr, u32) => {{
        type TargetQuantity = $crate::unit!($unit, u32);
        let result: TargetQuantity = $crate::rescale_u32($quantity);
        result as $crate::unit!($unit, u32)
    }};
    // u64
    ($quantity:expr, $unit:expr, u64) => {{
        type TargetQuantity = $crate::unit!($unit, u64);
        let result: TargetQuantity = $crate::rescale_u64($quantity);
        result as $crate::unit!($unit, u64)
    }};
    // u128
    ($quantity:expr, $unit:expr, u128) => {{
        type TargetQuantity = $crate::unit!($unit, u128);
        let result: TargetQuantity = $crate::rescale_u128($quantity);
        result as $crate::unit!($unit, u128)
    }};
}
