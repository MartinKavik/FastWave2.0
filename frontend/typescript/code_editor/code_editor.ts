// https://davidmyers.dev/blog/how-to-build-a-code-editor-with-codemirror-6-and-typescript/introduction

// @TODO Rewrite to Rust? (https://crates.io/crates/codemirror)

import { EditorState, Compartment, Extension } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import { basicSetup } from 'codemirror'
import { oneDark } from '@codemirror/theme-one-dark'
import { StreamLanguage } from "@codemirror/language"
import { languageServer } from 'codemirror-languageserver';

import { rust } from "@codemirror/lang-rust"
import { verilog } from "@codemirror/legacy-modes/mode/verilog"
import { toml } from "@codemirror/legacy-modes/mode/toml"

type LanguageServerConfig = {
    name: string
    file_path: string
    language: string
}

function create_language_server(config: LanguageServerConfig) {
    return languageServer({
        serverUri: `ws://localhost:9999?name=${config.name}`,
        rootUri: null,
        workspaceFolders: null,
        documentUri: `file:////${config.file_path}`,
        languageId: config.language
    });
}

export class CodeEditorController {
    constructor() {}

    editor_view: EditorView | null = null
    programming_language = new Compartment()
    language_server = new Compartment()
    file_content = new Compartment()

    set_selected_file(path: string | null, content: string) {
        this.editor_view!.dispatch({
            changes: [
                { from: 0, to: this.editor_view!.state.doc.length },
                { from: 0, insert: content },
            ]
        })

        if (typeof(path) === 'string') {
            let new_language: Extension | null = null;
            let new_language_server_config: LanguageServerConfig | null = null;

            if (path.endsWith(".rs")) {
                new_language = rust();
                new_language_server_config = {
                    name: 'rust-analyzer',
                    file_path: path,
                    language: 'rust',
                };
            } else if (path.endsWith(".v") || path.endsWith(".sv")) {
                new_language = StreamLanguage.define(verilog);
                new_language_server_config = {
                    name: 'veridian',
                    file_path: path,
                    language: 'verilog',
                };
            } else if (path.endsWith(".toml")) {
                new_language = StreamLanguage.define(toml);
                new_language_server_config = null;
            } else {
                console.error(`This file extension is not supported: '${path}'`)
            }

            if (new_language !== null) {
                this.editor_view!.dispatch({
                    effects: this.programming_language.reconfigure(new_language)
                })
            } else {
                this.editor_view!.dispatch({
                    effects: this.programming_language.reconfigure([])
                })
            }

            if (new_language_server_config !== null) {
                const new_language_server = create_language_server(new_language_server_config);
                this.editor_view!.dispatch({
                    effects: this.language_server.reconfigure(new_language_server)
                })
            } else {
                this.editor_view!.dispatch({
                    effects: this.language_server.reconfigure([])
                })
            }
        }
    }

    async init(parent_element: HTMLElement) {
        const state = EditorState.create({
            extensions: [
                basicSetup,
                oneDark,
                this.programming_language.of([]),
                this.language_server.of([])
            ],
        })

        this.editor_view = new EditorView({
            parent: parent_element,
            state
        });
    }
}
