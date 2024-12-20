// https://davidmyers.dev/blog/how-to-build-a-code-editor-with-codemirror-6-and-typescript/introduction

// @TODO Rewrite to Rust? (https://crates.io/crates/codemirror)

import { EditorState, Compartment } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import { basicSetup } from 'codemirror'
import { oneDark } from '@codemirror/theme-one-dark'
import { StreamLanguage } from "@codemirror/language"
import { languageServer } from 'codemirror-languageserver';

import { rust } from "@codemirror/lang-rust"

import { verilog } from "@codemirror/legacy-modes/mode/verilog"
import { scala } from "@codemirror/legacy-modes/mode/clike"

// @TODO rename `monaco_editor` to code_editor / codemirror

export class MonacoEditorController {
    constructor() {}

    async init(parent_element: HTMLElement) {
        const filename = "hello.rs";

        const ls = languageServer({
            // WebSocket server uri and other client options.
            serverUri: 'ws://localhost:9999',
            rootUri: 'file:///',
            workspaceFolders: [],
        
            // Alternatively, to share the same client across multiple instances of this plugin.
            // client: new LanguageServerClient({
            //     serverUri,
            //     rootUri: 'file:///'
            // }),
        
            documentUri: `file:///${filename}`,
            languageId: 'rust' // As defined at https://microsoft.github.io/language-server-protocol/specification#textDocumentItem.
        });

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

        // @TODO remove
        view.dispatch({
            effects: language.reconfigure(StreamLanguage.define(scala))
        })

        // @TODO remove
        view.dispatch({
            changes: [
                { from: 0, to: view.state.doc.length },
                { from: 0, insert: code_example_scala },
            ]
        })

        // @TODO remove
        view.dispatch({
            effects: language.reconfigure(rust())
        })

        // @TODO remove
        view.dispatch({
            changes: [
                { from: 0, to: view.state.doc.length },
                { from: 0, insert: code_example_rust },
            ]
        })

        // @TODO remove
        console.log("CONTENT: ", view.state.doc.toString())
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

const code_example_scala = `package examples

/** Quick sort, imperative style */
object sort {

  /** Nested methods can use and even update everything
   *  visible in their scope (including local variables or
   *  arguments of enclosing methods).
   */
  def sort(a: Array[Int]) {

    def swap(i: Int, j: Int) {
      val t = a(i); a(i) = a(j); a(j) = t
    }

    def sort1(l: Int, r: Int) {
      val pivot = a((l + r) / 2)
      var i = l
      var j = r
      while (i <= j) {
        while (a(i) < pivot) i += 1
        while (a(j) > pivot) j -= 1
        if (i <= j) {
          swap(i, j)
          i += 1
          j -= 1
        }
      }
      if (l < j) sort1(l, j)
      if (j < r) sort1(i, r)
    }

    if (a.length > 0)
      sort1(0, a.length - 1)
  }

  def println(ar: Array[Int]) {
    def print1 = {
      def iter(i: Int): String =
        ar(i) + (if (i < ar.length-1) "," + iter(i+1) else "")
      if (ar.length == 0) "" else iter(0)
    }
    Console.println("[" + print1 + "]")
  }

  def main(args: Array[String]) {
    val ar = Array(6, 2, 8, 5, 1)
    println(ar)
    sort(ar)
    println(ar)
  }

}
`;

const code_example_rust = `fn main() {
    // Print text to the console.
    println!("Hello World!");
}`
