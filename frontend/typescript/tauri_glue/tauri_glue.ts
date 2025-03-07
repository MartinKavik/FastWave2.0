// @TODO use TS and Tauri bindgens to make this code properly typed

import { core, event } from '@tauri-apps/api'

const invoke = core.invoke;
const listen = event.listen;

type Filename = string;
type FolderPath = string;
type FilePath = string;
type JavascriptCode = string;
type WellenHierarchy = unknown;
type Timeline = unknown;
type VarFormat = unknown;

type AddedDecodersCount = number;
type RemovedDecodersCount = number;
type DecoderPath = string;

type AddedDiagramConnectorsCount = number;
type RemovedDiagramConnectorsCount = number;
type DiagramConnectorPath = string;
type DiagramConnectorName = string;
type ComponentId = string;

type FileTreeItem = unknown;

export async function show_window(): Promise<void> {
    return await invoke("show_window");
}

export async function pick_and_load_waveform(): Promise<Filename | undefined> {
    return await invoke("pick_and_load_waveform");
}

export async function load_file_with_selected_vars(): Promise<JavascriptCode | undefined> {
    return await invoke("load_file_with_selected_vars");
}

export async function get_hierarchy(): Promise<WellenHierarchy> {
    return await invoke("get_hierarchy");
}

export async function load_signal_and_get_timeline(
    signal_ref_index: number,
    timeline_zoom: number,
    timeline_viewport_width: number,
    timeline_viewport_x: number,
    block_height: number,
    var_format: VarFormat,
): Promise<Timeline> {
    return await invoke("load_signal_and_get_timeline", {
        signal_ref_index,
        timeline_zoom,
        timeline_viewport_width,
        timeline_viewport_x,
        block_height,
        var_format
    });
}

export async function unload_signal(signal_ref_index: number): Promise<void> {
    return await invoke("unload_signal", { signal_ref_index });
}

export async function send_char(c : string): Promise<void> {
    return await invoke("send_char", { c });
}

export async function add_decoders(decoder_paths: Array<DecoderPath>): Promise<AddedDecodersCount> {
    return await invoke("add_decoders", { decoder_paths });
}

export async function remove_all_decoders(): Promise<RemovedDecodersCount> {
    return await invoke("remove_all_decoders");
}

export async function add_diagram_connectors(diagram_connector_paths: Array<DiagramConnectorPath>): Promise<AddedDiagramConnectorsCount> {
    return await invoke("add_diagram_connectors", { diagram_connector_paths });
}

export async function remove_all_diagram_connectors(): Promise<RemovedDiagramConnectorsCount> {
    return await invoke("remove_all_diagram_connectors");
}

export async function listen_diagram_connectors_messages(on_message: (message: any) => void) {
    return await listen("diagram_connector_message", (message) => on_message(message.payload));
}

export async function listen_term_update(on_message: (message: any) => void) {
    return await listen("term_content", (message) => on_message(message.payload));
}

export async function notify_diagram_connector_text_change(diagram_connector: DiagramConnectorName, component_id: ComponentId, text: string): Promise<void> {
    return await invoke("notify_diagram_connector_text_change", { diagram_connector, component_id, text });
}

export async function open_konata_file() {
    return await invoke("open_konata_file");
}

export async function read_file(path: FilePath): Promise<string> {
    return await invoke("read_file", { path });
}

export async function select_folder_to_open(): Promise<FolderPath | undefined> {
    return await invoke("select_folder_to_open");
}

export async function file_tree(path: FolderPath): Promise<FileTreeItem> {
    return await invoke("file_tree", { path });
}
