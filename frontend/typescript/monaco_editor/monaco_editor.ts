// https://github.com/CodinGame/monaco-vscode-api/wiki/Getting-started-guide

import * as monaco from 'monaco-editor';

import { initialize } from 'vscode/services'
import getTextMateServiceOverride from "@codingame/monaco-vscode-textmate-service-override";
import getLanguagesServiceOverride from "@codingame/monaco-vscode-languages-service-override";

export type WorkerLoader = () => Worker;

export class MonacoEditorController {
    constructor() {}

    async init(parent_element: HTMLElement) {
        const workerLoaders: Map<string, WorkerLoader> = new Map();
        workerLoaders.set(
            "TextEditorWorker",
            () => {
                return new Worker(
                    "/_api/public/monaco_editor/web_worker.js",
                    { type: 'module' }
                )
            }
        )
        
        window.MonacoEnvironment = {
          getWorker: function (_workerId, label) {
            console.log("Getting WebWorker with label:", label);
            const workerFactory = workerLoaders.get(label)
            if (workerFactory != null) {
                return workerFactory()
            }
            throw new Error(`Worker '${label}' not found`)
          }
        }

        await initialize({
            // ...getTextMateServiceOverride(),
            ...getLanguagesServiceOverride(),
        });

        monaco.editor.setTheme("vs-dark")
        monaco.editor.create(parent_element, {
            value: code_example_verilog,
            language: "verilog"
        });
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
