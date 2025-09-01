const { MacroExpander } = require('./out/macro-expander');

async function testMacroExpander() {
    console.log('Testing Macro Expander...\n');
    
    const expander = new MacroExpander();
    
    // Debug: Show what project directory was found
    console.log('Project directory found:', expander.projectDir);
    console.log('');
    
    // Test 1: Simple unit expansion
    console.log('Test 1: Expanding unit!(m)');
    const result1 = await expander.expandUnitMacro('unit!(m)');
    console.log('Result:', result1);
    
    if (result1.success) {
        console.log('✅ Successfully expanded unit!(m) to:', result1.expandedType);
    } else {
        console.log('❌ Failed to expand unit!(m):', result1.error);
    }
    
    console.log('\n' + '='.repeat(50) + '\n');
    
    // Test 2: Compound unit expansion
    console.log('Test 2: Expanding unit!(kg * m / s^2)');
    const result2 = await expander.expandUnitMacro('unit!(kg * m / s^2)');
    console.log('Result:', result2);
    
    if (result2.success) {
        console.log('✅ Successfully expanded unit!(kg * m / s^2) to:', result2.expandedType);
    } else {
        console.log('❌ Failed to expand unit!(kg * m / s^2):', result2.error);
    }
    
    console.log('\n' + '='.repeat(50) + '\n');
    
    // Test 3: Invalid input
    console.log('Test 3: Invalid input');
    const result3 = await expander.expandUnitMacro('not_a_macro');
    console.log('Result:', result3);
    
    if (!result3.success && result3.error === 'Text does not start with unit!') {
        console.log('✅ Correctly rejected invalid input');
    } else {
        console.log('❌ Did not handle invalid input correctly');
    }
    
    console.log('\nTest completed!');
}

// Run the test
testMacroExpander().catch(console.error);
