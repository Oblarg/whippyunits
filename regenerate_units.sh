#!/bin/bash

echo "Regenerating units system from dimensional metadata..."
echo ""

# Regenerate scale constants
echo "1. Regenerating scale constants..."
rustc generate_quantity_scale_consts.rs -o generate_quantity_scale_consts
./generate_quantity_scale_consts
rm -f generate_quantity_scale_consts
echo "   ✓ Generated constants in src/generated_constants.rs"
echo ""

# Regenerate unit macros
echo "2. Regenerating unit macros..."
rustc generate_unit_macros.rs -o generate_unit_macros
./generate_unit_macros
rm -f generate_unit_macros
echo "   ✓ Generated macros in src/generated_unit_macro.rs"
echo ""

echo "Done! All unit system components have been regenerated."
echo ""
echo "Generated files:"
echo "- src/generated_constants.rs (scale constants and conversion functions)"
echo "- src/generated_unit_macro.rs (unit macro patterns)"
echo ""
echo "To use these in your code:"
echo "1. Add 'mod generated_constants;' to src/lib.rs"
echo "2. Add 'mod generated_unit_macro;' to src/lib.rs"
echo "3. Use the generated helper functions for display and conversion"
echo "4. Use the unit! macro for declarative unit expressions"
