//! Default dimension data for whippyunits
//! 
//! This crate provides canonical dimension data that can be shared between
//! the main whippyunits library and the proc macro crate without circular dependencies.

/// Dimension exponents tuple: (mass, length, time, current, temperature, amount, luminosity, angle)
pub type DimensionExponents = (i16, i16, i16, i16, i16, i16, i16, i16);

/// Dimension information including name and optional SI unit symbols
#[derive(Debug, Clone)]
pub struct DimensionInfo {
    pub exponents: DimensionExponents,
    pub name: &'static str,
    pub si_symbol: Option<&'static str>,
    pub si_long_name: Option<&'static str>,
}

/// Canonical lookup table for all supported dimensions
/// 
/// This is the single source of truth for dimension data, shared between
/// the prettyprint logic and the proc macro DSL.
pub const DIMENSION_LOOKUP: &[DimensionInfo] = &[
    // Atomic dimensions (SI base quantities)
    DimensionInfo {
        exponents: (1, 0, 0, 0, 0, 0, 0, 0),
        name: "Mass",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 1, 0, 0, 0, 0, 0, 0),
        name: "Length",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 1, 0, 0, 0, 0, 0),
        name: "Time",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 0, 1, 0, 0, 0, 0),
        name: "Current",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 0, 0, 1, 0, 0, 0),
        name: "Temperature",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 0, 0, 0, 1, 0, 0),
        name: "Amount",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 0, 0, 0, 0, 1, 0),
        name: "Luminosity",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, 0, 0, 0, 0, 0, 1),
        name: "Angle",
        si_symbol: None,
        si_long_name: None,
    },
    
    // Length-like dimensions
    DimensionInfo {
        exponents: (0, 2, 0, 0, 0, 0, 0, 0),
        name: "Area",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 3, 0, 0, 0, 0, 0, 0),
        name: "Volume",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, -1, 0, 0, 0, 0, 0, 0),
        name: "Wave Number",
        si_symbol: None,
        si_long_name: None,
    },
    
    // Time-like dimensions
    DimensionInfo {
        exponents: (0, 0, -1, 0, 0, 0, 0, 0),
        name: "Frequency",
        si_symbol: Some("Hz"),
        si_long_name: Some("Hertz"),
    },
    
    // Velocity-like dimensions
    DimensionInfo {
        exponents: (0, 1, -1, 0, 0, 0, 0, 0),
        name: "Velocity",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 1, -2, 0, 0, 0, 0, 0),
        name: "Acceleration",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 1, -3, 0, 0, 0, 0, 0),
        name: "Jerk",
        si_symbol: None,
        si_long_name: None,
    },
    
    // Force-like dimensions
    DimensionInfo {
        exponents: (1, 1, -1, 0, 0, 0, 0, 0),
        name: "Momentum",
        si_symbol: Some("N⋅s"),
        si_long_name: Some("Newton Second"),
    },
    DimensionInfo {
        exponents: (1, 1, -2, 0, 0, 0, 0, 0),
        name: "Force",
        si_symbol: Some("N"),
        si_long_name: Some("Newton"),
    },
    DimensionInfo {
        exponents: (1, 2, -2, 0, 0, 0, 0, 0),
        name: "Energy",
        si_symbol: Some("J"),
        si_long_name: Some("Joule"),
    },
    DimensionInfo {
        exponents: (1, 2, -3, 0, 0, 0, 0, 0),
        name: "Power",
        si_symbol: Some("W"),
        si_long_name: Some("Watt"),
    },
    DimensionInfo {
        exponents: (1, 2, -1, 0, 0, 0, 0, 0),
        name: "Action",
        si_symbol: Some("J⋅s"),
        si_long_name: Some("Joule Second"),
    },
    DimensionInfo {
        exponents: (1, -1, -2, 0, 0, 0, 0, 0),
        name: "Pressure",
        si_symbol: Some("Pa"),
        si_long_name: Some("Pascal"),
    },
    
    // Density-like dimensions
    DimensionInfo {
        exponents: (1, -1, 0, 0, 0, 0, 0, 0),
        name: "Linear Mass Density",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (1, -2, 0, 0, 0, 0, 0, 0),
        name: "Surface Mass Density",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (1, -3, 0, 0, 0, 0, 0, 0),
        name: "Volume Mass Density",
        si_symbol: None,
        si_long_name: None,
    },
    
    // Viscosity-like dimensions
    DimensionInfo {
        exponents: (1, -1, -1, 0, 0, 0, 0, 0),
        name: "Viscosity",
        si_symbol: Some("Pa⋅s"),
        si_long_name: Some("Pascal Second"),
    },
    DimensionInfo {
        exponents: (0, 2, -1, 0, 0, 0, 0, 0),
        name: "Kinematic Viscosity",
        si_symbol: Some("St"),
        si_long_name: Some("Stokes"),
    },
    
    // Surface tension-like dimensions
    DimensionInfo {
        exponents: (1, 0, -2, 0, 0, 0, 0, 0),
        name: "Surface Tension",
        si_symbol: Some("N/m"),
        si_long_name: Some("Newton per Meter"),
    },
    
    // Specific energy-like dimensions
    DimensionInfo {
        exponents: (0, 2, -2, 0, 0, 0, 0, 0),
        name: "Specific Energy",
        si_symbol: Some("J/kg"),
        si_long_name: Some("Joule per Kilogram"),
    },
    DimensionInfo {
        exponents: (0, 2, -3, 0, 0, 0, 0, 0),
        name: "Specific Power",
        si_symbol: Some("W/kg"),
        si_long_name: Some("Watt per Kilogram"),
    },
    
    // Flow rate-like dimensions
    DimensionInfo {
        exponents: (1, 0, -1, 0, 0, 0, 0, 0),
        name: "Mass Flow Rate",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 3, -1, 0, 0, 0, 0, 0),
        name: "Volume Flow Rate",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (1, -1, -3, 0, 0, 0, 0, 0),
        name: "Power Density",
        si_symbol: Some("W/m"),
        si_long_name: Some("Watt per Meter"),
    },
    DimensionInfo {
        exponents: (1, -2, -2, 0, 0, 0, 0, 0),
        name: "Force Density",
        si_symbol: Some("N/m²"),
        si_long_name: Some("Newton per Square Meter"),
    },
    DimensionInfo {
        exponents: (1, 0, -3, 0, 0, 0, 0, 0),
        name: "Heat Flux",
        si_symbol: Some("W/m²"),
        si_long_name: Some("Watt per Square Meter"),
    },
    
    // Electrical dimensions
    DimensionInfo {
        exponents: (0, 0, 1, 1, 0, 0, 0, 0),
        name: "Electric Charge",
        si_symbol: Some("C"),
        si_long_name: Some("Coulomb"),
    },
    DimensionInfo {
        exponents: (1, 2, -3, -1, 0, 0, 0, 0),
        name: "Electric Potential",
        si_symbol: Some("V"),
        si_long_name: Some("Volt"),
    },
    DimensionInfo {
        exponents: (1, 2, -3, -2, 0, 0, 0, 0),
        name: "Electric Resistance",
        si_symbol: Some("Ω"),
        si_long_name: Some("Ohm"),
    },
    DimensionInfo {
        exponents: (-1, -2, 3, 2, 0, 0, 0, 0),
        name: "Electric Conductance",
        si_symbol: Some("S"),
        si_long_name: Some("Siemens"),
    },
    DimensionInfo {
        exponents: (-1, -2, 4, 2, 0, 0, 0, 0),
        name: "Capacitance",
        si_symbol: Some("F"),
        si_long_name: Some("Farad"),
    },
    DimensionInfo {
        exponents: (1, 2, -2, -2, 0, 0, 0, 0),
        name: "Inductance",
        si_symbol: Some("H"),
        si_long_name: Some("Henry"),
    },
    DimensionInfo {
        exponents: (1, 1, -3, -1, 0, 0, 0, 0),
        name: "Electric Field",
        si_symbol: Some("V/m"),
        si_long_name: Some("Volt per Meter"),
    },
    DimensionInfo {
        exponents: (1, 0, -2, -1, 0, 0, 0, 0),
        name: "Magnetic Field",
        si_symbol: Some("T"),
        si_long_name: Some("Tesla"),
    },
    DimensionInfo {
        exponents: (1, 2, -2, -1, 0, 0, 0, 0),
        name: "Magnetic Flux",
        si_symbol: Some("Wb"),
        si_long_name: Some("Weber"),
    },
    DimensionInfo {
        exponents: (0, -1, 1, 1, 0, 0, 0, 0),
        name: "Linear Charge Density",
        si_symbol: Some("C/m"),
        si_long_name: Some("Coulomb per Meter"),
    },
    DimensionInfo {
        exponents: (0, -2, 1, 1, 0, 0, 0, 0),
        name: "Surface Charge Density",
        si_symbol: Some("C/m²"),
        si_long_name: Some("Coulomb per Square Meter"),
    },
    DimensionInfo {
        exponents: (0, -3, 1, 1, 0, 0, 0, 0),
        name: "Volume Charge Density",
        si_symbol: Some("C/m³"),
        si_long_name: Some("Coulomb per Cubic Meter"),
    },
    DimensionInfo {
        exponents: (0, -1, 0, 1, 0, 0, 0, 0),
        name: "Magnetizing Field",
        si_symbol: Some("A/m"),
        si_long_name: Some("Ampere per Meter"),
    },
    
    // Thermodynamic dimensions
    DimensionInfo {
        exponents: (1, 2, -2, 0, -1, 0, 0, 0),
        name: "Entropy",
        si_symbol: Some("J/K"),
        si_long_name: Some("Joule per Kelvin"),
    },
    DimensionInfo {
        exponents: (0, 2, -2, 0, -1, 0, 0, 0),
        name: "Specific Heat Capacity",
        si_symbol: Some("J/(kg⋅K)"),
        si_long_name: Some("Joule per Kilogram Kelvin"),
    },
    DimensionInfo {
        exponents: (1, 2, -2, 0, -1, -1, 0, 0),
        name: "Molar Heat Capacity",
        si_symbol: Some("J/(mol⋅K)"),
        si_long_name: Some("Joule per Mole Kelvin"),
    },
    DimensionInfo {
        exponents: (1, 1, -3, 0, -1, 0, 0, 0),
        name: "Thermal Conductivity",
        si_symbol: Some("W/(m⋅K)"),
        si_long_name: Some("Watt per Meter Kelvin"),
    },
    DimensionInfo {
        exponents: (-1, -2, 3, 0, 1, 0, 0, 0),
        name: "Thermal Resistance",
        si_symbol: Some("K/W"),
        si_long_name: Some("Kelvin per Watt"),
    },
    DimensionInfo {
        exponents: (0, 0, 0, 0, -1, 0, 0, 0),
        name: "Thermal Expansion",
        si_symbol: None,
        si_long_name: None,
    },
    
    // Chemical dimensions
    DimensionInfo {
        exponents: (1, 0, 0, 0, 0, -1, 0, 0),
        name: "Molar Mass",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 3, 0, 0, 0, -1, 0, 0),
        name: "Molar Volume",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, -3, 0, 0, 0, 1, 0, 0),
        name: "Molar Concentration",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (-1, 0, 0, 0, 0, 1, 0, 0),
        name: "Molal Concentration",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, 0, -1, 0, 0, 1, 0, 0),
        name: "Molar Flow Rate",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (0, -2, -1, 0, 0, 1, 0, 0),
        name: "Molar Flux",
        si_symbol: None,
        si_long_name: None,
    },
    DimensionInfo {
        exponents: (1, 2, -2, 0, 0, -1, 0, 0),
        name: "Molar Energy",
        si_symbol: Some("J/mol"),
        si_long_name: Some("Joule per Mole"),
    },
    
    // Photometric dimensions
    DimensionInfo {
        exponents: (0, 0, 0, 0, 0, 0, 1, 0),
        name: "Luminous Flux",
        si_symbol: Some("lm"),
        si_long_name: Some("Lumen"),
    },
    DimensionInfo {
        exponents: (0, -2, 0, 0, 0, 0, 1, 0),
        name: "Illuminance",
        si_symbol: Some("lx"),
        si_long_name: Some("Lux"),
    },
    DimensionInfo {
        exponents: (0, -2, 1, 0, 0, 0, 1, 0),
        name: "Luminous Exposure",
        si_symbol: Some("lx⋅s"),
        si_long_name: Some("Lux Second"),
    },
    DimensionInfo {
        exponents: (-1, -2, 3, 0, 0, 0, 1, 0),
        name: "Luminous Efficacy",
        si_symbol: Some("lm/W"),
        si_long_name: Some("Lumen per Watt"),
    },
];

