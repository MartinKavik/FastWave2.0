// https://github.com/CodinGame/monaco-vscode-api/wiki/Getting-started-guide

import * as monaco from 'monaco-editor';

export type WorkerLoader = () => Worker;

export class MonacoEditorController {
    constructor() {}

    async init(parent_element: HTMLElement) {
        const workerLoaders: Partial<Record<string, WorkerLoader>> = {
            editorWorkerService: () => {
                return new Worker(
                    "/_api/public/monaco_editor/web_worker.js",
                    { type: 'module' }
                )
            }
        }
        
        window.MonacoEnvironment = {
          getWorker: function (_workerId, label) {
            console.log("Getting WebWorker with label:", label);
            const workerFactory = workerLoaders[label]
            if (workerFactory != null) {
                return workerFactory()
            }
            throw new Error(`Worker ${label} not found`)
          }
        }
        
        monaco.editor.create(parent_element, {
            value: "Hello world!",
        });
    }
}
