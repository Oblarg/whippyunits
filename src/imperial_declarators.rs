
macro_rules! define_imperial_quantity {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr, $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident,
        $(($scale_name:ident, $fn_name:ident, $conversion_factor:expr, $storage_scale:ident)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }
        
        // Generate the type definitions with individual storage scales
        $(
            pub type $scale_name = $crate::default_declarators::$storage_scale;
        )*
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    $crate::default_declarators::$storage_scale::new(self * $conversion_factor)
                }
            )*
        }
        
        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    $crate::default_declarators::$storage_scale::new((self as f64) * $conversion_factor)
                }
            )*
        }
    };
}

define_imperial_quantity!(
    0, 1, 0, 0, 0, 0, 0, 0,  // length dimension
    ImperialLength,
    (Inch, inches, 2.54, Centimeter),     
    (Foot, feet, 0.3048, Meter),
    (Yard, yards, 0.9144, Meter),
    (Mile, miles, 1.609344, Kilometer),
);

define_imperial_quantity!(
    1, 0, 0, 0, 0, 0, 0, 0,  // mass dimension
    ImperialMass,
    (Ounce, ounces, 28.349523125, Gram),
    (Pound, pounds, 0.45359237, Kilogram),
    (Stone, stones, 6.35029318, Kilogram),
    (Ton, tons, 1.0160469088, Megagram),
);

macro_rules! define_imperial_affine_quantity {
    (
        $mass_exp:expr, $length_exp:expr, $time_exp:expr, $current_exp:expr, $temperature_exp:expr, $amount_exp:expr, $luminosity_exp:expr, $angle_exp:expr,
        $trait_name:ident,
        $storage_scale:ident,
        $(($scale_name:ident, $fn_name:ident, $conversion_factor:expr, $offset:expr)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }
        
        // Generate the type definitions (all stored in the same scale)
        $(
            pub type $scale_name = $crate::default_declarators::$storage_scale;
        )*
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    $crate::default_declarators::$storage_scale::new(self * $conversion_factor + $offset)
                }
            )*
        }
        
        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    $crate::default_declarators::$storage_scale::new((self as f64) * $conversion_factor + $offset)
                }
            )*
        }
    };
}

define_imperial_affine_quantity!(
    0, 0, 0, 0, 1, 0, 0, 0,  // temperature dimension
    ImperialTemperature,
    Kelvin,
    (Fahrenheit, fahrenheit, 5.0/9.0, 255.3722222222222),  // °F to K: (F - 32) * 5/9 + 273.15
    (Rankine, rankine, 5.0/9.0, 0.0),                      // °R to K: R * 5/9
);