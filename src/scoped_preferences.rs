use crate::default_declarators;
use crate::quantity_type::Quantity;

#[macro_export]
macro_rules! define_local_quantity {
    (
        $local_quantity_scale:ident,
        $trait_name:ident, $(($scale_name:ident, $fn_name:ident)),* $(,)?
    ) => {
        // Generate the trait definition
        pub trait $trait_name {
            $(
                fn $fn_name(self) -> $crate::default_declarators::$local_quantity_scale;
            )*
        }

        // Generate extension trait implementations for f64
        impl $trait_name for f64 {
            $(
                fn $fn_name(self) -> $crate::default_declarators::$local_quantity_scale {
                    rescale_f64($crate::default_declarators::$scale_name::new(self))
                }
            )*
        }
    };
}

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
        $crate::define_local_quantity!(
            $mass_scale,
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

        $crate::define_local_quantity!(
            $length_scale,
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

        $crate::define_local_quantity!(
            $time_scale,
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

        $crate::define_local_quantity!(
            $current_scale,
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

        $crate::define_local_quantity!(
            $amount_scale,
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

        $crate::define_local_quantity!(
            $luminosity_scale,
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

        $crate::define_local_quantity!(
            $angle_scale,
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
    };
}
