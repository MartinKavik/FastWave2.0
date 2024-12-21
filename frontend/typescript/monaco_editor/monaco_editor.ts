// https://davidmyers.dev/blog/how-to-build-a-code-editor-with-codemirror-6-and-typescript/introduction

// @TODO Rewrite to Rust? (https://crates.io/crates/codemirror)

// @TODO add to the main README
// rustup component add rust-analyzer
// cargo install --git https://github.com/vivekmalneedi/veridian.git
// Install lsp-ws-proxy from fork by this command:
// lsp-ws-proxy -- veridian

import { EditorState, Compartment } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import { basicSetup } from 'codemirror'
import { oneDark } from '@codemirror/theme-one-dark'
import { StreamLanguage } from "@codemirror/language"
import { languageServer } from 'codemirror-languageserver';

import { rust } from "@codemirror/lang-rust"
import { verilog } from "@codemirror/legacy-modes/mode/verilog"
import { toml } from "@codemirror/legacy-modes/mode/toml"

// @TODO rename `monaco_editor` to code_editor / codemirror

export class MonacoEditorController {
    constructor() {}

    async init(parent_element: HTMLElement) {
        const root_path = "D:/repos/FastWave2.0/test_files/ide/ide_example_verilog/";
        const file_path = "D:/repos/FastWave2.0/test_files/ide/ide_example_verilog/example.v";

        const ls = languageServer({
            serverUri: 'ws://localhost:9999',
            rootUri: `file:///${root_path}`,
            workspaceFolders: null,
            documentUri: `file:////${file_path}`,
            languageId: 'verilog'
        });

        // const root_path = "D:/repos/FastWave2.0/test_files/ide/ide_example_rust/";
        // const file_path = "D:/repos/FastWave2.0/test_files/ide/ide_example_rust/src/main.rs";

        // const ls = languageServer({
        //     serverUri: 'ws://localhost:9999',
        //     rootUri: `file:///${root_path}`,
        //     workspaceFolders: null,
        //     documentUri: `file:////${file_path}`,
        //     languageId: 'rust'
        // });

        const language = new Compartment()

        const state = EditorState.create({
            doc: code_example_verilog,
            extensions: [
                basicSetup,
                oneDark,
                language.of(StreamLanguage.define(verilog)),
                ls
            ],
        })

        const view = new EditorView({
            parent: parent_element,
            state
        })

        // // @TODO remove
        // view.dispatch({
        //     effects: language.reconfigure(StreamLanguage.define(scala))
        // })

        // // @TODO remove
        // view.dispatch({
        //     changes: [
        //         { from: 0, to: view.state.doc.length },
        //         { from: 0, insert: code_example_scala },
        //     ]
        // })

        // // @TODO remove
        // view.dispatch({
        //     effects: language.reconfigure(rust())
        // })

        // // @TODO remove
        // view.dispatch({
        //     changes: [
        //         { from: 0, to: view.state.doc.length },
        //         { from: 0, insert: code_example_rust },
        //     ]
        // })

        // // @TODO remove
        // console.log("CONTENT: ", view.state.doc.toString())
    }
}

const code_example_verilog = `include "first_counter.v"
module first_counter_tb();
// Declare inputs as regs and outputs as wires
reg clock, reset, enable;
wire [3:0] counter_out;

// Initialize all variables
initial begin
  $display ("time\\t clk reset enable counter");
  $monitor ("%g\\t %b   %b     %b      %b",
	  $time, clock, reset, enable, counter_out);
  clock = 1;       // initial value of clock
  reset = 0;       // initial value of reset
  enable = 0;      // initial value of enable
   #5  reset = 1;    // Assert the reset
   #10  reset = 0;   // De-assert the reset
   #10  enable = 1;  // Assert enable
   #100  enable = 0; // De-assert enable
   #5  $finish;      // Terminate simulation
end

// Clock generator
always begin
   #5  clock = ~clock; // Toggle clock every 5 ticks
end

// Connect DUT to test bench
first_counter U_counter (
clock,
reset,
enable,
counter_out
);

endmodule
`;

const code_example_rust = `fn main() {
    // Print text to the console.
    println!("Hello World!");
}`

const code_example_toml = `[package]
name = "ide_example_rust"
version.workspace = true
edition.workspace = true
repository.workspace = true
authors.workspace = true
readme.workspace = true
publish.workspace = true

[dependencies]`
