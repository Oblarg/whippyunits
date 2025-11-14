#![feature(trait_alias)]
#![allow(mixed_script_confusables)]

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod compute_unit_dimensions_macro;
mod define_generic_dimension_macro;
mod define_literals_macro;
mod define_local_quantity_macro;
mod define_unit_declarators_macro;
mod generate_all_radian_erasures_macro;
mod generate_default_declarators_macro;
mod generate_literals_module_macro;
mod generate_local_unit_literals_macro;
mod local_unit_type_macro;
mod pow_lookup_macro;
mod quantity_macro;
mod unit_macro;
mod value_macro;

mod utils {
    pub mod culit;
    pub mod dimension_suggestions;
    pub mod lift_trace;
    pub mod literal_macros;
    pub mod scale_suggestions;
    pub mod shared_utils;
    pub mod unit_suggestions;
}

#[proc_macro]
#[doc(hidden)]
pub fn compute_unit_dimensions(input: TokenStream) -> TokenStream {
    compute_unit_dimensions_macro::compute_unit_dimensions(input)
}

#[proc_macro]
pub fn define_generic_dimension(input: TokenStream) -> TokenStream {
    let input =
        parse_macro_input!(input as define_generic_dimension_macro::DefineGenericDimensionInput);
    input.expand().into()
}

/// Creates a concrete [Quantity] type from a unit expression.
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
#[proc_macro]
pub fn proc_unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as unit_macro::UnitMacroInput);
    input.expand().into()
}

/// Creates a [Quantity] from a value and unit expression.
///
/// This macro supports both storage and nonstorage units. For nonstorage units,
/// it automatically dispatches to the appropriate declarator trait.
///
/// ## Syntax
///
/// ```rust,ignore
/// quantity!(value, unit_expr)
/// quantity!(value, unit_expr, storage_type)
/// quantity!(value, unit_expr, storage_type, brand_type)
/// ```
///
/// where:
/// - `value`: The value of the quantity
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
/// - `brand_type`: An optional brand type for the quantity. Defaults to `()`.
///
/// ## Examples
///
/// ```rust
/// # fn main() {
/// # use whippyunits::quantity;
/// // Basic quantities
/// let distance = quantity!(5.0, m);
/// let mass = quantity!(2.5, kg);
/// let time = quantity!(10.0, s);
///
/// // Compound units
/// let velocity = quantity!(10.0, m/s);
/// let acceleration = quantity!(9.81, m/s^2);
/// let force = quantity!(100.0, kg*m/s^2);
/// let energy = quantity!(50.0, kg.m2/s2);
///
/// // With explicit storage type
/// let distance_f32 = quantity!(5.0, m, f32);
/// let mass_i32 = quantity!(2, kg, i32);
///
/// // Complex expressions
/// let power = quantity!(1000.0, kg.m^2/s^3);
/// let pressure = quantity!(101325.0, kg/m.s^2);
///
/// // Nonstorage units (e.g., imperial units)
/// let length = quantity!(12.0, in); // inches
/// let mass = quantity!(1.0, lb); // pounds
/// # }
/// ```
///
/// ## Best Practice: Use Compound Unit Literal Expressions
///
/// For compound units, prefer using a compound unit literal expression in the macro
/// rather than performing arithmetic in source code:
///
/// ```rust
/// # fn main() {
/// # use whippyunits::quantity;
/// // ✅ Preferred: compound unit literal expression
/// let velocity = quantity!(10.0, m / s);
///
/// // ❌ Avoid: arithmetic in source code
/// // let velocity = quantity!(10.0, m) / quantity!(1.0, s);
/// # }
/// ```
///
/// Using compound unit literal expressions provides:
/// - **Better rust-analyzer interaction**: The proc macro always knows the result type,
///   enabling better IDE support and type inference
/// - **More reliable constant folding**: The math is frontloaded at compile time,
///   with no reliance on optimization to realize that values can be interned
#[proc_macro]
pub fn proc_quantity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as quantity_macro::QuantityMacroInput);
    input.expand().into()
}

