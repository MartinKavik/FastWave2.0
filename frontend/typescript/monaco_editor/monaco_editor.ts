import './style.css';
import * as monaco from 'monaco-editor';
// import { initialize as initializeEditorWorker } from 'monaco-editor/esm/vs/editor/editor.worker';

export class MonacoEditorController {
    constructor() {}

    async init(parent_element: HTMLElement) {
        // window.MonacoEnvironment = {
        //     getWorker
        //     getWorkerUrl: function (moduleId, label) {
        //         return './vs/editor/editor.worker.js';
        //     }
        //     // getWorker(_workerId: any, _label: string) {
        //     //     const worker = new initializeEditorWorker();
        //     //     console.log(worker);
        //     //     return worker;
        //     // }
        // };
        monaco.editor.create(parent_element, {
            value: "Hello world!"
        });
    }
}
