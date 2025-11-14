//! Linear IIR Filter Example: Unbounded Scale-Generic Signal Processing
//!
//! This example demonstrates an unbounded linear IIR (Infinite Impulse Response) filter
//! that works with any dimension and any scale. The filter is completely generic - it
//! doesn't use `define_generic_dimension` bounds, making it applicable to any quantity type.
//!
//! Key concepts:
//! - Unbounded generics: Works with any Quantity type, regardless of dimension
//! - Scale-generic: Works with any scale (meters, millimeters, seconds, milliseconds, etc.)
//! - Realistic signal processing: First-order low-pass IIR filter (exponential moving average)
//!
//! The filter implements: y[n] = α * x[n] + (1 - α) * y[n-1]
//! where α is the filter coefficient (0 < α ≤ 1)

use core::ops::{Add, Mul, Sub};
use whippyunits::dimension_traits::{Time, define_generic_dimension};
use whippyunits::op_result;
use whippyunits::quantity;
use whippyunits::unit;

/// First-order IIR low-pass filter (exponential moving average)
///
/// This filter is completely unbounded - it works with any Quantity type, regardless of
/// dimension or scale. The filter coefficient α determines the cutoff frequency:
/// - α = 1.0: No filtering (output = input)
/// - α → 0: Heavy filtering (slow response)
///
/// The filter maintains dimensional and scale coherence - if you filter a position signal
/// in meters, the output will also be in meters. If you filter a velocity signal in mm/s,
/// the output will be in mm/s.
pub struct IIRFilter<Q> {
    /// Filter coefficient (0 < α ≤ 1)
    /// Dimensionless - same for all quantity types
    alpha: f64,
    /// Previous output value
    /// Has the same dimension and scale as the input
    prev_output: Option<Q>,
}

// we use the op_result macro to make the arithmetic constraints readable
#[op_result]
impl<Q> IIRFilter<Q>
where
    Q: Copy,
{
    /// Create a new IIR filter with the given coefficient
    ///
    /// # Arguments
    /// * `alpha` - Filter coefficient (0 < α ≤ 1). Smaller values = more filtering.
    pub fn new(alpha: f64) -> Self {
        assert!(alpha > 0.0 && alpha <= 1.0, "Alpha must be in (0, 1]");
        Self {
            alpha,
            prev_output: None,
        }
    }

    /// Process a single sample through the filter
    ///
    /// The filter equation is: y[n] = α * x[n] + (1 - α) * y[n-1]
    ///
    /// This method is completely generic - it works with any Quantity type.
    /// The only constraint is that the quantity must support scalar multiplication
    /// and addition with itself.
    pub fn filter(&mut self, input: Q) -> Q
    where
        Q: Copy,
        [(); Q * f64 = Q]:,
        [(); Q + Q = Q]:,
    {
        let weighted_input = input * self.alpha;

        if let Some(prev) = self.prev_output {
            // y[n] = α * x[n] + (1 - α) * y[n-1]
            let weighted_prev = prev * (1.0 - self.alpha);
            let output = weighted_input + weighted_prev;
            self.prev_output = Some(output);
            output
        } else {
            // First sample: output = input (no previous value)
            self.prev_output = Some(weighted_input);
            weighted_input
        }
    }

    /// Reset the filter state
    pub fn reset(&mut self) {
        self.prev_output = None;
    }
}

// Define Frequency as 1/Time (can now use 1/T or T^-1)
define_generic_dimension!(Frequency, 1 / T);

/// Signal generator that produces noise
///
/// Generates: signal = base + noise
/// where noise has a normal distribution with the given standard deviation
///
/// The generator is completely generic - works with any signal dimension and scale.
pub struct SignalGenerator<Q, T> {
    noise_stddev: Q,  // Standard deviation of noise (same dimension as signal)
    sample_period: T, // Time between samples (time step)
    time: T,          // Current time
}

