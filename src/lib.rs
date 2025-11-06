#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(trait_alias)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[doc(hidden)]
pub trait IsI16<const S: i16> {}
impl<const S: i16> IsI16<S> for () {}

#[doc(hidden)]
pub struct Helper<const N: usize, ActualT>(ActualT);

#[doc(hidden)]
pub trait GetSecondGeneric {
    type Type;
}

impl<const N: usize, T> GetSecondGeneric for Helper<N, T> {
    type Type = T;
}

pub mod api;
#[doc(hidden)]
pub mod arithmetic;
#[doc(hidden)]
pub mod arithmetic_quantity_types;
pub mod default_declarators;
pub mod dimension_traits;
#[doc(hidden)]
pub mod print;
pub mod quantity;
#[doc(hidden)]
pub mod rescale_macro;
#[doc(hidden)]
pub mod scale_conversion;
pub mod serialization;
#[doc(hidden)]
pub mod value_macro;

/// Define a set of "local declarators" that shadow the default declarators,
/// and either:
///
///  - Apply a brand type to the quantity, or
///  - Auto-rescale to a given "base unit scale" for storage
///
/// ## Syntax
///
/// ```rust,ignore
/// // Branded copy of the default declarators
/// define_unit_declarators!(
///     $namespace:ident,
///     $brand:ident
/// );
///
/// // Rescaling declarators; auto-rescale to the given base unit scale for storage
/// define_unit_declarators!(
///     $namespace:ident,
///     $mass_scale:ident,
///     $length_scale:ident,
///     $time_scale:ident,
///     $current_scale:ident,
///     $temperature_scale:ident,
///     $amount_scale:ident,
///     $luminosity_scale:ident,
///     $angle_scale:ident
/// );
///
/// // Branded rescaling declarators
/// define_unit_declarators!(
///     $namespace:ident,
///     $brand:ident,
///     $mass_scale:ident,
///     $length_scale:ident,
///     $time_scale:ident,
///     $current_scale:ident,
///     $temperature_scale:ident,
///     $amount_scale:ident,
///     $luminosity_scale:ident,
///     $angle_scale:ident
/// );
/// ```
///
/// where:
///
/// - $namespace: The name for the declarator module
/// - $brand: The name of the brand type to apply to the quantity (omit for unbranded declarators)
/// - $mass_scale: The scale for mass units (full unit name, e.g. "Kilogram")
/// - $length_scale: The scale for length units (full unit name, e.g. "Kilometer")
/// - $time_scale: The scale for time units (full unit name, e.g. "Second")
/// - $current_scale: The scale for current units (full unit name, e.g. "Ampere")
/// - $temperature_scale: The scale for temperature units (full unit name, e.g. "Kelvin")
/// - $amount_scale: The scale for amount units (full unit name, e.g. "Mole")
/// - $luminosity_scale: The scale for luminosity units (full unit name, e.g. "Candela")
/// - $angle_scale: The scale for angle units (full unit name, e.g. "Radian")
///
/// ## Usage
///
/// ```rust
/// use whippyunits::define_unit_declarators;
///
/// define_unit_declarators!(local_scale, Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian);
///
/// // autoconverting literals are available in the inner "literals" module
/// #[culit::culit(local_scale::literals)]
/// fn example() {
///     // trait declarators and the quantity! macro are available in the module
///     use local_scale::*;
///     let distance = 1.0.meters(); // automatically stores as 1000.0 millimeters
///     let distance = quantity!(1.0, m); // so does this
///     let distance = 1.0m; // and so does this!
///
///     // compound/derived units are "lifted" to the provided scale preferences
///     let energy = 1.0J;
///     // Hovering on J will show a "conversion trace":
///     // J = kg^1 * m^2 * s^-2 (drop prefix: mJ → J)
///     // ↓ (length: m → mm, factor: 10^-3)
///     // ↓ (exponent: 2, total factor: 10^-6)
///     // = kg^1 * mm^2 * s^-2
///     // = µJ
///     assert_eq!(energy.unsafe_value, 1000.0 * 1000.0);
/// }
/// ```
#[doc(inline)]
pub use whippyunits_proc_macros::define_unit_declarators;


