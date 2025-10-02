// use crate::default_declarators;
// use crate::quantity_type::Quantity;

#[macro_export]
macro_rules! define_local_quantity {
    (
        $local_quantity_scale:ident,
        $trait_name:ident, $(($scale_name:ident, $fn_name:ident)),* $(,)?
    ) => {
        // Generate the trait definition (generic over storage type)
        pub trait $trait_name<T = f64> {
            $(
                fn $fn_name(self) -> $crate::default_declarators::$local_quantity_scale<T>;
            )*
        }

        // Generate extension trait implementations for f64 (default)
        impl $trait_name<f64> for f64 {
            $(
                fn $fn_name(self) -> $crate::default_declarators::$local_quantity_scale<f64> {
                    rescale_f64($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }

        // Generate extension trait implementations for i32
        impl $trait_name<i32> for i32 {
            $(
                fn $fn_name(self) -> $crate::default_declarators::$local_quantity_scale<i32> {
                    rescale_i32($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }

        // Generate extension trait implementations for i64
        impl $trait_name<i64> for i64 {
            $(
                fn $fn_name(self) -> $crate::default_declarators::$local_quantity_scale<i64> {
                    rescale_i64($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }
    };
}

// Generate the define_base_units macro from source of truth
whippyunits_proc_macros::generate_scoped_preferences!();