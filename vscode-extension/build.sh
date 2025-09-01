#!/bin/bash

echo "Building WhippyUnits Refactor Extension..."

# Install dependencies if node_modules doesn't exist
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies..."
    npm install
fi

# Compile TypeScript
echo "Compiling TypeScript..."
npm run compile

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "To test the extension:"
    echo "1. Open this directory in VSCode"
    echo "2. Press F5 to run the extension in development mode"
    echo "3. Open the example.rs file"
    echo "4. Select a unit! macro and right-click → Refactor → Generate Unit Alias"
else
    echo "❌ Build failed!"
    exit 1
fi
