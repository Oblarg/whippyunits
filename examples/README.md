# WhippyUnits Examples

## Quick Start

**New to whippyunits?** Start with the [`getting_started/`](./getting_started/) directory, which contains a step-by-step learning path.

**Run an example:**
```bash
cargo run --example <example_name>
```

## Example Categories

### Getting Started (`getting_started/`)

**Start here if you're new to whippyunits!**

- `setup.rs` - Project setup and installation
- `hello_world.rs` - Your first quantity
- `concepts.rs` - Core concepts (dimensions, scales, type safety)
- `declarators.rs` - Different ways to create quantities
- `storage_types.rs` - Storage type parameter (f64, f32, i32, etc.) and when to use each
- `common_errors.rs` - Understanding and fixing common errors

### Operations (`operations/`)

Fundamental operations with quantities:

- **`arithmetic.rs`** - Basic arithmetic operations (addition, subtraction, multiplication, division)
- **`comparison.rs`** - Comparison operators (scale-strict comparisons)
- **`rescaling.rs`** - Converting between units of the same dimension
- **`value_access.rs`** - Safe value access with `value!` macro, or unsafely via `.unsafe_value`

### Erasure (`erasure/`)

Converting dimensionless and angular quantities to numeric types:

- **`scalar_erasure.rs`** - Erasing dimensionless quantities (ratios) to scalars
  - Automatic rescaling to unity on erasure
  - Safe vs unsafe access patterns
  - Using with standard library functions

- **`angular_erasure.rs`** - Erasing angular quantities to scalars or non-angular quantities
  - Automatic rescaling to radian scale on erasure
  - Automatic erasure of compound units with radian components
  - Safe vs unsafe access patterns
  - Using with standard library functions

### Type Assertions (`type_assertions/`)

Using type annotations to verify operations at compile time:

- **`safe_mult_div.rs`** - Verifying multiplication/division dimensions
- **`rescale_targeting.rs`** - Type-safe rescaling operations

### Custom Declarators (`custom_declarators/`)

Creating custom unit creation functions with specialized behavior:

- **`branded_declarators.rs`** - Adding type-level brands to prevent mixing quantities from different contexts (e.g., different coordinate systems)
- **`rescaling_declarators.rs`** - Automatically converting all quantities to specified base scales for storage

### Generics (`generics/`)

Unit-generic programming patterns using generic traits and trait bounds:

- **`centripetal_acceleration.rs`** - Scale-generic physics calculations using `define_generic_dimension!`
- **`filter.rs`** - Unbounded generic filtering that works with any quantity type
- **`pid_controller.rs`** - Scale-generic control systems with dimensional trait bounds

**Note:** These examples demonstrate how to write code that works across different scales, dimensions, and even different units-of-measure libraries. The patterns shown are more about generic Rust programming than whippyunits-specific syntax.

### Serialization (`serialization/`)

Working with JSON and string formats:

- **`from_json.rs`** - Parsing quantities from JSON
- **`from_string.rs`** - Parsing quantities from strings

## By Experience Level

### Beginner ("What is unit safety?")
1. [`getting_started/`](./getting_started/) - Installation, setup, and quantity creation
2. [`operations/`](./operations/) - Fundamental operations
3. [`erasure/`](./erasure/) - "Lowering" to underlying numeric types, dimensionless quantities, or non-angular quantities
4. [`type_assertions/`](./type_assertions/) - Compile-time verification

### Intermediate ("How do I use this for my specific problem?")
1. [`custom_declarators/`](./custom_declarators/) - Custom declarator namespaces that produce branded or rescaled quantities
2. [`serialization/`](./serialization/) - Serialization and deserialization

### Expert ("How do I write library-agnostic code?")
1. [`generics/`](./generics/) - Write code that works across multiple scales, dimensions, unit systems, or even different units-of-measure libraries. These examples focus on generic Rust programming patterns (trait bounds, generic functions) rather than whippyunits-specific syntax.

## Running Examples

All examples can be run with:
```bash
cargo run --example <example_name>
```

For example:
```bash
cargo run --example hello_world
cargo run --example arithmetic
cargo run --example scalar_erasure
```

## Documentation

- Main documentation: [`../README.md`](../README.md)
- API documentation: [docs.rs/whippyunits](https://docs.rs/whippyunits)
- Getting Started guide: [`getting_started/README.md`](./getting_started/README.md)

## Contributing

Found a bug or have a suggestion? Examples are a great way to demonstrate features and patterns. Consider adding examples for:
- New use cases
- Common patterns
- Edge cases
- Integration examples