/// Access the underlying numeric value of a [Quantity](crate::Quantity).
///
/// Because value! explicitly specifies the target unit, this is considered a
/// "unit-safe" operation - the type system will guarantee that the access is
/// dimensionally coherent and the value is correctly scaled.
///
/// Quantities with any `Brand` other than the default `()` must specify their brand
/// explicitly in the `value!` macro arguments; failure to do so will result in a
/// compile error.
///
/// Examples:
/// ```rust
/// # fn main() {
/// # use whippyunits::default_declarators::*;
/// # use whippyunits::value;
/// # use whippyunits::quantity;
///
/// let distance_f64 = quantity!(1.0, m);
/// let val_f64: f64 = value!(distance_f64, m);   // 1.0
/// let val_f64: f64 = value!(distance_f64, mm);  // 1000.0
/// // let _value: f64 = value!(distance_f64, s);  // ❌ compile error (incompatible dimension)
///
/// let distance_i32 = quantity!(1, m, i32);
/// let val_i32: i32 = value!(distance_i32, m, i32);   // 1
/// let val_i32: i32 = value!(distance_i32, mm, i32);  // 1000
/// // let _value: i32 = value!(distance_i32, s, i32);  // ❌ compile error (incompatible dimension)
/// # }
/// ```
#[proc_macro]
pub fn proc_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as value_macro::ValueMacroInput);
    input.expand().into()
}

#[proc_macro]
#[doc(hidden)]
pub fn local_unit_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as local_unit_type_macro::LocalQuantityMacroInput);
    input.expand().into()
}

#[proc_macro]
pub fn define_literals(input: TokenStream) -> TokenStream {
    define_literals_macro::define_literals(input)
}

/// Generate exponentiation lookup tables with parametric range
/// Usage: pow_lookup!(base: 2, range: -20..=20, type: rational)
#[proc_macro]
#[doc(hidden)]
pub fn pow_lookup(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as pow_lookup_macro::PowLookupInput);
    input.expand().into()
}

/// Generate π exponentiation lookup tables with rational approximation
/// Usage: pow_pi_lookup!(range: -10..=10, type: rational)
#[proc_macro]
#[doc(hidden)]
pub fn pow_pi_lookup(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as pow_lookup_macro::PiPowLookupInput);
    input.expand().into()
}

/// Generate all radian erasure implementations (both to scalar and to dimensionless quantities)
/// Usage: generate_all_radian_erasures!()
#[proc_macro]
#[doc(hidden)]
pub fn generate_all_radian_erasures(input: TokenStream) -> TokenStream {
    let input =
        parse_macro_input!(input as generate_all_radian_erasures_macro::AllRadianErasuresInput);
    input.expand().into()
}

/// Generate default declarators using the source of truth from whippyunits-core
/// Usage: generate_default_declarators!()
#[proc_macro]
#[doc(hidden)]
pub fn generate_default_declarators(input: TokenStream) -> TokenStream {
    let input =
        parse_macro_input!(input as generate_default_declarators_macro::DefaultDeclaratorsInput);
    input.expand().into()
}

/// Generate literals module for culit integration
/// Usage: generate_literals_module!()
#[proc_macro]
#[doc(hidden)]
pub fn generate_literals_module(input: TokenStream) -> TokenStream {
    generate_literals_module_macro::generate_literals_module(input)
}

/// Generate local unit literals namespace with lift trace documentation
#[proc_macro]
#[doc(hidden)]
pub fn generate_local_unit_literals(input: TokenStream) -> TokenStream {
    let input =
        parse_macro_input!(input as generate_local_unit_literals_macro::LocalUnitLiteralsInput);
    input.expand().into()
}

/// Define a local quantity trait and implementations for a given scale and set of units.
///
/// This is an internal macro used by define_unit_declarators! to generate the trait definitions.
/// Based on the original scoped_preferences.rs implementation.
#[proc_macro]
#[doc(hidden)]
pub fn define_local_quantity(input: TokenStream) -> TokenStream {
    define_local_quantity_macro::define_local_quantity(input)
}

/// Define a set of declarators that auto-convert to a given set of base units.
///
/// See [`define_unit_declarators`] for full documentation.
#[proc_macro]
pub fn define_unit_declarators(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as define_unit_declarators_macro::DefineBaseUnitsInput);
    input.expand().into()
}

