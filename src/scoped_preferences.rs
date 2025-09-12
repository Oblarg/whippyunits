use crate::generated_quantity_type::Quantity;
use crate::default_declarators;

#[macro_export]
macro_rules! mass_scale_p10 {
    (Picogram) => { -15 };
    (Nanogram) => { -12 };
    (Microgram) => { -9 };
    (Milligram) => { -6 };
    (Centigram) => { -5 };
    (Decigram) => { -4 };
    (Gram) =>  { -3 };
    (Decagram) => { -2 };
    (Hectogram) => { -1 };
    (Kilogram) => { 0 };
    (Megagram) => { 3 };
    (Gigagram) => { 6 };
    (Teragram) => { 9 };
    (Petagram) => { 12 };
    (Exagram) => { 15 };
    (Zettagram) => { 18 };
    (Yottagram) => { 21 };
}

#[macro_export]
macro_rules! length_scale_p10 {
    (Picometer) => { -12 };
    (Nanometer) => { -9 };
    (Micrometer) => { -6 };
    (Millimeter) => { -3 };
    (Centimeter) => { -2 };
    (Decimeter) => { -1 };
    (Meter) =>  { 0 };
    (Decameter) => { 1 };
    (Hectometer) => { 2 };
    (Kilometer) => { 3 };
    (Megameter) => { 6 };
    (Gigameter) => { 9 };
    (Terameter) => { 12 };
    (Petameter) => { 15 };
    (Exameter) => { 18 };
    (Zettameter) => { 21 };
    (Yottameter) => { 24 };
}

#[macro_export]
macro_rules! time_scale_p2 {
    (Picosecond) => { -12 };
    (Nanosecond) => { -9 };
    (Microsecond) => { -6 };
    (Millisecond) => { -3 };
    (Centisecond) => { -2 };
    (Decisecond) => { -1 };
    (Second) =>  { 0 };
    (Decasecond) => { 1 };
    (Hectosecond) => { 2 };
    (Kilosecond) => { 3 };
    (Megasecond) => { 6 };
    (Gigasecond) => { 9 };
    (Terasecond) => { 12 };
    (Petasecond) => { 15 };
    (Exasecond) => { 18 };
    (Zettasecond) => { 21 };
    (Yottasecond) => { 24 };

    (Minute) => { 2 };
    (Hour) => { 4 };
    (Day) => { 7 };
}

#[macro_export]
macro_rules! time_scale_p3 {
    (Minute) => { 1 };
    (Hour) => { 2 };
    (Day) => { 3 };
    ($other:ident) => { 0 };
}

#[macro_export]
macro_rules! time_scale_p5 {
    (Minute) => { 2 };
    (Hour) => { 4 };
    (Day) => { 7 };
    ($other:ident) => { $crate::time_scale_p2!($other)};
}

#[macro_export]
macro_rules! current_scale_p10 {
    (Picoampere) => { -12 };
    (Nanoampere) => { -9 };
    (Microampere) => { -6 };
    (Milliampere) => { -3 };
    (Centiampere) => { -2 };
    (Deciampere) => { -1 };
    (Ampere) =>  { 0 };
    (Decaampere) => { 1 };
    (Hectoampere) => { 2 };
    (Kiloampere) => { 3 };
    (Megaampere) => { 6 };
    (Gigaampere) => { 9 };
    (Teraampere) => { 12 };
    (Petaampere) => { 15 };
    (Exaampere) => { 18 };
    (Zettaampere) => { 21 };
    (Yottaampere) => { 24 };
}

#[macro_export]
macro_rules! temperature_scale_p10 {
    (Picokelvin) => { -12 };
    (Nanokelvin) => { -9 };
    (Microkelvin) => { -6 };
    (Millikelvin) => { -3 };
    (Centikelvin) => { -2 };
    (Decikelvin) => { -1 };
    (Kelvin) =>  { 0 };
    (Decakelvin) => { 1 };
    (Hectokelvin) => { 2 };
    (Kilokelvin) => { 3 };
    (Megakelvin) => { 6 };
    (Gigakelvin) => { 9 };
    (Terakelvin) => { 12 };
    (Petakelvin) => { 15 };
    (Exakelvin) => { 18 };
    (Zettakelvin) => { 21 };
    (Yottakelvin) => { 24 };
}

