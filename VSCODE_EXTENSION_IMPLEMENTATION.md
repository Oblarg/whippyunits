# WhippyUnits VSCode Extension Implementation

## Overview

This document describes the implementation of the VSCode extension that provides refactor functionality for WhippyUnits `unit!` macro expressions. The extension allows users to generate type aliases from `unit!` macro expressions by using rustc to expand the macros.

## Architecture

### Core Components

1. **Extension Entry Point** (`src/extension.ts`)
   - Registers the "Generate Unit Alias" command
   - Handles user interaction and file editing
   - Coordinates the refactor workflow

2. **Macro Expander** (`src/macro-expander.ts`)
   - Uses rustc to expand `unit!` macros
   - Generates temporary Rust files
   - Parses rustc output to extract expanded types

3. **Test Suite** (`src/test/`)
   - Unit tests for the macro expander
   - Integration tests for the extension

## Workflow

### 1. User Interaction
1. User selects a `unit!` macro expression in a Rust file
2. User right-clicks and selects "Refactor" → "Generate Unit Alias"
3. Extension prompts for an alias name
4. Extension validates the alias name (must be a valid Rust identifier)

### 2. Macro Expansion
1. Extension creates a temporary Rust file with the macro usage
2. Runs `rustc --pretty=expanded` on the temporary file
3. Parses the output to extract the expanded type
4. Cleans up the temporary file

### 3. Code Generation
1. Extension generates a type alias declaration
2. Finds the best location to insert the alias (before functions, after modules, etc.)
3. Applies the edit using VSCode's WorkspaceEdit API

## Key Implementation Details

### Macro Expansion Strategy

The extension uses rustc directly because rust-analyzer doesn't expand macros. The process:

1. **Temporary File Generation**: Creates a minimal Rust program that uses the `unit!` macro
2. **rustc Execution**: Uses `rustc --pretty=expanded` to get the expanded output
3. **Fallback Strategy**: If `--pretty=expanded` fails, tries `--emit=mir`
4. **Output Parsing**: Extracts the expanded `Quantity<...>` type from rustc output

### Type Alias Insertion

The extension intelligently places type aliases:

1. **Before Functions**: If the macro is inside a function, insert before the function
2. **Before Structs/Enums**: If near type definitions, insert before them
3. **After Modules**: If near module declarations, insert after them
4. **File Top**: As a last resort, insert at the top of the file

### Error Handling

The extension handles various error scenarios:

- **rustc not found**: Clear error message about missing Rust toolchain
- **Macro expansion failure**: Detailed error from rustc stderr
- **Type extraction failure**: Fallback to alternative parsing methods
- **File editing failure**: Graceful degradation with user feedback

## File Structure

```
vscode-extension/
├── package.json              # Extension manifest
├── tsconfig.json             # TypeScript configuration
├── src/
│   ├── extension.ts          # Main extension entry point
│   ├── macro-expander.ts     # Macro expansion logic
│   └── test/
│       ├── runTest.ts        # Test runner
│       ├── suite/
│       │   ├── index.ts      # Test suite setup
│       │   └── macro-expander.test.ts  # Unit tests
├── example.rs                # Example usage
├── README.md                 # User documentation
├── build.sh                  # Build script
└── .vscodeignore            # Package exclusions
```

## Dependencies

### Runtime Dependencies
- `child_process`: For spawning rustc processes
- `fs`, `path`, `os`: For file system operations

### Development Dependencies
- `@types/vscode`: VSCode extension API types
- `@types/node`: Node.js types
- `typescript`: TypeScript compiler
- `@vscode/test-electron`: Testing framework
- `mocha`: Test runner
- `glob`: File pattern matching for tests

## Testing

### Unit Tests
- Test macro input validation
- Test error handling scenarios
- Test type extraction logic

### Integration Tests
- Test full refactor workflow
- Test file editing operations
- Test user interaction flows

## Usage Example

**Input:**
```rust
fn calculate_distance() -> unit!(m) {
    5.0.meters()
}
```

**User Action:**
1. Select `unit!(m)`
2. Right-click → Refactor → Generate Unit Alias
3. Enter alias name: "Distance"

**Output:**
```rust
type Distance = whippyunits::Quantity<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>;

fn calculate_distance() -> Distance {
    5.0.meters()
}
```

## Future Enhancements

1. **Batch Refactoring**: Refactor multiple `unit!` macros at once
2. **Smart Naming**: Suggest alias names based on context
3. **Import Management**: Automatically add necessary imports
4. **Undo Support**: Better integration with VSCode's undo system
5. **Configuration**: User-configurable insertion preferences

## Troubleshooting

### Common Issues

1. **"rustc not found"**
   - Ensure Rust toolchain is installed
   - Check PATH environment variable
   - Verify rustc is accessible from terminal

2. **"Failed to expand macro"**
   - Check that WhippyUnits crate is in Cargo.toml
   - Verify the `unit!` macro syntax is correct
   - Check rustc error output for specific issues

3. **"No active editor"**
   - Ensure a Rust file is open
   - Verify text is selected
   - Check file language is set to Rust

### Debug Mode

To enable debug logging:
1. Open VSCode Developer Tools (Help → Toggle Developer Tools)
2. Look for console output from the extension
3. Check the "Output" panel for extension logs

## Development Setup

1. **Clone and Install:**
   ```bash
   cd vscode-extension
   npm install
   ```

2. **Build:**
   ```bash
   npm run compile
   ```

3. **Test:**
   ```bash
   npm run test
   ```

4. **Run in Development:**
   - Open the extension directory in VSCode
   - Press F5 to launch extension host
   - Test with example.rs file

## Conclusion

This VSCode extension provides a seamless refactoring experience for WhippyUnits users by leveraging rustc's macro expansion capabilities. The implementation is robust, handles errors gracefully, and provides clear feedback to users. The extension follows VSCode best practices and integrates well with the existing development workflow.
