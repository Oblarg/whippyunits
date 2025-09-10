use crate::generated_quantity_type::Quantity;

macro_rules! define_mass_quantity {
    ($trait_name:ident,$(($scale_name:ident, $fn_name:ident, $scale_exponent:expr)),* $(,)?) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }
        
        // Generate the type definitions
        $(
            pub type $scale_name = Quantity<
                1, $scale_exponent,
                0, 0,
                0, 0, 0, 0,
                0, 0,
                0, 0,
                0, 0,
                0, 0,
                0, 0, 0, 0, 0,
                f64,
            >;
        )*
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self)
                }
            )*
        }
        
        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self as f64)
                }
            )*
        }
    };
}

macro_rules! define_length_quantity {
    ($trait_name:ident, $(($scale_name:ident, $fn_name:ident, $scale_exponent:expr)),* $(,)?) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }
        
        // Generate the type definitions
        $(
            pub type $scale_name = Quantity<
                0, 0,
                1, $scale_exponent,
                0, 0, 0, 0,
                0, 0,
                0, 0,
                0, 0,
                0, 0,
                0, 0, 0, 0, 0,
                f64,
            >;
        )*
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self)
                }
            )*
        }
        
        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self as f64)
                }
            )*
        }
    };
}

macro_rules! define_time_quantity {
    ($trait_name:ident, $(($scale_name:ident, $fn_name:ident, $scale_p2:expr, $scale_p3:expr, $scale_p5:expr)),* $(,)?) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }
        
        // Generate the type definitions
        $(
            pub type $scale_name = Quantity<
                0, 0,
                0, 0,
                1, $scale_p2, $scale_p3, $scale_p5,
                0, 0,
                0, 0,
                0, 0,
                0, 0,
                0, 0, 0, 0, 0,
                f64,
            >;
        )*
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self)
                }
            )*
        }
        
        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self as f64)
                }
            )*
        }
    };
}

macro_rules! define_current_quantity {
    ($trait_name: ident, $(($scale_name:ident, $fn_name:ident, $scale_exponent:expr)),* $(,)?) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }
        
        // Generate the type definitions
        $(
            pub type $scale_name = Quantity<
                0, 0,
                0, 0,
                0, 0, 0, 0,
                1, $scale_exponent,
                0, 0,
                0, 0,
                0, 0,
                0, 0, 0, 0, 0,
                f64,
            >;
        )*
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self)
                }
            )*
        }
        
        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self as f64)
                }
            )*
        }
    };
}

macro_rules! define_temperature_quantity {
    ($trait_name: ident, $(($scale_name:ident, $fn_name:ident, $scale_exponent:expr)),* $(,)?) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }
        
        // Generate the type definitions
        $(
            pub type $scale_name = Quantity<
                0, 0,
                0, 0,
                0, 0, 0, 0,
                0, 0,
                1, $scale_exponent,
                0, 0,
                0, 0,
                0, 0, 0, 0, 0,
                f64,
            >;
        )*
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self)
                }
            )*
        }
        
        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self as f64)
                }
            )*
        }
    };
}

macro_rules! define_amount_quantity {
    ($trait_name: ident, $(($scale_name:ident, $fn_name:ident, $scale_exponent:expr)),* $(,)?) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }
        
        // Generate the type definitions
        $(
            pub type $scale_name = Quantity<
                0, 0,
                0, 0,
                0, 0, 0, 0,
                0, 0,
                0, 0,
                1, $scale_exponent,
                0, 0,
                0, 0, 0, 0, 0,
                f64,
            >;
        )*
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self)
                }
            )*
        }
        
        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self as f64)
                }
            )*
        }
    };
}

macro_rules! define_luminosity_quantity {
    ($trait_name:ident, $(($scale_name:ident, $fn_name:ident, $scale_exponent:expr)),* $(,)?) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }
        
        // Generate the type definitions
        $(
            pub type $scale_name = Quantity<
                0, 0,
                0, 0,
                0, 0, 0, 0,
                0, 0,
                0, 0,
                0, 0,
                1, $scale_exponent,
                0, 0, 0, 0, 0,
                f64,
            >;
        )*
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self)
                }
            )*
        }
        
        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self as f64)
                }
            )*
        }
    };
}

