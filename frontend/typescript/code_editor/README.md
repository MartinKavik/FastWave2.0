Init
- `npm install`

Watch & build (without typechecking)
- `node_modules/.bin/esbuild code_editor.ts --bundle --minify --outfile=../bundles/code_editor.js --format=esm --watch`

Watch & typecheck (without building)
- `node_modules/.bin/tsc code_editor.ts --watch -noEmit --preserveWatchOutput --target esnext --module esnext --moduleResolution bundler`

Created with commands:
- `npm i -E codemirror`
- `npm i -E @codemirror/theme-one-dark`
- `npm i -E @codemirror/language`
- `npm i -E @codemirror/legacy-modes`
- `npm i -E @codemirror/view`
- `npm i -E @codemirror/state`
- `npm i -E @codemirror/lang-rust`
- `npm i -E codemirror-languageserver`
- `npm i -E events`
- `npm i -D @types/events`
- `npm i -D @types/node`
- `npm i -D esbuild typescript@5.5.4`