impl<Q, T> SignalGenerator<Q, T>
where
    Q: Copy,
    T: Time + Copy,
{
    /// Create a new signal generator
    ///
    /// # Arguments
    /// * `noise_stddev` - Standard deviation of noise (same dimension as signal)
    /// * `sample_period` - Time between samples (time step)
    pub fn new(noise_stddev: Q, sample_period: T) -> Self
    where
        T: Sub<T, Output = T>,
    {
        // Initialize time to zero (same type as sample_period)
        let zero_time = sample_period - sample_period;
        Self {
            noise_stddev,
            sample_period,
            time: zero_time,
        }
    }

    /// Generate the next sample
    ///
    /// Returns: base + noise
    pub fn next_sample(&mut self, base: Q) -> Q
    where
        Q: Add<Q, Output = Q> + Mul<f64, Output = Q> + Copy,
        T: Add<T, Output = T>,
    {
        // Generate noise: uniform distribution in [-stddev, stddev]
        let u = rand::random(); // [0, 1)
        let z = 2.0 * u - 1.0; // [-1, 1)
        let noise = self.noise_stddev * z;

        // Combine: signal = base + noise
        let signal = base + noise;

        // Advance time by sample period
        self.time = self.time + self.sample_period;

        signal
    }

    /// Reset the generator
    pub fn reset(&mut self)
    where
        T: Sub<T, Output = T>,
    {
        let zero_time = self.sample_period - self.sample_period;
        self.time = zero_time;
    }
}

// Simple PRNG for noise (avoiding external dependencies)
mod rand {
    static mut STATE: u64 = 123456789;

    pub fn random() -> f64 {
        unsafe {
            // Linear congruential generator
            STATE = STATE.wrapping_mul(1103515245).wrapping_add(12345);
            // Convert to [0, 1) by dividing by u64::MAX + 1
            STATE as f64 / (u64::MAX as f64 + 1.0)
        }
    }
}

fn main() {
    println!("Linear IIR Filter Demo");
    println!("======================\n");
    println!("Generate noisy oscillating signals and filter them!\n");

    // Example 1: Filtering position signals (meters) with seconds
    println!("Example 1: Position Signal (meters, sample period: 0.1s)");
    println!("--------------------------------------------------------");
    let sample_period = quantity!(0.1, s);
    let noise_stddev = quantity!(0.1, m);
    let mut generator = SignalGenerator::new(noise_stddev, sample_period);
    let mut filter = IIRFilter::new(0.3);
    let base = quantity!(1.0, m);

    println!("  Raw signal (base + noise):");
    for i in 0..10 {
        let signal: unit!(m) = generator.next_sample(base);
        let filtered: unit!(m) = filter.filter(signal);
        println!("    Sample {}: {} -> {}", i, signal, filtered);
    }

    // Example 2: Filtering velocity signals (m/s) with milliseconds
    println!("\nExample 2: Velocity Signal (m/s, sample period: 100ms)");
    println!("-------------------------------------------------------");
    let sample_period = quantity!(100.0, ms);
    let noise_stddev = quantity!(0.3, m / s);
    let mut generator = SignalGenerator::new(noise_stddev, sample_period);
    let mut filter = IIRFilter::new(0.2);
    let base = quantity!(10.0, m / s);

    println!("  Raw signal (base + noise):");
    for i in 0..10 {
        let signal: unit!(m / s) = generator.next_sample(base);
        let filtered: unit!(m / s) = filter.filter(signal);
        println!("    Sample {}: {} -> {}", i, signal, filtered);
    }

    // Example 3: Different time scale (milliseconds) and position scale (millimeters)
    println!("\nExample 3: Position Signal (mm, sample period: 10ms)");
    println!("-----------------------------------------------------");
    let sample_period = quantity!(10.0, ms);
    let noise_stddev = quantity!(100.0, mm);
    let mut generator = SignalGenerator::new(noise_stddev, sample_period);
    let mut filter = IIRFilter::new(0.3);
    let base = quantity!(1000.0, mm);

    println!("  Raw signal (base + noise):");
    for i in 0..10 {
        let signal: unit!(mm) = generator.next_sample(base);
        let filtered: unit!(mm) = filter.filter(signal);
        println!("    Sample {}: {} -> {}", i, signal, filtered);
    }

    println!("\n✅ Filter works with any dimension and scale - completely unbounded!");
}