macro_rules! define_angle_quantity {
    ($trait_name:ident, $(($scale_name:ident, $fn_name:ident, $scale_p2:expr, $scale_p3:expr, $scale_p5:expr, $scale_pi:expr)),* $(,)?) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $scale_name;
            )*
        }
        
        // Generate the type definitions
        $(
            pub type $scale_name = Quantity<
                0, 0,
                0, 0,
                0, 0, 0, 0,
                0, 0,
                0, 0,
                0, 0,
                0, 0,
                1, $scale_p2, $scale_p3, $scale_p5, $scale_pi,
                f64,
            >;
        )*
        
        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self)
                }
            )*
        }
        
        // Generate extension trait implementations for i32
        impl $trait_name for i32 {
            $(
                fn $fn_name(self) -> $scale_name {
                    Quantity::new(self as f64)
                }
            )*
        }
    };
}

define_mass_quantity!(
    SIMass,
    (Picogram, picograms, -15),
    (Nanogram, nanograms, -12),
    (Microgram, micrograms, -9),
    (Milligram, milligrams, -6),
    (Centigram, centigrams, -5),
    (Decigram, decigrams, -4),
    (Gram, grams, -3),
    (Decagram, decagrams, -2),
    (Hectogram, hectograms, -1),
    (Kilogram, kilograms, 0),
    (Megagram, megagrams, 3),
    (Gigagram, gigagrams, 6),
    (Teragram, teragrams, 9),
    (Petagram, petagrams, 12),
    (Exagram, exagrams, 15),
    (Zettagram, zettagrams, 18),
    (Yottagram, yottagrams, 21),
);

define_length_quantity!(
    SILength,
    (Picometer, picometers, -15),
    (Nanometer, nanometers, -12),
    (Micrometer, micrometers, -9),
    (Millimeter, millimeters, -3),
    (Centimeter, centimeters, -2),
    (Decimeter, decimeters, -1),
    (Meter, meters, 0),
    (Decameter, decameters, 1),
    (Hectometer, hectometers, 2),
    (Kilometer, kilometers, 3),
    (Megameter, megameters, 6),
    (Gigameter, gigameters, 9),
    (Terameter, terameters, 12),
    (Petameter, petameters, 15),
    (Exameter, exameters, 18),
    (Zettameter, zettameters, 21),
    (Yottameter, yottameters, 24),
);

define_time_quantity!(
    SITime,
    (Picosecond, picoseconds, -12, 0, -12),
    (Nanosecond, nanoseconds, -9, 0, -9),
    (Microsecond, microseconds, -6, 0, -6),
    (Millisecond, milliseconds, -3, 0, -3),
    (Second, seconds, 0, 0, 0),
    (Decasecond, decaseconds, 1, 0, 1),
    (Hectosecond, hectoseconds, 2, 0, 2),
    (Kilosecond, kiloseconds, 3, 0, 3),
    (Megasecond, megaseconds, 6, 0, 6),
    (Gigasecond, gigaseconds, 9, 0, 9),
    (Terasecond, teraseconds, 12, 0, 12),
    (Petasecond, petaseconds, 15, 0, 15),
    (Exasecond, exaseconds, 18, 0, 18),
    (Zettasecond, zettaseconds, 21, 0, 21),
    (Yottasecond, yottaseconds, 24, 0, 24),
);

define_time_quantity!(
    CommonTime,
    (Minute, minutes, 2, 1, 1),
    (Hour, hours, 4, 2, 2),
    (Day, days, 7, 3, 2),
);

define_current_quantity!(
    SICurrent,
    (Picoampere, picoamperes, -12),
    (Nanoampere, nanoamperes, -9),
    (Microampere, microamperes, -6),
    (Milliampere, milliamperes, -3),
    (Centiampere, centiamperes, -2),
    (Deciampere, deciamperes, -1),
    (Ampere, amperes, 0),
    (Decaampere, decaamperes, 1),
    (Hectoampere, hectoamperes, 2),
    (Kiloampere, kiloamperes, 3),
    (Megaampere, megaamperes, 6),
    (Gigaampere, gigaamperes, 9),
    (Teraampere, teraamperes, 12),
    (Petaampere, petaamperes, 15),
    (Exaampere, exaamperes, 18),
    (Zettaampere, zettaamperes, 21),
    (Yottaampere, yottaamperes, 24),
);

