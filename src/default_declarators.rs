use crate::quantity_type::Quantity;
use crate::define_generic_dimension;

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
        
        // Generate extension trait implementations for f64 (default)
        impl $trait_name<f64> for f64 {
            $(
                fn $fn_name(self) -> $scale_name<f64> {
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

define_quantity!(
    1, 0, 0, 0, 0, 0, 0, 0,
    SIMass,
    (Quectogram, quectograms, -33, 0, -33, 0),  // -3 (gram base) + -30 (quecto)
    (Rontogram, rontograms, -30, 0, -30, 0),    // -3 (gram base) + -27 (ronto)
    (Yoctogram, yoctograms, -27, 0, -27, 0),    // -3 (gram base) + -24 (yocto)
    (Zeptogram, zeptograms, -24, 0, -24, 0),    // -3 (gram base) + -21 (zepto)
    (Attogram, attograms, -21, 0, -21, 0),      // -3 (gram base) + -18 (atto)
    (Femtogram, femtograms, -18, 0, -18, 0),    // -3 (gram base) + -15 (femto)
    (Picogram, picograms, -15, 0, -15, 0),      // -3 (gram base) + -12 (pico)
    (Nanogram, nanograms, -12, 0, -12, 0),      // -3 (gram base) + -9 (nano)
    (Microgram, micrograms, -9, 0, -9, 0),      // -3 (gram base) + -6 (micro)
    (Milligram, milligrams, -6, 0, -6, 0),      // -3 (gram base) + -3 (milli)
    (Centigram, centigrams, -5, 0, -5, 0),      // -3 (gram base) + -2 (centi)
    (Decigram, decigrams, -4, 0, -4, 0),        // -3 (gram base) + -1 (deci)
    (Gram, grams, -3, 0, -3, 0),                // -3 (gram base) + 0 (no prefix)
    (Decagram, decagrams, -2, 0, -2, 0),        // -3 (gram base) + 1 (deca)
    (Hectogram, hectograms, -1, 0, -1, 0),      // -3 (gram base) + 2 (hecto)
    (Kilogram, kilograms, 0, 0, 0, 0),          // -3 (gram base) + 3 (kilo)
    (Megagram, megagrams, 3, 0, 3, 0),          // -3 (gram base) + 6 (mega)
    (Gigagram, gigagrams, 6, 0, 6, 0),          // -3 (gram base) + 9 (giga)
    (Teragram, teragrams, 9, 0, 9, 0),          // -3 (gram base) + 12 (tera)
    (Petagram, petagrams, 12, 0, 12, 0),        // -3 (gram base) + 15 (peta)
    (Exagram, exagrams, 15, 0, 15, 0),          // -3 (gram base) + 18 (exa)
    (Zettagram, zettagrams, 18, 0, 18, 0),      // -3 (gram base) + 21 (zetta)
    (Yottagram, yottagrams, 21, 0, 21, 0),      // -3 (gram base) + 24 (yotta)
    (Ronagram, ronagrams, 24, 0, 24, 0),        // -3 (gram base) + 27 (ronna)
    (Quettagram, quettagrams, 27, 0, 27, 0)     // -3 (gram base) + 30 (quetta)
);

define_quantity!(
    0, 1, 0, 0, 0, 0, 0, 0,
    SILength,
    (Quectometer, quectometers, -30, 0, -30, 0),
    (Rontometer, rontometers, -27, 0, -27, 0),
    (Yoctometer, yoctometers, -24, 0, -24, 0),
    (Zeptometer, zeptometers, -21, 0, -21, 0),
    (Attometer, attometers, -18, 0, -18, 0),
    (Femtometer, femtometers, -15, 0, -15, 0),
    (Picometer, picometers, -12, 0, -12, 0),
    (Nanometer, nanometers, -9, 0, -9, 0),
    (Micrometer, micrometers, -6, 0, -6, 0),
    (Millimeter, millimeters, -3, 0, -3, 0),
    (Centimeter, centimeters, -2, 0, -2, 0),
    (Decimeter, decimeters, -1, 0, -1, 0),
    (Meter, meters, 0, 0, 0, 0),
    (Decameter, decameters, 1, 0, 1, 0),
    (Hectometer, hectometers, 2, 0, 2, 0),
    (Kilometer, kilometers, 3, 0, 3, 0),
    (Megameter, megameters, 6, 0, 6, 0),
    (Gigameter, gigameters, 9, 0, 9, 0),
    (Terameter, terameters, 12, 0, 12, 0),
    (Petameter, petameters, 15, 0, 15, 0),
    (Exameter, exameters, 18, 0, 18, 0),
    (Zettameter, zettameters, 21, 0, 21, 0),
    (Yottameter, yottameters, 24, 0, 24, 0),
    (Ronameter, ronameters, 27, 0, 27, 0),
    (Quettameter, quettameters, 30, 0, 30, 0),
);

define_quantity!(
    0, 0, 1, 0, 0, 0, 0, 0,
    SITime,
    (Quectosecond, quectoseconds, -30, 0, -30, 0),
    (Rontosecond, rontoseconds, -27, 0, -27, 0),
    (Yoctosecond, yoctoseconds, -24, 0, -24, 0),
    (Zeptosecond, zeptoseconds, -21, 0, -21, 0),
    (Attosecond, attoseconds, -18, 0, -18, 0),
    (Femtosecond, femtoseconds, -15, 0, -15, 0),
    (Picosecond, picoseconds, -12, 0, -12, 0),
    (Nanosecond, nanoseconds, -9, 0, -9, 0),
    (Microsecond, microseconds, -6, 0, -6, 0),
    (Millisecond, milliseconds, -3, 0, -3, 0),
    (Centisecond, centiseconds, -2, 0, -2, 0),
    (Decisecond, deciseconds, -1, 0, -1, 0),
    (Second, seconds, 0, 0, 0, 0),
    (Decasecond, decaseconds, 1, 0, 1, 0),
    (Hectosecond, hectoseconds, 2, 0, 2, 0),
    (Kilosecond, kiloseconds, 3, 0, 3, 0),
    (Megasecond, megaseconds, 6, 0, 6, 0),
    (Gigasecond, gigaseconds, 9, 0, 9, 0),
    (Terasecond, teraseconds, 12, 0, 12, 0),
    (Petasecond, petaseconds, 15, 0, 15, 0),
    (Exasecond, exaseconds, 18, 0, 18, 0),
    (Zettasecond, zettaseconds, 21, 0, 21, 0),
    (Yottasecond, yottaseconds, 24, 0, 24, 0),
    (Ronasecond, ronaseconds, 27, 0, 27, 0),
    (Quettasecond, quettaseconds, 30, 0, 30, 0),
);

define_quantity!(
    0, 0, 1, 0, 0, 0, 0, 0,
    CommonTime,
    (Minute, minutes, 2, 1, 1, 0),
    (Hour, hours, 4, 2, 2, 0),
    (Day, days, 7, 3, 2, 0),
);

define_quantity!(
    0, 0, 0, 1, 0, 0, 0, 0,
    SICurrent,
    (Quectoampere, quectoamperes, -30, 0, -30, 0),
    (Rontoampere, rontoamperes, -27, 0, -27, 0),
    (Yoctoampere, yoctoamperes, -24, 0, -24, 0),
    (Zeptoampere, zeptoamperes, -21, 0, -21, 0),
    (Attoampere, attoamperes, -18, 0, -18, 0),
    (Femtoampere, femtoamperes, -15, 0, -15, 0),
    (Picoampere, picoamperes, -12, 0, -12, 0),
    (Nanoampere, nanoamperes, -9, 0, -9, 0),
    (Microampere, microamperes, -6, 0, -6, 0),
    (Milliampere, milliamperes, -3, 0, -3, 0),
    (Centiampere, centiamperes, -2, 0, -2, 0),
    (Deciampere, deciamperes, -1, 0, -1, 0),
    (Ampere, amperes, 0, 0, 0, 0),
    (Decaampere, decaamperes, 1, 0, 1, 0),
    (Hectoampere, hectoamperes, 2, 0, 2, 0),
    (Kiloampere, kiloamperes, 3, 0, 3, 0),
    (Megaampere, megaamperes, 6, 0, 6, 0),
    (Gigaampere, gigaamperes, 9, 0, 9, 0),
    (Teraampere, teraamperes, 12, 0, 12, 0),
    (Petaampere, petaamperes, 15, 0, 15, 0),
    (Exaampere, exaamperes, 18, 0, 18, 0),
    (Zettaampere, zettaamperes, 21, 0, 21, 0),
    (Yottaampere, yottaamperes, 24, 0, 24, 0),
    (Ronaampere, ronaamperes, 27, 0, 27, 0),
    (Quettaampere, quettaamperes, 30, 0, 30, 0),
);

define_quantity!(
    0, 0, 0, 0, 1, 0, 0, 0,
    SITemperature,
    (Quectokelvin, quectokelvins, -30, 0, -30, 0),
    (Rontokelvin, rontokelvins, -27, 0, -27, 0),
    (Yoctokelvin, yoctokelvins, -24, 0, -24, 0),
    (Zeptokelvin, zeptokelvins, -21, 0, -21, 0),
    (Attokelvin, attokelvins, -18, 0, -18, 0),
    (Femtokelvin, femtokelvins, -15, 0, -15, 0),
    (Picokelvin, picokelvins, -12, 0, -12, 0),
    (Nanokelvin, nanokelvins, -9, 0, -9, 0),
    (Microkelvin, microkelvins, -6, 0, -6, 0),
    (Millikelvin, millikelvins, -3, 0, -3, 0),
    (Centikelvin, centikelvins, -2, 0, -2, 0),
    (Decikelvin, decikelvins, -1, 0, -1, 0),
    (Kelvin, kelvins, 0, 0, 0, 0),
    (Decakelvin, decakelvins, 1, 0, 1, 0),
    (Hectokelvin, hectokelvins, 2, 0, 2, 0),
    (Kilokelvin, kilokelvins, 3, 0, 3, 0),
    (Megakelvin, megakelvins, 6, 0, 6, 0),
    (Gigakelvin, gigakelvins, 9, 0, 9, 0),
    (Terakelvin, terakelvins, 12, 0, 12, 0),
    (Petakelvin, petakelvins, 15, 0, 15, 0),
    (Exakelvin, exakelvins, 18, 0, 18, 0),
    (Zettakelvin, zettakelvins, 21, 0, 21, 0),
    (Yottakelvin, yottakelvins, 24, 0, 24, 0),
    (Ronakelvin, ronakelvins, 27, 0, 27, 0),
    (Quettakelvin, quettakelvins, 30, 0, 30, 0),
);

define_affine_quantity!(
    0, 0, 0, 0, 1, 0, 0, 0,  // temperature dimension
    CommonTemperature,
    Kelvin,
    (Celsius, celsius, 273.15),  // °C to K: C + 273.15
);

define_quantity!(
    0, 0, 0, 0, 0, 1, 0, 0,
    SIAmount,
    (Quectomole, quectomoles, -30, 0, -30, 0),
    (Rontomole, rontomoles, -27, 0, -27, 0),
    (Yoctomole, yoctomoles, -24, 0, -24, 0),
    (Zeptomole, zeptomoles, -21, 0, -21, 0),
    (Attomole, attomoles, -18, 0, -18, 0),
    (Femtomole, femtomoles, -15, 0, -15, 0),
    (Picomole, picomoles, -12, 0, -12, 0),
    (Nanomole, nanomoles, -9, 0, -9, 0),
    (Micromole, micromoles, -6, 0, -6, 0),
    (Millimole, millimoles, -3, 0, -3, 0),
    (Centimole, centimoles, -2, 0, -2, 0),
    (Decimole, decimoles, -1, 0, -1, 0),
    (Mole, moles, 0, 0, 0, 0),
    (Decamole, decamoles, 1, 0, 1, 0),
    (Hectomole, hectomoles, 2, 0, 2, 0),
    (Kilomole, kilomoles, 3, 0, 3, 0),
    (Megamole, megamoles, 6, 0, 6, 0),
    (Gigamole, gigamoles, 9, 0, 9, 0),
    (Teramole, teramoles, 12, 0, 12, 0),
    (Petamole, petamoles, 15, 0, 15, 0),
    (Examole, examoles, 18, 0, 18, 0),
    (Zettamole, zettamoles, 21, 0, 21, 0),
    (Yottamole, yottamoles, 24, 0, 24, 0),
    (Ronamole, ronamoles, 27, 0, 27, 0),
    (Quettamole, quettamoles, 30, 0, 30, 0),
);

define_quantity!(
    0, 0, 0, 0, 0, 0, 1, 0,
    SILuminosity,
    (Quectocandela, quectocandelas, -30, 0, -30, 0),
    (Rontocandela, rontocandelas, -27, 0, -27, 0),
    (Yoctocandela, yoctocandelas, -24, 0, -24, 0),
    (Zeptocandela, zeptocandelas, -21, 0, -21, 0),
    (Attocandela, attocandelas, -18, 0, -18, 0),
    (Femtocandela, femtocandelas, -15, 0, -15, 0),
    (Picocandela, picocandelas, -12, 0, -12, 0),
    (Nanocandela, nanocandelas, -9, 0, -9, 0),
    (Microcandela, microcandelas, -6, 0, -6, 0),
    (Millicandela, millicandelas, -3, 0, -3, 0),
    (Centicandela, centicandelas, -2, 0, -2, 0),
    (Decicandela, decicandelas, -1, 0, -1, 0),
    (Candela, candelas, 0, 0, 0, 0),
    (Decacandela, decacandelas, 1, 0, 1, 0),
    (Hectocandela, hectocandelas, 2, 0, 2, 0),
    (Kilocandela, kilocandelas, 3, 0, 3, 0),
    (Megacandela, megacandelas, 6, 0, 6, 0),
    (Gigacandela, gigacandelas, 9, 0, 9, 0),
    (Teracandela, teracandelas, 12, 0, 12, 0),
    (Petacandela, petacandelas, 15, 0, 15, 0),
    (Exacandela, exacandelas, 18, 0, 18, 0),
    (Zettacandela, zettacandelas, 21, 0, 21, 0),
    (Yottacandela, yottacandelas, 24, 0, 24, 0),
    (Ronacandela, ronacandelas, 27, 0, 27, 0),
    (Quettacandela, quettacandelas, 30, 0, 30, 0),
);

define_quantity!(
    0, 0, 0, 0, 0, 0, 0, 1,
    SIAngle,
    (Quectoradian, quectoradians, -30, 0, -30, 0),
    (Rontoradian, rontoradians, -27, 0, -27, 0),
    (Yoctoradian, yoctoradians, -24, 0, -24, 0),
    (Zeptoradian, zeptoradians, -21, 0, -21, 0),
    (Attoradian, attoradians, -18, 0, -18, 0),
    (Femtoradian, femtoradians, -15, 0, -15, 0),
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
    (Ronaradian, ronaradians, 27, 0, 27, 0),
    (Quettaradian, quettaradians, 30, 0, 30, 0),
);

define_quantity!(
    0, 0, 0, 0, 0, 0, 0, 1,
    CommonAngle,
    (Turn, turns, 1, 0, 0, 1),
    (Degrees, degrees, -2, -2, -1, 1),
    (Gradians, gradians, -3, 0, -2, 1),
    (Arcminutes, arcminutes, -4, -3, -2, 1),
    (Arcseconds, arcseconds, -6, -4, -3, 1),
);

#[macro_export]
macro_rules! quantity {
    ($value:expr, $unit:expr) => {
        <$crate::unit!($unit)>::new($value)
    };
    ($value:expr, $unit:expr, $storage_type:ty) => {
        <$crate::unit!($unit, $storage_type)>::new($value)
    };
}