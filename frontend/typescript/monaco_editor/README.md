Init
- `npm install`

Watch & build (without typechecking)
- `node_modules/.bin/esbuild monaco_editor.ts --bundle --minify --outfile=../bundles/monaco_editor.js --format=esm --watch`

Watch & typecheck (without building)
- `node_modules/.bin/tsc monaco_editor.ts --watch -noEmit --preserveWatchOutput --target esnext --module esnext --moduleResolution bundler`

Created with commands:
- `npm i -E monaco-editor`
- `npm i -D esbuild typescript`