define_temperature_quantity!(
    SITemperature,
    (Picokelvin, picokelvins, -12),
    (Nanokelvin, nanokelvins, -9),
    (Microkelvin, microkelvins, -6),
    (Millikelvin, millikelvins, -3),
    (Centikelvin, centikelvins, -2),
    (Decikelvin, decikelvins, -1),
    (Kelvin, kelvins, 0),
    (Decakelvin, decakelvins, 1),
    (Hectokelvin, hectokelvins, 2),
    (Kilokelvin, kilokelvins, 3),
    (Megakelvin, megakelvins, 6),
    (Gigakelvin, gigakelvins, 9),
    (Terakelvin, terakelvins, 12),
    (Petakelvin, petakelvins, 15),
    (Exakelvin, exakelvins, 18),
    (Zettakelvin, zettakelvins, 21),
    (Yottakelvin, yottakelvins, 24),
);

define_amount_quantity!(
    SIAmount,
    (Picomole, picomoles, -12),
    (Nanomole, nanomoles, -9),
    (Micromole, micromoles, -6),
    (Millimole, millimoles, -3),
    (Centimole, centimoles, -2),
    (Decimole, decimoles, -1),
    (Mole, moles, 0),
    (Decamole, decamoles, 1),
    (Hectomole, hectomoles, 2),
    (Kilomole, kilomoles, 3),
    (Megamole, megamoles, 6),
    (Gigamole, gigamoles, 9),
    (Teramole, teramoles, 12),
    (Petamole, petamoles, 15),
    (Examole, examoles, 18),
    (Zettamole, zettamoles, 21),
    (Yottamole, yottamoles, 24),
);

define_luminosity_quantity!(
    SILuminosity,
    (Picocandela, picocandelas, -12),
    (Nanocandela, nanocandelas, -9),
    (Microcandela, microcandelas, -6),
    (Millicandela, millicandelas, -3),
    (Centicandela, centicandelas, -2),
    (Decicandela, decicandelas, -1),
    (Candela, candelas, 0),
    (Decacandela, decacandelas, 1),
    (Hectocandela, hectocandelas, 2),
    (Kilocandela, kilocandelas, 3),
    (Megacandela, megacandelas, 6),
    (Gigacandela, gigacandelas, 9),
    (Teracandela, teracandelas, 12),
    (Petacandela, petacandelas, 15),
    (Exacandela, exacandelas, 18),
    (Zettacandela, zettacandelas, 21),
    (Yottacandela, yottacandelas, 24),
);

define_angle_quantity!(
    SIAngle,
    (Picoradian, picoradians, -12, 0, -12, 0),
    (Nanoradian, nanoradians, -9, 0, -9, 0),
    (Microradian, microradians, -6, 0, -6, 0),
    (Milliradian, milliradians, -3, 0, -3, 0),
    (Centiradian, centiradians, -2, 0, -2, 0),
    (Deciradian, deciradians, -1, 0, -1, 0),
    (Radian, radians, 0, 0, 0, 0),
    (Decaradian, decaradians, 1, 0, 1, 0),
    (Hectoradian, hectoradians, 2, 0, 2, 0),
    (Kiloradian, kiloradians, 3, 0, 3, 0),
    (Megaradian, megaradians, 6, 0, 6, 0),
    (Gigaradian, gigaradians, 9, 0, 9, 0),
    (Teraradian, teraradians, 12, 0, 12, 0),
    (Petaradian, petaradians, 15, 0, 15, 0),
    (Exaradian, exaradians, 18, 0, 18, 0),
    (Zettaradian, zettaradians, 21, 0, 21, 0),
    (Yottaradian, yottaradians, 24, 0, 24, 0),
);

define_angle_quantity!(
    CommonAngle,
    (Turn, turns, 1, 0, 0, 1),
    (Degrees, degrees, -2, -2, -1, 1),
    (Gradians, gradians, -3, 0, -2, 1),
    (Arcminutes, arcminutes, -4, -3, -2, 1),
    (Arcseconds, arcseconds, -6, -4, -3, 1),
);

define_unit_macro!(
    0,
    0,
    0, 0, 0,
    0,
    0,
    0,
    0,
    0, 0, 0, 0,
);

pub use unit;