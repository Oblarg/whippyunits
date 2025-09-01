import * as vscode from 'vscode';
import { MacroExpander } from './macro-expander';

export function activate(context: vscode.ExtensionContext) {
    console.log('WhippyUnits Refactor extension is now active!');

    const macroExpander = new MacroExpander();

    let disposable = vscode.commands.registerCommand('whippyunits.generateUnitAlias', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor found');
            return;
        }

        const document = editor.document;
        if (document.languageId !== 'rust') {
            vscode.window.showErrorMessage('This command only works in Rust files');
            return;
        }

        const selection = editor.selection;
        const range = new vscode.Range(selection.start, selection.end);
        const text = document.getText(range);

        // Check if the selected text contains a unit! macro
        if (!text.includes('unit!')) {
            vscode.window.showErrorMessage('Please select a unit! macro expression');
            return;
        }

        try {
            // Get the alias name from user
            const aliasName = await vscode.window.showInputBox({
                prompt: 'Enter the name for the type alias',
                placeHolder: 'e.g., Distance, Velocity, Force',
                validateInput: (value: string) => {
                    if (!value || value.trim() === '') {
                        return 'Alias name cannot be empty';
                    }
                    if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(value)) {
                        return 'Alias name must be a valid Rust identifier';
                    }
                    return null;
                }
            });

            if (!aliasName) {
                return; // User cancelled
            }

            // Expand the unit! macro using rustc
            const result = await macroExpander.expandUnitMacro(text);
            
            if (!result.success || !result.expandedType) {
                vscode.window.showErrorMessage(`Failed to expand unit! macro: ${result.error}`);
                return;
            }

            // Generate the type alias declaration
            const typeAlias = `type ${aliasName} = ${result.expandedType};`;

            // Find the best place to insert the type alias
            const insertPosition = findBestInsertPosition(document, selection.start);
            
            // Create the edit
            const edit = new vscode.WorkspaceEdit();
            edit.insert(document.uri, insertPosition, typeAlias + '\n');

            // Apply the edit
            const success = await vscode.workspace.applyEdit(edit);
            
            if (success) {
                vscode.window.showInformationMessage(`Generated type alias: ${aliasName}`);
            } else {
                vscode.window.showErrorMessage('Failed to insert type alias');
            }

        } catch (error) {
            vscode.window.showErrorMessage(`Error: ${error}`);
        }
    });

    context.subscriptions.push(disposable);
}



function findBestInsertPosition(document: vscode.TextDocument, currentPosition: vscode.Position): vscode.Position {
    // Try to find a good place to insert the type alias
    // Look for the start of the current function or module
    
    const text = document.getText();
    const lines = text.split('\n');
    
    // Start from the current line and go backwards
    for (let i = currentPosition.line; i >= 0; i--) {
        const line = lines[i].trim();
        
        // If we find a function declaration, insert before it
        if (line.startsWith('fn ') || line.startsWith('pub fn ')) {
            return new vscode.Position(i, 0);
        }
        
        // If we find a struct or enum declaration, insert before it
        if (line.startsWith('struct ') || line.startsWith('enum ') || 
            line.startsWith('pub struct ') || line.startsWith('pub enum ')) {
            return new vscode.Position(i, 0);
        }
        
        // If we find a module declaration, insert after it
        if (line.startsWith('mod ') || line.startsWith('pub mod ')) {
            return new vscode.Position(i + 1, 0);
        }
    }
    
    // If we can't find a good place, insert at the top of the file
    return new vscode.Position(0, 0);
}

export function deactivate() {}
