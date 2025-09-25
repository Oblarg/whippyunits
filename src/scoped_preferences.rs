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

#[macro_export]
macro_rules! define_base_units {
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
        // Generate custom literals within this scope
        $crate::define_default_literals!();

        $crate::define_local_quantity!(
            $mass_scale,
            LocalMass,
            (Quectogram, quectograms),
            (Rontogram, rontograms),
            (Yoctogram, yoctograms),
            (Zeptogram, zeptograms),
            (Attogram, attograms),
            (Femtogram, femtograms),
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
            (Ronagram, ronagrams),
            (Quettagram, quettagrams),
        );

        $crate::define_local_quantity!(
            $length_scale,
            LocalLength,
            (Quectometer, quectometers),
            (Rontometer, rontometers),
            (Yoctometer, yoctometers),
            (Zeptometer, zeptometers),
            (Attometer, attometers),
            (Femtometer, femtometers),
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
            (Ronameter, ronameters),
            (Quettameter, quettameters),
        );

        $crate::define_local_quantity!(
            $time_scale,
            LocalTime,
            (Quectosecond, quectoseconds),
            (Rontosecond, rontoseconds),
            (Yoctosecond, yoctoseconds),
            (Zeptosecond, zeptoseconds),
            (Attosecond, attoseconds),
            (Femtosecond, femtoseconds),
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
            (Ronasecond, ronaseconds),
            (Quettasecond, quettaseconds),
            (Minute, minutes),
            (Hour, hours),
            (Day, days),
        );

        $crate::define_local_quantity!(
            $current_scale,
            LocalCurrent,
            (Quectoampere, quectoamperes),
            (Rontoampere, rontoamperes),
            (Yoctoampere, yoctoamperes),
            (Zeptoampere, zeptoamperes),
            (Attoampere, attoamperes),
            (Femtoampere, femtoamperes),
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
            (Ronaampere, ronaamperes),
            (Quettaampere, quettaamperes),
        );

        $crate::define_local_quantity!(
            $amount_scale,
            LocalAmount,
            (Quectomole, quectomoles),
            (Rontomole, rontomoles),
            (Yoctomole, yoctomoles),
            (Zeptomole, zeptomoles),
            (Attomole, attomoles),
            (Femtomole, femtomoles),
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
            (Ronamole, ronamoles),
            (Quettamole, quettamoles),
        );

        $crate::define_local_quantity!(
            $luminosity_scale,
            LocalLuminosity,
            (Quectocandela, quectocandelas),
            (Rontocandela, rontocandelas),
            (Yoctocandela, yoctocandelas),
            (Zeptocandela, zeptocandelas),
            (Attocandela, attocandelas),
            (Femtocandela, femtocandelas),
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
            (Ronacandela, ronacandelas),
            (Quettacandela, quettacandelas),
        );

        $crate::define_local_quantity!(
            $angle_scale,
            LocalAngle,
            (Quectoradian, quectoradians),
            (Rontoradian, rontoradians),
            (Yoctoradian, yoctoradians),
            (Zeptoradian, zeptoradians),
            (Attoradian, attoradians),
            (Femtoradian, femtoradians),
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
            (Ronaradian, ronaradians),
            (Quettaradian, quettaradians),
            (Turn, turns),
            (Degrees, degrees),
            (Gradians, gradians),
            (Arcminutes, arcminutes),
            (Arcseconds, arcseconds),
        );

        // Helper macro to construct the target type using local scale parameters
        // This uses a procedural macro to map the dimension to the appropriate local scale type
        #[macro_export]
        macro_rules! local_unit_type {
            ($unit:expr) => {
                $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale)
            };
            ($unit:expr, $storage_type:ty) => {
                $crate::local_unit_type!($unit, $mass_scale, $length_scale, $time_scale, $current_scale, $temperature_scale, $amount_scale, $luminosity_scale, $angle_scale, $storage_type)
            };
        }

        // Define a local quantity! macro that delegates to the default quantity! macro
        // and then rescales to the preferred units using the existing machinery
        #[macro_export]
        macro_rules! quantity {
            ($value:expr, $unit:expr) => {
                {
                    // Create the quantity using the default quantity! macro (source type)
                    let default_quantity = $crate::quantity!($value, $unit);
                    
                    
                    let target_quantity: local_unit_type!($unit) = $crate::api::rescale_f64(default_quantity);
                    target_quantity
                }
            };
            ($value:expr, $unit:expr, f64) => {
                {
                    // Create the quantity using the default quantity! macro with f64 storage type
                    let default_quantity = $crate::quantity!($value, $unit, f64);
                    
                    let target_quantity: local_unit_type!($unit, f64) = $crate::api::rescale_f64(default_quantity);
                    target_quantity
                }
            };
            ($value:expr, $unit:expr, i32) => {
                {
                    // Create the quantity using the default quantity! macro with i32 storage type
                    let default_quantity = $crate::quantity!($value, $unit, i32);
                    
                    let target_quantity: local_unit_type!($unit, i32) = $crate::api::rescale_i32(default_quantity);
                    target_quantity
                }
            };
            ($value:expr, $unit:expr, i64) => {
                {
                    // Create the quantity using the default quantity! macro with i64 storage type
                    let default_quantity = $crate::quantity!($value, $unit, i64);
                    
                    let target_quantity: local_unit_type!($unit, i64) = $crate::api::rescale_i64(default_quantity);
                    target_quantity
                }
            };
        }
    };
}
