//! PID Controller Example: Demonstrating Generic Dimensions with Disjunctions
//!
//! This example shows how to use generic dimensions with disjunctions to write
//! a PID controller that works with any process variable type (angular position,
//! angular velocity, linear position, linear velocity) while maintaining full
//! dimensional safety.
//!
//! The key insight: PID controller gains are NOT dimensionless - they have dimensions
//! derived from the process variable and control output types using the Mul/Div trait
//! associated types.

use whippyunits::dimension_traits::Time;
use whippyunits::dimension_traits::define_generic_dimension;
use whippyunits::op_result;

// Define ProcessVariable as a disjunction - can be angular position, angular velocity,
// linear position, or linear velocity
define_generic_dimension!(ProcessVariable, Length, L / T, Angle, A / T);

// Define ControlOutput as a disjunction - voltage or current
define_generic_dimension!(ControlOutput, M * L ^ 2 / T ^ 3 / I, I);

/// PID Controller type namespace that provides type aliases for derived types
///
/// This struct provides type aliases as generic parameters with computed defaults.
/// Type aliases are computed from PV, CO, and T:
/// - `ProportionalGain = CO / PV`
/// - `IntegralGain = CO / (PV * T)`
/// - `DerivativeGain = (CO * T) / PV`
pub struct PIDController<
    PV: ProcessVariable,
    CO: ControlOutput,
    T: Time,
    ProportionalGain,
    IntegralGain,
    DerivativeGain,
    Integral,
    Derivative,
> {
    kp: ProportionalGain,
    ki: IntegralGain,
    kd: DerivativeGain,
    dt: T,
    integral: Integral,
    prev_error: Option<PV>,
    _phantom: core::marker::PhantomData<(fn() -> CO, Derivative)>,
}

#[op_result]
impl<
    PV: ProcessVariable,
    CO: ControlOutput,
    T: Time,
    ProportionalGain,
    IntegralGain,
    DerivativeGain,
    Integral,
    Derivative,
> PIDController<PV, CO, T, ProportionalGain, IntegralGain, DerivativeGain, Integral, Derivative>
where
    PV: Copy,
    T: Copy,
    ProportionalGain: Copy,
    IntegralGain: Copy,
    DerivativeGain: Copy,
    Integral: Copy,
    [(); PV / T = Derivative]:,
    [(); PV * T = Integral]:,
    [(); ProportionalGain * PV = CO]:,
    [(); IntegralGain * Integral = CO]:,
    [(); DerivativeGain * Derivative = CO]:,
    [(); PV - PV = PV]:,
    [(); CO + CO = CO]:,
    [(); Integral + Integral = Integral]:,
{
    /// Create a new PID controller with the given gains and time step
    pub fn new(
        kp: ProportionalGain,
        ki: IntegralGain,
        kd: DerivativeGain,
        dt: T,
        initial_pv: PV,
    ) -> Self {
        let zero_pv = initial_pv - initial_pv;
        Self {
            kp,
            ki,
            kd,
            dt,
            integral: zero_pv * dt,
            prev_error: None,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Compute PID control output
    ///
    /// Dimensional analysis:
    /// - Kp * PV = CO  → Kp = CO / PV
    /// - Ki * (PV * T) = CO  → Ki = CO / (PV * T)
    /// - Kd * (PV / T) = CO  → Kd = (CO * T) / PV
    ///
    /// Returns: control_output
    pub fn compute(&mut self, pv: PV, setpoint: PV) -> CO {
        // Error
        let error = pv - setpoint;

        // Proportional term
        let p_term = self.kp * error;

        // Integral term
        let error_dt = error * self.dt;
        self.integral = self.integral + error_dt;
        let i_term = self.ki * self.integral;

        // Derivative term
        let d_term = if let Some(prev_error) = self.prev_error {
            let error_change = error - prev_error;
            let error_rate = error_change / self.dt;
            self.kd * error_rate
        } else {
            // On first iteration, derivative term is zero
            // Compute zero by: d_term = kd * (error_rate where error_rate = 0)
            // We can get zero error_rate as: (error - error) / dt
            let zero_error_rate = (error - error) / self.dt;
            self.kd * zero_error_rate
        };

        let output = p_term + i_term + d_term;
        self.prev_error = Some(error);
        output
    }
}

fn main() {
    use whippyunits::quantity;
    use whippyunits::unit;

    println!("Testing PID Controller with Concrete Types");
    println!("==========================================");
    println!();

    // Example: Position control with voltage output
    // Process variable: linear position (meters)
    // Control output: voltage (volts)
    // Time: milliseconds

    // Setpoint: 1.0 meters
    let setpoint = quantity!(1.0, m);
    // Current position: 0.5 meters
    let pv = quantity!(0.5, m);
    // Time step: 10 milliseconds
    let dt = quantity!(10.0, ms);

    // PID gains (calculated from CO/PV, CO/(PV*T), (CO*T)/PV)
    // Kp: V / m = volts per meter
    let kp = quantity!(5.0, V / m);
    // Ki: V / (m * ms) = volts per meter-millisecond
    let ki = quantity!(2.0, V / (m * ms));
    // Kd: (V * ms) / m = volt-milliseconds per meter
    let kd = quantity!(1.0, V * ms / m);

    println!("Initial state:");
    println!("  Setpoint: {}", setpoint);
    println!("  PV: {}", pv);
    println!("  dt: {}", dt);
    println!();

    // Create a PID controller instance with gains and time step
    let mut controller = PIDController::new(kp, ki, kd, dt, pv);

    // Run a few iterations
    for i in 0..5 {
        // Type inference: Rust infers PV, CO, T from the concrete quantity types
        let control_output: unit!(V) = controller.compute(pv, setpoint);

        println!("Iteration {}:", i + 1);
        println!("  Control output: {}", control_output);
        println!();
    }

    println!("✅ PID controller test completed successfully!");
}
