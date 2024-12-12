import './style.css'
import * as monaco from 'monaco-editor';

export type WorkerLoader = () => Worker;

export class MonacoEditorController {
    constructor() {}

    async init(parent_element: HTMLElement) {
        const workerLoaders: Partial<Record<string, WorkerLoader>> = {
            TextEditorWorker: () => new Worker(
                new URL('monaco-editor/esm/vs/editor/editor.worker.js', import.meta.url), 
                { type: 'module' }
            )
        }
        
        window.MonacoEnvironment = {
          getWorker: function (_workerId, label) {
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