/// Creates a concrete Quantity type from a unit expression with local scale preferences.
///
/// This macro is similar to [`unit!`], but allows specifying scale preferences for each dimension,
/// causing units to be automatically rescaled to match the specified base unit scales. This is useful
/// when you want to ensure quantities are stored in specific units regardless of how they are constructed.
///
/// This macro is typically used internally by [`define_unit_declarators!`] to generate local declarators,
/// but can also be used directly when you need explicit control over scale conversion for a specific type.
///
/// ## Syntax
///
/// ```rust,ignore
/// local_unit!(unit_expr, mass_scale, length_scale, time_scale, current_scale, temperature_scale, amount_scale, luminosity_scale, angle_scale);
/// local_unit!(unit_expr, mass_scale, length_scale, time_scale, current_scale, temperature_scale, amount_scale, luminosity_scale, angle_scale, storage_type);
/// local_unit!(unit_expr, mass_scale, length_scale, time_scale, current_scale, temperature_scale, amount_scale, luminosity_scale, angle_scale, storage_type, brand_type);
/// ```
///
/// Where:
/// - `unit_expr`: A "unit literal expression" (same syntax as [`unit!`])
/// - `mass_scale`: The scale identifier for mass units (e.g., `Kilogram`, `Gram`)
/// - `length_scale`: The scale identifier for length units (e.g., `Meter`, `Millimeter`)
/// - `time_scale`: The scale identifier for time units (e.g., `Second`, `Hour`)
/// - `current_scale`: The scale identifier for current units (e.g., `Ampere`)
/// - `temperature_scale`: The scale identifier for temperature units (e.g., `Kelvin`)
/// - `amount_scale`: The scale identifier for amount units (e.g., `Mole`)
/// - `luminosity_scale`: The scale identifier for luminosity units (e.g., `Candela`)
/// - `angle_scale`: The scale identifier for angle units (e.g., `Radian`)
/// - `storage_type`: An optional storage type for the quantity. Defaults to `f64`.
/// - `brand_type`: An optional brand type for the quantity. Defaults to `()`.
///
/// ## Examples
///
/// ```rust
/// use whippyunits::local_unit;
///
/// // Create a type that stores all lengths in millimeters
/// type LengthMm = local_unit!(m, Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian);
///
/// // Compound units are automatically converted to match scale preferences
/// type EnergyMicroJoule = local_unit!(J, Kilogram, Millimeter, Second, Ampere, Kelvin, Mole, Candela, Radian);
/// // Hovering on the `J` term in the unit literal expression will show a "conversion trace":
/// // J = kg^1 * m^2 * s^-2 (drop prefix: mJ → J)
/// // ↓ (length: m → mm, factor: 10^-3)
/// // ↓ (exponent: 2, total factor: 10^-6)
/// // = kg^1 * mm^2 * s^-2
/// // = µJ
///
/// // With explicit storage type
/// type MassKg = local_unit!(kg, Kilogram, Meter, Second, Ampere, Kelvin, Mole, Candela, Radian, f32);
///
/// // With brand type
/// struct MyBrand;
/// type BrandedLength = local_unit!(m, Kilogram, Meter, Second, Ampere, Kelvin, Mole, Candela, Radian, f64, MyBrand);
/// ```
#[doc(inline)]
pub use whippyunits_proc_macros::local_unit_type as local_unit;
/// Creates a concrete Quantity type from a unit expression.
///
/// This is particularly useful for constraining the result of potentially-type-ambiguous operations,
/// such as multiplication of two quantities with different dimensions.  If you want to construct a
/// quantity with a known value, use the `quantity!` macro instead.
///
/// ## Syntax
///
/// ```rust,ignore
/// unit!(unit_expr);
/// unit!(unit_expr, storage_type);
/// ```
///
/// Where:
/// - `unit_expr`: A "unit literal expression"
///     - A "unit literal expression" is either:
///         - An atomic unit (may include prefix):
///             - `m`, `kg`, `s`, `A`, `K`, `mol`, `cd`, `rad`
///         - An exponentiation of an atomic unit:
///             - `m2`, `m^2`
///         - A multiplication of two or more (possibly exponentiated) atomic units:
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
/// # use whippyunits::api::rescale;
/// # use whippyunits::unit;
/// // Constrain a multiplication to compile error if the units are wrong:
/// let area = 5.0m * 5.0m; // ⚠️ Correct, but unchecked; will compile regardless of the units
/// let area = 5.0m * 5.0s; // ❌ BUG: compiles fine, but is not an area
/// let area: unit!(m^2) = 5.0m * 5.0m; // ✅ Correct, will compile only if the units are correct
/// // let area: unit!(m^2) = 5.0m * 5.0s; // 🚫 Compile error, as expected
///
/// // Specify the target dimension of a rescale operation:
/// let area: unit!(mm) = rescale(5.0m);
/// assert_eq!(area.unsafe_value, 5000.0);
/// # }
/// ```
#[doc(inline)]
pub use whippyunits_proc_macros::proc_unit as unit;


// from_json, from_string, from_json_strict, and from_string_strict macros are exported via #[macro_export] in serialization.rs
// value! macro is exported via #[macro_export] in value_macro.rs
// rescale! macro is exported via #[macro_export] in rescale_macro.rs
