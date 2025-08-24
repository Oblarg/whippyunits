// Declarative unit macro that parses unit expressions
// This approach uses a simpler pattern matching approach

#[macro_export]
macro_rules! unit {
    // Single units
    (mm) => { crate::Quantity<1, -1, 0, {isize::MAX}, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    (m) => { crate::Quantity<1, 0, 0, {isize::MAX}, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    (km) => { crate::Quantity<1, 1, 0, {isize::MAX}, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    
    (mg) => { crate::Quantity<0, {isize::MAX}, 1, -1, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    (g) => { crate::Quantity<0, {isize::MAX}, 1, 0, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    (kg) => { crate::Quantity<0, {isize::MAX}, 1, 1, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    
    (ms) => { crate::Quantity<0, {isize::MAX}, 0, {isize::MAX}, 1, -3, 0, -3, -1> };
    (s) => { crate::Quantity<0, {isize::MAX}, 0, {isize::MAX}, 1, 0, 0, 0, 0> };
    (min) => { crate::Quantity<0, {isize::MAX}, 0, {isize::MAX}, 1, 2, 1, 1, 1> };
    
    // Units with exponents
    (mm^2) => { crate::Quantity<2, -1, 0, {isize::MAX}, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    (m^2) => { crate::Quantity<2, 0, 0, {isize::MAX}, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    (km^2) => { crate::Quantity<2, 1, 0, {isize::MAX}, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    
    (kg^2) => { crate::Quantity<0, {isize::MAX}, 2, 1, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    (s^2) => { crate::Quantity<0, {isize::MAX}, 0, {isize::MAX}, 2, 0, 0, 0, 0> };
    
    // Common compound units
    (m * s) => { crate::Quantity<1, 0, 0, {isize::MAX}, 1, 0, 0, 0, 0> };
    (mm * s) => { crate::Quantity<1, -1, 0, {isize::MAX}, 1, 0, 0, 0, 0> };
    (m^3) => { crate::Quantity<3, 0, 0, {isize::MAX}, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    
    // Force units (kg * m * s^-2)
    (kg * m * s^-2) => { crate::Quantity<1, 0, 1, 1, -2, 0, 0, 0, 0> };
    
    // Energy units (kg * m^2 * s^-2)
    (kg * m^2 * s^-2) => { crate::Quantity<2, 0, 1, 1, -2, 0, 0, 0, 0> };
    
    // Power units (kg * m^2 * s^-3)
    (kg * m^2 * s^-3) => { crate::Quantity<2, 0, 1, 1, -3, 0, 0, 0, 0> };
    
    // Velocity units (m * s^-1)
    (m * s^-1) => { crate::Quantity<1, 0, 0, {isize::MAX}, -1, 0, 0, 0, 0> };
    (m/s) => { crate::Quantity<1, 0, 0, {isize::MAX}, -1, 0, 0, 0, 0> };
    
    // Acceleration units (m * s^-2)
    (m * s^-2) => { crate::Quantity<1, 0, 0, {isize::MAX}, -2, 0, 0, 0, 0> };
    (m/s^2) => { crate::Quantity<1, 0, 0, {isize::MAX}, -2, 0, 0, 0, 0> };
    
    // Area units
    (mm^2) => { crate::Quantity<2, -1, 0, {isize::MAX}, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    
    // Volume units
    (mm^3) => { crate::Quantity<3, -1, 0, {isize::MAX}, 0, {isize::MAX}, {isize::MAX}, {isize::MAX}, {isize::MAX}> };
    
    // Catch-all for unknown units
    ($unknown:tt) => {
        compile_error!(concat!("Unknown unit: ", stringify!($unknown), ". Use proc_unit!() for complex expressions."))
    };
}
