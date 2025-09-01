#!/bin/bash

echo "Regenerating units system from dimensional metadata..."
echo ""

# Change to generators directory
cd generators

        # Regenerate dimension traits
        echo "1. Regenerating dimension traits..."
        cargo run --bin generate_dimension_traits
        echo "   ✓ Generated dimension traits in ../src/dimension_traits.rs"
        echo ""

        # Regenerate dimension structs
        echo "2. Regenerating dimension structs..."
        cargo run --bin generate_dimension_structs
        echo "   ✓ Generated dimension structs in ../src/dimension_structs.rs"
        echo ""

        # Regenerate Quantity type
        echo "3. Regenerating Quantity type..."
        cargo run --bin generate_quantity_type
        echo "   ✓ Generated Quantity type in ../src/generated_quantity_type.rs"
        echo ""

        # Regenerate arithmetic
        echo "4. Regenerating arithmetic..."
        cargo run --bin generate_arithmetic
        echo "   ✓ Generated arithmetic in ../src/generated_arithmetic.rs"
        echo ""

        # Regenerate unit macros
        echo "5. Regenerating unit macros..."
        cargo run --bin generate_unit_macros
        echo "   ✓ Generated macros in ../src/generated_unit_macro.rs"
        echo ""

        # Regenerate scale constants
        echo "6. Regenerating scale constants..."
        cargo run --bin generate_quantity_scale_consts
        echo "   ✓ Generated constants in ../src/generated_constants.rs"
        echo ""

        # Regenerate prettyprint parser
        echo "7. Regenerating prettyprint parser..."
        cargo run --bin generate_prettyprint
        echo "   ✓ Generated prettyprint parser in ../src/generated_prettyprint.rs"
        echo ""

# Return to original directory
cd ..

echo "Done! All unit system components have been regenerated."
echo ""
echo "Generated files:"
echo "- src/dimensions.rs (dimensional traits and implementations)"
echo "- src/generated_quantity_type.rs (Quantity type definition)"
echo "- src/generated_arithmetic.rs (arithmetic trait implementations)"
echo "- src/generated_unit_macro.rs (unit macro patterns)"
echo "- src/generated_constants.rs (scale constants and conversion functions)"
echo "- src/generated_prettyprint.rs (variadic prettyprinting parser)"
echo ""
echo "To use these in your code:"
echo "1. Add 'mod generated_constants;' to src/lib.rs"
echo "2. Add 'mod generated_unit_macro;' to src/lib.rs"
echo "3. Add 'mod generated_quantity_type;' to src/lib.rs"
echo "4. Add 'mod generated_arithmetic;' to src/lib.rs"
echo "5. Use the generated helper functions for display and conversion"
echo "6. Use the unit! macro for declarative unit expressions"
