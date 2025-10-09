#[doc(hidden)]
macro_rules! define_imperial_quantity {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr, $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident,
        $(($fn_name:ident, $conversion_factor:expr, $storage_scale:ident)),* $(,)?
    ) => {
        // Generate the trait definition (generic over storage type)
        pub trait $trait_name<T = f64> {
            $(
                fn $fn_name(self) -> $crate::default_declarators::$storage_scale<T>;
            )*
        }

        // Generate extension trait implementations for f64 (default)
        impl $trait_name<f64> for f64 {
            $(
                fn $fn_name(self) -> $crate::default_declarators::$storage_scale<f64> {
                    $crate::default_declarators::$storage_scale::new(self * $conversion_factor)
                }
            )*
        }

        // Generate extension trait implementations for i32
        impl $trait_name<i32> for i32 {
            $(
                fn $fn_name(self) -> $crate::default_declarators::$storage_scale<i32> {
                    $crate::default_declarators::$storage_scale::new((self as f64 * $conversion_factor) as i32)
                }
            )*
        }

        // Generate extension trait implementations for i64
        impl $trait_name<i64> for i64 {
            $(
                fn $fn_name(self) -> $crate::default_declarators::$storage_scale<i64> {
                    $crate::default_declarators::$storage_scale::new((self as f64 * $conversion_factor) as i64)
                }
            )*
        }
    };
}

define_imperial_quantity!(
    0,
    1,
    0,
    0,
    0,
    0,
    0,
    0, // length dimension
    ImperialLength,
    (inches, 2.54, Centimeter),
    (feet, 0.3048, Meter),
    (yards, 0.9144, Meter),
    (miles, 1.609344, Kilometer),
);

define_imperial_quantity!(
    1,
    0,
    0,
    0,
    0,
    0,
    0,
    0, // mass dimension
    ImperialMass,
    (ounces, 28.349523125, Gram),
    (pounds, 0.45359237, Kilogram),
    (stones, 6.35029318, Kilogram),
    (tons, 1.0160469088, Megagram),
);

#[doc(hidden)]
macro_rules! define_imperial_affine_quantity {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr, $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident,
        $storage_scale:ident,
        $(($scale_name:ident, $fn_name:ident, $conversion_factor:expr, $offset:expr)),* $(,)?
    ) => {
        // Generate the trait definition (generic over storage type)
        pub trait $trait_name<T = f64> {
            $(
                fn $fn_name(self) -> $scale_name<T>;
            )*
        }

        // Generate the type definitions (all stored in the same scale, generic with f64 default)
        $(
            pub type $scale_name<T = f64> = $crate::default_declarators::$storage_scale<T>;
        )*

        // Generate extension trait implementations for f64 (default)
        impl $trait_name<f64> for f64 {
            $(
                fn $fn_name(self) -> $scale_name<f64> {
                    $crate::default_declarators::$storage_scale::new(self * $conversion_factor + $offset)
                }
            )*
        }

        // Generate extension trait implementations for i32
        impl $trait_name<i32> for i32 {
            $(
                fn $fn_name(self) -> $scale_name<i32> {
                    $crate::default_declarators::$storage_scale::new((self as f64 * $conversion_factor + $offset) as i32)
                }
            )*
        }

        // Generate extension trait implementations for i64
        impl $trait_name<i64> for i64 {
            $(
                fn $fn_name(self) -> $scale_name<i64> {
                    $crate::default_declarators::$storage_scale::new((self as f64 * $conversion_factor + $offset) as i64)
                }
            )*
        }
    };
}

define_imperial_affine_quantity!(
    0,
    0,
    0,
    0,
    1,
    0,
    0,
    0, // temperature dimension
    ImperialTemperature,
    Kelvin,
    (Fahrenheit, fahrenheit, 5.0 / 9.0, 255.3722222222222), // °F to K: (F - 32) * 5/9 + 273.15
    (Rankine, rankine, 5.0 / 9.0, 0.0),                     // °R to K: R * 5/9
);
