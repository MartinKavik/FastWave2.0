Init
- `npm install`

Watch & build (without typechecking)
- `node_modules/.bin/esbuild monaco_editor.ts --bundle --minify --outfile=../bundles/monaco_editor.js --format=esm --watch`

Watch & typecheck (without building)
- `node_modules/.bin/tsc monaco_editor.ts --watch -noEmit --preserveWatchOutput --target esnext --module esnext --moduleResolution bundler`

Created with commands:
- `npm i -E vscode@npm:@codingame/monaco-vscode-api`
- `npm i -E monaco-editor@npm:@codingame/monaco-vscode-editor-api`
- `npm i -E @codingame/monaco-vscode-chat-extensions-notebook-task-terminal-testing-common`
- `npm i -D @types/vscode`
- `npm i -D esbuild typescript`