/// Look up dimension information by name (case-insensitive)
/// 
/// Returns the dimension info if found, or None if the dimension name is not recognized.
/// The search is case-insensitive and supports various naming conventions:
/// - Canonical names: "Electric Charge"
/// - Underscore variants: "electric_charge"
/// - No space variants: "electriccharge"
/// - UpperCamelCase: "ElectricCharge"
pub fn lookup_dimension_by_name(name: &str) -> Option<&'static DimensionInfo> {
    let name_lower = name.to_lowercase();
    let name_no_spaces = name_lower.replace(' ', "");
    
    DIMENSION_LOOKUP.iter().find(|info| {
        let info_name_lower = info.name.to_lowercase();
        
        // Direct match
        if info_name_lower == name_lower {
            return true;
        }
        
        // Match with spaces removed (for UpperCamelCase support)
        let info_name_no_spaces = info_name_lower.replace(' ', "");
        if info_name_no_spaces == name_no_spaces {
            return true;
        }
        
        // Handle common naming variations
        match info_name_lower.as_str() {
            "volume mass density" => {
                name_lower == "density" || name_lower == "volume_mass_density" || name_lower == "volumemassdensity"
            },
            "linear mass density" => {
                name_lower == "linear_mass_density" || name_lower == "linearmassdensity"
            },
            "surface mass density" => {
                name_lower == "surface_mass_density" || name_lower == "surfacemassdensity"
            },
            "wave number" => {
                name_lower == "wavenumber" || name_lower == "wave_number"
            },
            "kinematic viscosity" => {
                name_lower == "kinematic_viscosity" || name_lower == "kinematicviscosity"
            },
            "surface tension" => {
                name_lower == "surface_tension" || name_lower == "surfacetension"
            },
            "specific energy" => {
                name_lower == "specific_energy" || name_lower == "specificenergy"
            },
            "specific power" => {
                name_lower == "specific_power" || name_lower == "specificpower"
            },
            "mass flow rate" => {
                name_lower == "mass_flow_rate" || name_lower == "massflowrate"
            },
            "volume flow rate" => {
                name_lower == "volume_flow_rate" || name_lower == "volumeflowrate"
            },
            "power density" => {
                name_lower == "power_density" || name_lower == "powerdensity"
            },
            "force density" => {
                name_lower == "force_density" || name_lower == "forcedensity"
            },
            "heat flux" => {
                name_lower == "heat_flux" || name_lower == "heatflux"
            },
            "electric charge" => {
                name_lower == "electric_charge" || name_lower == "electriccharge" || name_lower == "charge"
            },
            "electric potential" => {
                name_lower == "electric_potential" || name_lower == "electricpotential" || name_lower == "potential"
            },
            "electric resistance" => {
                name_lower == "electric_resistance" || name_lower == "electricresistance" || name_lower == "resistance"
            },
            "electric conductance" => {
                name_lower == "electric_conductance" || name_lower == "electricconductance" || name_lower == "conductance"
            },
            "electric field" => {
                name_lower == "electric_field" || name_lower == "electricfield"
            },
            "magnetic field" => {
                name_lower == "magnetic_field" || name_lower == "magneticfield"
            },
            "magnetic flux" => {
                name_lower == "magnetic_flux" || name_lower == "magneticflux"
            },
            "linear charge density" => {
                name_lower == "linear_charge_density" || name_lower == "linearchargedensity"
            },
            "surface charge density" => {
                name_lower == "surface_charge_density" || name_lower == "surfacechargedensity"
            },
            "volume charge density" => {
                name_lower == "volume_charge_density" || name_lower == "volumechargedensity"
            },
            "magnetizing field" => {
                name_lower == "magnetizing_field" || name_lower == "magnetizingfield"
            },
            "specific heat capacity" => {
                name_lower == "specific_heat_capacity" || name_lower == "specificheatcapacity"
            },
            "molar heat capacity" => {
                name_lower == "molar_heat_capacity" || name_lower == "molarheatcapacity"
            },
            "thermal conductivity" => {
                name_lower == "thermal_conductivity" || name_lower == "thermalconductivity"
            },
            "thermal resistance" => {
                name_lower == "thermal_resistance" || name_lower == "thermalresistance"
            },
            "thermal expansion" => {
                name_lower == "thermal_expansion" || name_lower == "thermalexpansion"
            },
            "molar mass" => {
                name_lower == "molar_mass" || name_lower == "molarmass"
            },
            "molar volume" => {
                name_lower == "molar_volume" || name_lower == "molarvolume"
            },
            "molar concentration" => {
                name_lower == "molar_concentration" || name_lower == "molarconcentration"
            },
            "molal concentration" => {
                name_lower == "molal_concentration" || name_lower == "molalconcentration"
            },
            "molar flow rate" => {
                name_lower == "molar_flow_rate" || name_lower == "molarflowrate"
            },
            "molar flux" => {
                name_lower == "molar_flux" || name_lower == "molarflux"
            },
            "molar energy" => {
                name_lower == "molar_energy" || name_lower == "molarenergy"
            },
            "luminous exposure" => {
                name_lower == "luminous_exposure" || name_lower == "luminousexposure"
            },
            "luminous efficacy" => {
                name_lower == "luminous_efficacy" || name_lower == "luminousefficacy"
            },
            _ => false,
        }
    })
}

/// Look up dimension information by exponents
/// 
/// Returns the dimension info if found, or None if the exponent combination is not recognized.
pub fn lookup_dimension_by_exponents(exponents: DimensionExponents) -> Option<&'static DimensionInfo> {
    DIMENSION_LOOKUP.iter().find(|info| info.exponents == exponents)
}