#[macro_export]
macro_rules! amount_scale_p10 {
    (Picomole) => { -12 };
    (Nanomole) => { -9 };
    (Micromole) => { -6 };
    (Millimole) => { -3 };
    (Centimole) => { -2 };
    (Decimole) => { -1 };
    (Mole) =>  { 0 };
    (Decamole) => { 1 };
    (Hectomole) => { 2 };
    (Kilomole) => { 3 };
    (Megamole) => { 6 };
    (Gigamole) => { 9 };
    (Teramole) => { 12 };
    (Petamole) => { 15 };
    (Examole) => { 18 };
    (Zettamole) => { 21 };
    (Yottamole) => { 24 };
}

#[macro_export]
macro_rules! luminosity_scale_p10 {
    (Picocandela) => { -12 };
    (Nanocandela) => { -9 };
    (Microcandela) => { -6 };
    (Millicandela) => { -3 };
    (Centicandela) => { -2 };
    (Decicandela) => { -1 };
    (Candela) =>  { 0 };
    (Decacandela) => { 1 };
    (Hectocandela) => { 2 };
    (Kilocandela) => { 3 };
    (Megacandela) => { 6 };
    (Gigacandela) => { 9 };
    (Teracandela) => { 12 };
    (Petacandela) => { 15 };
    (Exacandela) => { 18 };
    (Zettacandela) => { 21 };
    (Yottacandela) => { 24 };
}

#[macro_export]
macro_rules! angle_scale_p2 {
    (Picoradian) => { -12 };
    (Nanoradian) => { -9 };
    (Microradian) => { -6 };
    (Milliradian) => { -3 };
    (Centiradian) => { -2 };
    (Deciradian) => { -1 };
    (Radian) =>  { 0 };
    (Decaradian) => { 1 };
    (Hectoradian) => { 2 };
    (Kiloradian) => { 3 };
    (Megaradian) => { 6 };
    (Gigaradian) => { 9 };
    (Teraradian) => { 12 };
    (Petaradian) => { 15 };
    (Exaradian) => { 18 };
    (Zettaradian) => { 21 };
    (Yottaradian) => { 24 };

    (Turn) => { 1 };
    (Degree) => { -2 };
    (Gradian) => { -3 };
    (Arcminute) => { -4 };
    (Arcsecond) => { -6 };
}

#[macro_export]
macro_rules! angle_scale_p3 {
    (Degree) => { -2 };
    (Arcminute) => { -3 };
    (Arcsecond) => { -6 };
    ($other:ident) => { 0 };
}

#[macro_export]
macro_rules! angle_scale_p5 {
    (Degree) => { -1 };
    (Gradian) => { -2 };
    (Arcminute) => { -2 };
    (Arcsecond) => { -3 };
    ($other:ident) => { $crate::angle_scale_p2!($other) };
}

#[macro_export]
macro_rules! angle_scale_pi {
    (Turn) => { 1 };
    (Degree) => { 1 };
    (Gradian) => { 1 };
    (Arcminute) => { 1 };
    (Arcsecond) => { 1 };
    ($other:ident) => { 0 };
}

