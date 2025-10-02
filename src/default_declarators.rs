use crate::quantity_type::Quantity;
use whippyunits_proc_macros::generate_default_declarators;

macro_rules! define_quantity {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr, $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident,
        $(($scale_name:ident, $fn_name:ident, $scale_p2:expr, $scale_p3:expr, $scale_p5:expr, $scale_pi:expr)),* $(,)?
    ) => {
        // Generate the trait definition (generic over storage type)
        pub trait $trait_name<T = f64> {
            $(
                fn $fn_name(self) -> $scale_name<T>;
            )*
        }

        // Generate the type definitions (generic with f64 default)
        $(
            pub type $scale_name<T = f64> = Quantity<
                $mass_exp, $length_exp, $time_exp, $current_exp, $temperature_exp, $amount_exp, $luminosity_exp, $angle_exp,
                $scale_p2, $scale_p3, $scale_p5, $scale_pi,
                T,
            >;
        )*

        // Generate default extension trait implementation (uses default f64)
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self)
                }
            )*
        }

        // Generate extension trait implementations for i32
        impl $trait_name<i32> for i32 {
            $(
                fn $fn_name(self) -> $scale_name<i32> {
                    Quantity::new(self)
                }
            )*
        }
    };
}

macro_rules! define_affine_quantity {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr, $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident,
        $storage_scale:ident,
        $(($scale_name:ident, $fn_name:ident, $offset:expr)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }

        // Generate the type definitions (all stored in the same scale)
        $(
            pub type $scale_name = $storage_scale;
        )*

        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    $storage_scale::new(self + $offset)
                }
            )*
        }

        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    $storage_scale::new((self as f64) + $offset)
                }
            )*
        }
    };
}

// Generate all default declarators using the source of truth from default-dimensions
generate_default_declarators!();

#[macro_export]
macro_rules! quantity {
    ($value:expr, $unit:expr) => {
        <$crate::unit!($unit)>::new($value)
    };
    ($value:expr, $unit:expr, $storage_type:ty) => {
        <$crate::unit!($unit, $storage_type)>::new($value)
    };
}
