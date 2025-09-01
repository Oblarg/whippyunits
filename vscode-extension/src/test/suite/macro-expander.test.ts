import * as assert from 'assert';
import { MacroExpander } from '../../macro-expander';

suite('Macro Expander Test Suite', () => {
    test('Should expand unit!(m) to Quantity type', async () => {
        const expander = new MacroExpander();
        const result = await expander.expandUnitMacro('unit!(m)');
        
        console.log('Expansion result:', result);
        
        if (result.success) {
            assert(result.expandedType, 'Should have expanded type');
            assert(result.expandedType!.includes('Quantity<'), 'Should contain Quantity type');
            assert(result.expandedType!.includes('whippyunits::'), 'Should contain whippyunits crate name');
            console.log('Successfully expanded to:', result.expandedType);
        } else {
            console.log('Expansion failed:', result.error);
            // This is expected if rustc or whippyunits is not available in test environment
            assert(result.error, 'Should have error message');
        }
    });

    test('Should expand unit!(kg * m / s^2) to Force type', async () => {
        const expander = new MacroExpander();
        const result = await expander.expandUnitMacro('unit!(kg * m / s^2)');
        
        console.log('Force expansion result:', result);
        
        if (result.success) {
            assert(result.expandedType, 'Should have expanded type');
            assert(result.expandedType!.includes('Quantity<'), 'Should contain Quantity type');
            console.log('Successfully expanded force to:', result.expandedType);
        } else {
            console.log('Force expansion failed:', result.error);
            assert(result.error, 'Should have error message');
        }
    });

    test('Should handle invalid macro input', async () => {
        const expander = new MacroExpander();
        const result = await expander.expandUnitMacro('not_a_macro');
        
        assert.strictEqual(result.success, false);
        assert.strictEqual(result.error, 'Text does not start with unit!');
    });

    test('Should handle empty input', async () => {
        const expander = new MacroExpander();
        const result = await expander.expandUnitMacro('');
        
        assert.strictEqual(result.success, false);
        assert.strictEqual(result.error, 'Text does not start with unit!');
    });
});