#[macro_export]
macro_rules! define_local_mass_quantity {
    (
        $local_mass_scale:ident,
        $mass_scale_p10:expr,
        $length_scale_p10:expr,
        $time_scale_p2:expr, $time_scale_p3:expr, $time_scale_p5:expr,
        $electric_current_scale_p10:expr,
        $temperature_scale_p10:expr,
        $amount_of_substance_scale_p10:expr,
        $luminous_intensity_scale_p10:expr,
        $angle_scale_p2:expr, $angle_scale_p3:expr, $angle_scale_p5:expr, $angle_scale_pi:expr,
        $trait_name:ident,$(($scale_name:ident, $fn_name:ident)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $local_mass_scale;
            )*
        }
        
        pub type $local_mass_scale = Quantity<
            1, $mass_scale_p10,
            0, $length_scale_p10,
            0, $time_scale_p2, $time_scale_p3, $time_scale_p5,
            0, $electric_current_scale_p10,
            0, $temperature_scale_p10,
            0, $amount_of_substance_scale_p10,
            0, $luminous_intensity_scale_p10,
            0, $angle_scale_p2, $angle_scale_p3, $angle_scale_p5, $angle_scale_pi,
            f64,
        >;
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $local_mass_scale {
                    rescale_f64($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! define_local_length_quantity {
    (
        $local_length_scale:ident,
        $mass_scale_p10:expr,
        $length_scale_p10:expr,
        $time_scale_p2:expr, $time_scale_p3:expr, $time_scale_p5:expr,
        $electric_current_scale_p10:expr,
        $temperature_scale_p10:expr,
        $amount_of_substance_scale_p10:expr,
        $luminous_intensity_scale_p10:expr,
        $angle_scale_p2:expr, $angle_scale_p3:expr, $angle_scale_p5:expr, $angle_scale_pi:expr,
        $trait_name:ident, $(($scale_name:ident, $fn_name:ident)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $local_length_scale;
            )*
        }
        
        pub type $local_length_scale = Quantity<
            0, $mass_scale_p10,
            1, $length_scale_p10,
            0, $time_scale_p2, $time_scale_p3, $time_scale_p5,
            0, $electric_current_scale_p10,
            0, $temperature_scale_p10,
            0, $amount_of_substance_scale_p10,
            0, $luminous_intensity_scale_p10,
            0, $angle_scale_p2, $angle_scale_p3, $angle_scale_p5, $angle_scale_pi,
            f64,
        >;
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $local_length_scale {
                    rescale_f64($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! define_local_time_quantity {
    (
        $local_time_scale:ident,
        $mass_scale_p10:expr,
        $length_scale_p10:expr,
        $time_scale_p2:expr, $time_scale_p3:expr, $time_scale_p5:expr,
        $electric_current_scale_p10:expr,
        $temperature_scale_p10:expr,
        $amount_of_substance_scale_p10:expr,
        $luminous_intensity_scale_p10:expr,
        $angle_scale_p2:expr, $angle_scale_p3:expr, $angle_scale_p5:expr, $angle_scale_pi:expr,
        $trait_name:ident, $(($scale_name:ident, $fn_name:ident)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $local_time_scale;
            )*
        }
        
        pub type $local_time_scale = Quantity<
            0, $mass_scale_p10,
            0, $length_scale_p10,
            1, $time_scale_p2, $time_scale_p3, $time_scale_p5,
            0, $electric_current_scale_p10,
            0, $temperature_scale_p10,
            0, $amount_of_substance_scale_p10,
            0, $luminous_intensity_scale_p10,
            0, $angle_scale_p2, $angle_scale_p3, $angle_scale_p5, $angle_scale_pi,
            f64,
        >;
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $local_time_scale {
                    rescale_f64($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! define_local_current_quantity {
    (
        $local_current_scale:ident,
        $mass_scale_p10:expr,
        $length_scale_p10:expr,
        $time_scale_p2:expr, $time_scale_p3:expr, $time_scale_p5:expr,
        $electric_current_scale_p10:expr,
        $temperature_scale_p10:expr,
        $amount_of_substance_scale_p10:expr,
        $luminous_intensity_scale_p10:expr,
        $angle_scale_p2:expr, $angle_scale_p3:expr, $angle_scale_p5:expr, $angle_scale_pi:expr,
        $trait_name:ident, $(($scale_name:ident, $fn_name:ident)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $local_current_scale;
            )*
        }
        
        pub type $local_current_scale = Quantity<
            0, $mass_scale_p10,
            0, $length_scale_p10,
            0, $time_scale_p2, $time_scale_p3, $time_scale_p5,
            1, $electric_current_scale_p10,
            0, $temperature_scale_p10,
            0, $amount_of_substance_scale_p10,
            0, $luminous_intensity_scale_p10,
            0, $angle_scale_p2, $angle_scale_p3, $angle_scale_p5, $angle_scale_pi,
            f64,
        >;
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $local_current_scale {
                    rescale_f64($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! define_local_temperature_quantity {
    (
        $local_temperature_scale:ident,
        $mass_scale_p10:expr,
        $length_scale_p10:expr,
        $time_scale_p2:expr, $time_scale_p3:expr, $time_scale_p5:expr,
        $electric_current_scale_p10:expr,
        $temperature_scale_p10:expr,
        $amount_of_substance_scale_p10:expr,
        $luminous_intensity_scale_p10:expr,
        $angle_scale_p2:expr, $angle_scale_p3:expr, $angle_scale_p5:expr, $angle_scale_pi:expr,
        $trait_name:ident, $(($scale_name:ident, $fn_name:ident)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $local_temperature_scale;
            )*
        }
        
        pub type $local_temperature_scale = Quantity<
            0, $mass_scale_p10,
            0, $length_scale_p10,
            0, $time_scale_p2, $time_scale_p3, $time_scale_p5,
            0, $electric_current_scale_p10,
            1, $temperature_scale_p10,
            0, $amount_of_substance_scale_p10,
            0, $luminous_intensity_scale_p10,
            0, $angle_scale_p2, $angle_scale_p3, $angle_scale_p5, $angle_scale_pi,
            f64,
        >;
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $local_temperature_scale {
                    rescale_f64($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! define_local_amount_quantity {
    (
        $local_amount_scale:ident,
        $mass_scale_p10:expr,
        $length_scale_p10:expr,
        $time_scale_p2:expr, $time_scale_p3:expr, $time_scale_p5:expr,
        $electric_current_scale_p10:expr,
        $temperature_scale_p10:expr,
        $amount_of_substance_scale_p10:expr,
        $luminous_intensity_scale_p10:expr,
        $angle_scale_p2:expr, $angle_scale_p3:expr, $angle_scale_p5:expr, $angle_scale_pi:expr,
        $trait_name:ident, $(($scale_name:ident, $fn_name:ident)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $local_amount_scale;
            )*
        }
        
        pub type $local_amount_scale = Quantity<
            0, $mass_scale_p10,
            0, $length_scale_p10,
            0, $time_scale_p2, $time_scale_p3, $time_scale_p5,
            0, $electric_current_scale_p10,
            0, $temperature_scale_p10,
            1, $amount_of_substance_scale_p10,
            0, $luminous_intensity_scale_p10,
            0, $angle_scale_p2, $angle_scale_p3, $angle_scale_p5, $angle_scale_pi,
            f64,
        >;
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $local_amount_scale {
                    rescale_f64($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! define_local_luminosity_quantity {
    (
        $local_luminosity_scale:ident,
        $mass_scale_p10:expr,
        $length_scale_p10:expr,
        $time_scale_p2:expr, $time_scale_p3:expr, $time_scale_p5:expr,
        $electric_current_scale_p10:expr,
        $temperature_scale_p10:expr,
        $amount_of_substance_scale_p10:expr,
        $luminous_intensity_scale_p10:expr,
        $angle_scale_p2:expr, $angle_scale_p3:expr, $angle_scale_p5:expr, $angle_scale_pi:expr,
        $trait_name:ident, $(($scale_name:ident, $fn_name:ident)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $local_luminosity_scale;
            )*
        }
        
        pub type $local_luminosity_scale = Quantity<
            0, $mass_scale_p10,
            0, $length_scale_p10,
            0, $time_scale_p2, $time_scale_p3, $time_scale_p5,
            0, $electric_current_scale_p10,
            0, $temperature_scale_p10,
            0, $amount_of_substance_scale_p10,
            1, $luminous_intensity_scale_p10,
            0, $angle_scale_p2, $angle_scale_p3, $angle_scale_p5, $angle_scale_pi,
            f64,
        >;
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $local_luminosity_scale {
                    rescale_f64($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! define_local_angle_quantity {
    (
        $local_angle_scale:ident,
        $mass_scale_p10:expr,
        $length_scale_p10:expr,
        $time_scale_p2:expr, $time_scale_p3:expr, $time_scale_p5:expr,
        $electric_current_scale_p10:expr,
        $temperature_scale_p10:expr,
        $amount_of_substance_scale_p10:expr,
        $luminous_intensity_scale_p10:expr,
        $angle_scale_p2:expr, $angle_scale_p3:expr, $angle_scale_p5:expr, $angle_scale_pi:expr,
        $trait_name:ident, $(($scale_name:ident, $fn_name:ident)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $local_angle_scale;
            )*
        }
        
        pub type $local_angle_scale = Quantity<
            0, $mass_scale_p10,
            0, $length_scale_p10,
            0, $time_scale_p2, $time_scale_p3, $time_scale_p5,
            0, $electric_current_scale_p10,
            0, $temperature_scale_p10,
            0, $amount_of_substance_scale_p10,
            0, $luminous_intensity_scale_p10,
            1, $angle_scale_p2, $angle_scale_p3, $angle_scale_p5, $angle_scale_pi,
            f64,
        >;
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $local_angle_scale {
                    rescale_f64($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }
    };
}

#[rustfmt::skip]
#[macro_export]
macro_rules! set_unit_preferences {
    (
        $mass_scale:ident,
        $length_scale:ident, 
        $time_scale:ident,
        $current_scale:ident,
        $temperature_scale:ident,
        $amount_scale:ident,
        $luminosity_scale:ident,
        $angle_scale:ident
    ) => {
        $crate::define_local_mass_quantity!(
            $mass_scale,
            { $crate::mass_scale_p10!($mass_scale) },
            { $crate::length_scale_p10!($length_scale) },
            { $crate::time_scale_p2!($time_scale) }, { $crate::time_scale_p3!($time_scale) }, { $crate::time_scale_p2!($time_scale) },
            { $crate::current_scale_p10!($current_scale) },
            { $crate::temperature_scale_p10!($temperature_scale) },
            { $crate::amount_scale_p10!($amount_scale) },
            { $crate::luminosity_scale_p10!($luminosity_scale) },
            { $crate::angle_scale_p2!($angle_scale) }, { $crate::angle_scale_p3!($angle_scale) }, { $crate::angle_scale_p5!($angle_scale) }, { $crate::angle_scale_pi!($angle_scale) },
            LocalMass,
            (Picogram, picograms),
            (Nanogram, nanograms),
            (Microgram, micrograms),
            (Milligram, milligrams),
            (Centigram, centigrams),
            (Decigram, decigrams),
            (Gram, grams),
            (Decagram, decagrams),
            (Hectogram, hectograms),
            (Kilogram, kilograms),
            (Megagram, megagrams),
            (Gigagram, gigagrams),
            (Teragram, teragrams),
            (Petagram, petagrams),
            (Exagram, exagrams),
            (Zettagram, zettagrams),
            (Yottagram, yottagrams),
        );

        $crate::define_local_length_quantity!(
            $length_scale,
            { $crate::mass_scale_p10!($mass_scale) },
            { $crate::length_scale_p10!($length_scale) },
            { $crate::time_scale_p2!($time_scale) }, { $crate::time_scale_p3!($time_scale) }, { $crate::time_scale_p2!($time_scale) },
            { $crate::current_scale_p10!($current_scale) },
            { $crate::temperature_scale_p10!($temperature_scale) },
            { $crate::amount_scale_p10!($amount_scale) },
            { $crate::luminosity_scale_p10!($luminosity_scale) },
            { $crate::angle_scale_p2!($angle_scale) }, { $crate::angle_scale_p3!($angle_scale) }, { $crate::angle_scale_p5!($angle_scale) }, { $crate::angle_scale_pi!($angle_scale) },
            LocalLength,
            (Picometer, picometers),
            (Nanometer, nanometers),
            (Micrometer, micrometers),
            (Millimeter, millimeters),
            (Centimeter, centimeters),
            (Decimeter, decimeters),
            (Meter, meters),
            (Decameter, decameters),
            (Hectometer, hectometers),
            (Kilometer, kilometers),
            (Megameter, megameters),
            (Gigameter, gigameters),
            (Terameter, terameters),
            (Petameter, petameters),
            (Exameter, exameters),
            (Zettameter, zettameters),
            (Yottameter, yottameters),
        );

        $crate::define_local_time_quantity!(
            $time_scale,
            { $crate::mass_scale_p10!($mass_scale) },
            { $crate::length_scale_p10!($length_scale) },
            { $crate::time_scale_p2!($time_scale) }, { $crate::time_scale_p3!($time_scale) }, { $crate::time_scale_p2!($time_scale) },
            { $crate::current_scale_p10!($current_scale) },
            { $crate::temperature_scale_p10!($temperature_scale) },
            { $crate::amount_scale_p10!($amount_scale) },
            { $crate::luminosity_scale_p10!($luminosity_scale) },
            { $crate::angle_scale_p2!($angle_scale) }, { $crate::angle_scale_p3!($angle_scale) }, { $crate::angle_scale_p5!($angle_scale) }, { $crate::angle_scale_pi!($angle_scale) },
            LocalTime,
            (Picosecond, picoseconds),
            (Nanosecond, nanoseconds),
            (Microsecond, microseconds),
            (Millisecond, milliseconds),
            (Centisecond, centiseconds),
            (Decisecond, deciseconds),
            (Second, seconds),
            (Decasecond, decaseconds),
            (Hectosecond, hectoseconds),
            (Kilosecond, kiloseconds),
            (Megasecond, megaseconds),
            (Gigasecond, gigaseconds),
            (Terasecond, teraseconds),
            (Petasecond, petaseconds),
            (Exasecond, exaseconds),
            (Zettasecond, zettaseconds),
            (Yottasecond, yottaseconds),
            (Minute, minutes),
            (Hour, hours),
            (Day, days),
        );

        $crate::define_local_current_quantity!(
            $current_scale,
            { $crate::mass_scale_p10!($mass_scale) },
            { $crate::length_scale_p10!($length_scale) },
            { $crate::time_scale_p2!($time_scale) }, { $crate::time_scale_p3!($time_scale) }, { $crate::time_scale_p2!($time_scale) },
            { $crate::current_scale_p10!($current_scale) },
            { $crate::temperature_scale_p10!($temperature_scale) },
            { $crate::amount_scale_p10!($amount_scale) },
            { $crate::luminosity_scale_p10!($luminosity_scale) },
            { $crate::angle_scale_p2!($angle_scale) }, { $crate::angle_scale_p3!($angle_scale) }, { $crate::angle_scale_p5!($angle_scale) }, { $crate::angle_scale_pi!($angle_scale) },
            LocalCurrent,
            (Picoampere, picoamperes),
            (Nanoampere, nanoamperes),
            (Microampere, microamperes),
            (Milliampere, milliamperes),
            (Centiampere, centiamperes),
            (Deciampere, deciamperes),
            (Ampere, amperes),
            (Decaampere, decaamperes),
            (Hectoampere, hectoamperes),
            (Kiloampere, kiloamperes),
            (Megaampere, megaamperes),
            (Gigaampere, gigaamperes),
            (Teraampere, teraamperes),
            (Petaampere, petaamperes),
            (Exaampere, exaamperes),
            (Zettaampere, zettaamperes),
            (Yottaampere, yottaamperes),
        );

        $crate::define_local_amount_quantity!(
            $amount_scale,
            { $crate::mass_scale_p10!($mass_scale) },
            { $crate::length_scale_p10!($length_scale) },
            { $crate::time_scale_p2!($time_scale) }, { $crate::time_scale_p3!($time_scale) }, { $crate::time_scale_p2!($time_scale) },
            { $crate::current_scale_p10!($current_scale) },
            { $crate::temperature_scale_p10!($temperature_scale) },
            { $crate::amount_scale_p10!($amount_scale) },
            { $crate::luminosity_scale_p10!($luminosity_scale) },
            { $crate::angle_scale_p2!($angle_scale) }, { $crate::angle_scale_p3!($angle_scale) }, { $crate::angle_scale_p5!($angle_scale) }, { $crate::angle_scale_pi!($angle_scale) },
            LocalAmount,
            (Picomole, picomoles),
            (Nanomole, nanomoles),
            (Micromole, micromoles),
            (Millimole, millimoles),
            (Centimole, centimoles),
            (Decimole, decimoles),
            (Mole, moles),
            (Decamole, decamoles),
            (Hectomole, hectomoles),
            (Kilomole, kilomoles),
            (Megamole, megamoles),
            (Gigamole, gigamoles),
            (Teramole, teramoles),
            (Petamole, petamoles),
            (Examole, examoles),
            (Zettamole, zettamoles),
            (Yottamole, yottamoles),
        );

        $crate::define_local_luminosity_quantity!(
            $luminosity_scale,
            { $crate::mass_scale_p10!($mass_scale) },
            { $crate::length_scale_p10!($length_scale) },
            { $crate::time_scale_p2!($time_scale) }, { $crate::time_scale_p3!($time_scale) }, { $crate::time_scale_p2!($time_scale) },
            { $crate::current_scale_p10!($current_scale) },
            { $crate::temperature_scale_p10!($temperature_scale) },
            { $crate::amount_scale_p10!($amount_scale) },
            { $crate::luminosity_scale_p10!($luminosity_scale) },
            { $crate::angle_scale_p2!($angle_scale) }, { $crate::angle_scale_p3!($angle_scale) }, { $crate::angle_scale_p5!($angle_scale) }, { $crate::angle_scale_pi!($angle_scale) },
            LocalLuminosity,
            (Picocandela, picocandelas),
            (Nanocandela, nanocandelas),
            (Microcandela, microcandelas),
            (Millicandela, millicandelas),
            (Centicandela, centicandelas),
            (Decicandela, decicandelas),
            (Candela, candelas),
            (Decacandela, decacandelas),
            (Hectocandela, hectocandelas),
            (Kilocandela, kilocandelas),
            (Megacandela, megacandelas),
            (Gigacandela, gigacandelas),
            (Teracandela, teracandelas),
            (Petacandela, petacandelas),
            (Exacandela, exacandelas),
            (Zettacandela, zettacandelas),
            (Yottacandela, yottacandelas),
        );

        $crate::define_local_angle_quantity!(
            $angle_scale,
            { $crate::mass_scale_p10!($mass_scale) },
            { $crate::length_scale_p10!($length_scale) },
            { $crate::time_scale_p2!($time_scale) }, { $crate::time_scale_p3!($time_scale) }, { $crate::time_scale_p2!($time_scale) },
            { $crate::current_scale_p10!($current_scale) },
            { $crate::temperature_scale_p10!($temperature_scale) },
            { $crate::amount_scale_p10!($amount_scale) },
            { $crate::luminosity_scale_p10!($luminosity_scale) },
            { $crate::angle_scale_p2!($angle_scale) }, { $crate::angle_scale_p3!($angle_scale) }, { $crate::angle_scale_p5!($angle_scale) }, { $crate::angle_scale_pi!($angle_scale) },
            LocalAngle,
            (Picoradian, picoradians),
            (Nanoradian, nanoradians),
            (Microradian, microradians),
            (Milliradian, milliradians),
            (Centiradian, centiradians),
            (Deciradian, deciradians),
            (Radian, radians),
            (Decaradian, decaradians),
            (Hectoradian, hectoradians),
            (Kiloradian, kiloradians),
            (Megaradian, megaradians),
            (Gigaradian, gigaradians),
            (Teraradian, teraradians),
            (Petaradian, petaradians),
            (Exaradian, exaradians),
            (Zettaradian, zettaradians),
            (Yottaradian, yottaradians),
            (Turn, turns),
            (Degrees, degrees),
            (Gradians, gradians),
            (Arcminutes, arcminutes),
            (Arcseconds, arcseconds),
        );

        $crate::define_unit_macro!(
            { $crate::mass_scale_p10!($mass_scale) },
            { $crate::length_scale_p10!($length_scale) },
            { $crate::time_scale_p2!($time_scale) }, { $crate::time_scale_p3!($time_scale) }, { $crate::time_scale_p2!($time_scale) },
            { $crate::current_scale_p10!($current_scale) },
            { $crate::temperature_scale_p10!($temperature_scale) },
            { $crate::amount_scale_p10!($amount_scale) },
            { $crate::luminosity_scale_p10!($luminosity_scale) },
            { $crate::angle_scale_p2!($angle_scale) }, { $crate::angle_scale_p3!($angle_scale) }, { $crate::angle_scale_p5!($angle_scale) }, { $crate::angle_scale_pi!($angle_scale) }
        );
    };
}