/// Convert an arithmetic expression to associated type syntax (with ::Output).
///
/// Examples:
/// - `output!(CO / PV)` → `<CO as Div<PV>>::Output`
/// - `output!(CO / PV * PV)` → `<<CO as Div<PV>>::Output as Mul<PV>>::Output`
/// - `output!((CO * T) / PV)` → `<<CO as Mul<T>>::Output as Div<PV>>::Output`
/// - `output!(1 / T)` → `<<whippyunits::quantity::Quantity<whippyunits::quantity::Scale<whippyunits::quantity::_2<0>, whippyunits::quantity::_3<0>, whippyunits::quantity::_5<0>, whippyunits::quantity::_Pi<0>>, whippyunits::quantity::Dimension<whippyunits::quantity::_M<0>, whippyunits::quantity::_L<0>, whippyunits::quantity::_T<0>, whippyunits::quantity::_I<0>, whippyunits::quantity::_Θ<0>, whippyunits::quantity::_N<0>, whippyunits::quantity::_J<0>, whippyunits::quantity::_A<0>>, f64> as Div<T>>::Output`
#[proc_macro]
pub fn output(input: TokenStream) -> TokenStream {
    use quote::quote;
    use syn::{parse_macro_input, Expr, Lit};

    /// Recursively convert an expression to associated type syntax (with ::Output)
    fn expr_to_result_type(expr: &Expr) -> proc_macro2::TokenStream {
        match expr {
            Expr::Binary(bin) => {
                let left = expr_to_result_type(&bin.left);
                let right = expr_to_result_type(&bin.right);

                match bin.op {
                    syn::BinOp::Mul(_) => {
                        quote! {
                            <#left as Mul<#right>>::Output
                        }
                    }
                    syn::BinOp::Div(_) => {
                        quote! {
                            <#left as Div<#right>>::Output
                        }
                    }
                    syn::BinOp::Add(_) => {
                        quote! {
                            <#left as Add<#right>>::Output
                        }
                    }
                    syn::BinOp::Sub(_) => {
                        quote! {
                            <#left as Sub<#right>>::Output
                        }
                    }
                    _ => {
                        quote! { #left }
                    }
                }
            }
            Expr::Paren(paren) => expr_to_result_type(&paren.expr),
            Expr::Path(path) => {
                quote! { #path }
            }
            Expr::Group(group) => expr_to_result_type(&group.expr),
            Expr::Lit(lit) => {
                // Handle literal `1` as dimensionless quantity type
                match &lit.lit {
                    Lit::Int(int_lit) if int_lit.base10_digits() == "1" => {
                        quote! {
                            whippyunits::quantity::Quantity<
                                whippyunits::quantity::Scale<
                                    whippyunits::quantity::_2<0>,
                                    whippyunits::quantity::_3<0>,
                                    whippyunits::quantity::_5<0>,
                                    whippyunits::quantity::_Pi<0>
                                >,
                                whippyunits::quantity::Dimension<
                                    whippyunits::quantity::_M<0>,
                                    whippyunits::quantity::_L<0>,
                                    whippyunits::quantity::_T<0>,
                                    whippyunits::quantity::_I<0>,
                                    whippyunits::quantity::_Θ<0>,
                                    whippyunits::quantity::_N<0>,
                                    whippyunits::quantity::_J<0>,
                                    whippyunits::quantity::_A<0>
                                >,
                                f64
                            >
                        }
                    }
                    _ => {
                        quote! { #expr }
                    }
                }
            }
            _ => {
                quote! { #expr }
            }
        }
    }

    let input = parse_macro_input!(input as Expr);
    let result = expr_to_result_type(&input);
    result.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        // Test that the macro can parse valid input
        let input = "LengthOrMass, Length, Mass";
        let parsed =
            syn::parse_str::<define_generic_dimension_macro::DefineGenericDimensionInput>(input);
        assert!(parsed.is_ok());

        let parsed = parsed.unwrap();
        assert_eq!(parsed.trait_name.to_string(), "LengthOrMass");
        assert_eq!(parsed.dimension_exprs.len(), 2);
    }

    #[test]
    fn test_expand_macro() {
        // Test that the macro expands without panicking
        let input = syn::parse_str::<define_generic_dimension_macro::DefineGenericDimensionInput>(
            "LengthOrMass, Length, Mass",
        )
        .unwrap();

        let expanded = input.expand();
        // The expanded code should contain the trait name
        let expanded_str = expanded.to_string();
        assert!(expanded_str.contains("LengthOrMass"));
        assert!(expanded_str.contains("trait"));
    }
}
