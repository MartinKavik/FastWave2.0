use once_cell::sync::Lazy;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, RwLock as StdRwLock};
use std::time::Duration;
use tauri::Emitter;
use tauri::{async_runtime::RwLock, AppHandle};
use tauri_plugin_dialog::DialogExt;
use tokio::fs::read_to_string;
use tokio::time::sleep;
use wasmtime::AsContextMut;
use wellen::simple::Waveform;

type Filename = String;
type FolderPath = String;
type JavascriptCode = String;

type AddedDecodersCount = usize;
type RemovedDecodersCount = usize;
type DecoderPath = String;

type AddedDiagramConnectorsCount = usize;
type RemovedDiagramConnectorsCount = usize;
type DiagramConnectorPath = String;
type DiagramConnectorName = String;
type ComponentId = String;
use alacritty_terminal::event::Notify;
use shared::term::{TerminalDownMsg, TerminalScreen};

mod aterm;
mod component_manager;
mod terminal_size;
use std::sync::Mutex;

pub static APP_HANDLE: Lazy<Arc<StdRwLock<Option<AppHandle>>>> = Lazy::new(<_>::default);
pub static WAVEFORM: Lazy<StdRwLock<Arc<RwLock<Option<Waveform>>>>> = Lazy::new(<_>::default);

static TERM: Lazy<Mutex<aterm::ATerm>> =
    Lazy::new(|| Mutex::new(aterm::ATerm::new().expect("Failed to initialize ATerm")));

#[derive(Default)]
struct Store {
    waveform: Arc<RwLock<Option<Waveform>>>,
    val: Arc<RwLock<bool>>,
}

#[tauri::command(rename_all = "snake_case")]
async fn show_window(window: tauri::Window) {
    window.show().unwrap();
}

#[tauri::command(rename_all = "snake_case")]
async fn pick_and_load_waveform(
    store: tauri::State<'_, Store>,
    app: tauri::AppHandle,
) -> Result<Option<Filename>, ()> {
    let Some(file_path) = app.dialog().file().blocking_pick_file() else {
        return Ok(None);
    };
    let file_buf = file_path.into_path().unwrap();
    let file_str = file_buf.as_os_str().to_str().unwrap();
    // @TODO `read` should accept `Path` instead of `&str`
    let waveform = wellen::simple::read(file_str);
    let Ok(waveform) = waveform else {
        panic!("Waveform file reading failed")
    };
    *store.waveform.write().await = Some(waveform);
    *WAVEFORM.write().unwrap() = Arc::clone(&store.waveform);
    Ok(Some(
        file_buf.file_name().unwrap().to_string_lossy().to_string(),
    ))
}

#[tauri::command(rename_all = "snake_case")]
async fn load_file_with_selected_vars(app: tauri::AppHandle) -> Result<Option<JavascriptCode>, ()> {
    let Some(file_path) = app.dialog().file().blocking_pick_file() else {
        return Ok(None);
    };
    // @TODO Tokio's `fs` or a Tauri `fs`?
    let Ok(javascript_code) = fs::read_to_string(file_path.into_path().unwrap()) else {
        panic!("Selected vars file reading failed")
    };
    Ok(Some(javascript_code))
}

#[tauri::command(rename_all = "snake_case")]
async fn get_hierarchy(store: tauri::State<'_, Store>) -> Result<serde_json::Value, ()> {
    let waveform_lock = store.waveform.read().await;
    let waveform = waveform_lock.as_ref().unwrap();
    let hierarchy = waveform.hierarchy();
    Ok(serde_json::to_value(hierarchy).unwrap())
}

#[tauri::command(rename_all = "snake_case")]
async fn load_signal_and_get_timeline(
    signal_ref_index: usize,
    timeline_zoom: f64,
    timeline_viewport_width: u32,
    timeline_viewport_x: i32,
    block_height: u32,
    var_format: shared::VarFormat,
    store: tauri::State<'_, Store>,
) -> Result<serde_json::Value, ()> {
    // @TODO run (all?) in a blocking thread?
    let signal_ref = wellen::SignalRef::from_index(signal_ref_index).unwrap();
    let mut waveform_lock = store.waveform.write().await;
    let waveform = waveform_lock.as_mut().unwrap();
    waveform.load_signals_multi_threaded(&[signal_ref]);
    let signal = waveform.get_signal(signal_ref).unwrap();
    let time_table = waveform.time_table();
    let timeline = shared::signal_to_timeline(
        signal,
        time_table,
        timeline_zoom,
        timeline_viewport_width,
        timeline_viewport_x,
        block_height,
        var_format,
        |mut value: String| {
            Box::pin(async {
                // We need to spawn a (non-runtime-specific?) blocking task before calling component methods to prevent this error:
                // "Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks."
                // @TODO Workaround? Is it a problem only for non-Rust components? Is it needed only when there is a problem in the component (e.g. "`Err` value: wasm trap: cannot enter component instance"?)
                // let value = std::thread::spawn(move || {
                // futures::executor::block_on(async move {
                let decoders = component_manager::decoders::DECODERS.read().await;
                let mut store_lock = component_manager::decoders::STORE.lock().await;
                let mut store = store_lock.as_context_mut();

                for decoder in decoders.iter() {
                    value = decoder
                        .component_decoder_decoder()
                        .call_format_signal_value(&mut store, &value)
                        // @TODO Resolve panic when running non-Rust components:
                        // `Err` value: wasm trap: cannot enter component instance
                        // https://github.com/bytecodealliance/wasmtime/issues/8670 ?
                        .unwrap()
                }
                // value
                // })
                // }).join().unwrap();
                value
            })
        },
    )
    .await;
    Ok(serde_json::to_value(timeline).unwrap())
}

#[tauri::command(rename_all = "snake_case")]
async fn unload_signal(signal_ref_index: usize, store: tauri::State<'_, Store>) -> Result<(), ()> {
    let signal_ref = wellen::SignalRef::from_index(signal_ref_index).unwrap();
    let mut waveform_lock = store.waveform.write().await;
    let waveform = waveform_lock.as_mut().unwrap();
    waveform.unload_signals(&[signal_ref]);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn send_char(c: String) -> Result<(), ()> {
    if c.len() == 1 {
        let term = TERM.lock().unwrap();
        term.tx.notify(c.into_bytes());
        Ok(())
    } else {
        Err(())
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn add_decoders(decoder_paths: Vec<DecoderPath>) -> Result<AddedDecodersCount, ()> {
    Ok(component_manager::decoders::add_decoders(decoder_paths).await)
}

#[tauri::command(rename_all = "snake_case")]
async fn remove_all_decoders() -> Result<RemovedDecodersCount, ()> {
    Ok(component_manager::decoders::remove_all_decoders().await)
}

#[tauri::command(rename_all = "snake_case")]
async fn add_diagram_connectors(
    diagram_connector_paths: Vec<DiagramConnectorPath>,
) -> Result<AddedDiagramConnectorsCount, ()> {
    Ok(
        component_manager::diagram_connectors::add_diagram_connectors(diagram_connector_paths)
            .await,
    )
}

#[tauri::command(rename_all = "snake_case")]
async fn remove_all_diagram_connectors() -> Result<RemovedDiagramConnectorsCount, ()> {
    Ok(component_manager::diagram_connectors::remove_all_diagram_connectors().await)
}

#[tauri::command(rename_all = "snake_case")]
async fn notify_diagram_connector_text_change(
    diagram_connector: DiagramConnectorName,
    component_id: ComponentId,
    text: String,
) -> Result<(), ()> {
    Ok(
        component_manager::diagram_connectors::notify_diagram_connector_text_change(
            diagram_connector,
            component_id,
            text,
        )
        .await,
    )
}

#[tauri::command(rename_all = "snake_case")]
async fn open_konata_file(app: tauri::AppHandle) {
    let Some(file_path) = app.dialog().file().blocking_pick_file() else {
        return;
    };
    let file_str = file_path
        .into_path()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let port = 30000;
    let base_url = format!("http://localhost:{port}");
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(1))
        .build()
        .unwrap();

    let mut konata_server_ready = false;

    let is_konata_server_ready = || async {
        client
            .get(format!("{base_url}/status"))
            .send()
            .await
            .is_ok()
    };

    if is_konata_server_ready().await {
        konata_server_ready = true;
    } else {
        spawn_konata_app();
    }

    let mut attempts = 1;
    while !konata_server_ready {
        attempts += 1;
        if attempts > 5 {
            eprintln!("Failed to get Konata server status (5 attempts)");
            return;
        }
        konata_server_ready = is_konata_server_ready().await;
        sleep(Duration::from_secs(1)).await;
    }

    client
        .post(format!("{base_url}/open-konata-file"))
        .json(&serde_json::json!({
            "file_path": file_str
        }))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}

#[cfg(target_family = "windows")]
fn spawn_konata_app() {
    Command::new("cscript")
        .current_dir("../../Konata")
        .arg("konata.vbs")
        .spawn()
        .unwrap();
}

#[cfg(target_family = "unix")]
fn spawn_konata_app() {
    Command::new("sh")
        .current_dir("../../Konata")
        .arg("konata.sh")
        .spawn()
        .unwrap();
}

#[tauri::command(rename_all = "snake_case")]
async fn read_file(path: String) -> Result<String, String> {
    read_to_string(path)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command(rename_all = "snake_case")]
async fn select_folder_to_open(app: tauri::AppHandle) -> Result<Option<FolderPath>, ()> {
    let Some(file_path) = app.dialog().file().blocking_pick_folder() else {
        return Ok(None);
    };
    let path_buf = file_path.into_path().unwrap();
    let folder_path = path_buf.as_os_str().to_str().unwrap();
    Ok(Some(folder_path.to_owned()))
}

#[tauri::command(rename_all = "snake_case")]
async fn file_tree(path: FolderPath) -> Result<shared::FileTreeItem, ()> {
    let root = file_tree_item(path.into());
    Ok(root)
}

fn file_tree_item(path: PathBuf) -> shared::FileTreeItem {
    let metadata = fs::metadata(path.clone()).unwrap();
    if metadata.is_file() {
        shared::FileTreeItem::new_file(path)
    } else {
        let children = fs::read_dir(&path)
            .unwrap()
            .map(|entry| file_tree_item(entry.unwrap().path()));
        shared::FileTreeItem::new_folder(path, children.collect())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // https://github.com/tauri-apps/tauri/issues/8462
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tauri::Builder::default()
        .manage(Store::default())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        // Note: Add all handlers to `frontend/src/tauri_bridge.rs`
        .invoke_handler(tauri::generate_handler![
            show_window,
            pick_and_load_waveform,
            load_file_with_selected_vars,
            get_hierarchy,
            load_signal_and_get_timeline,
            unload_signal,
            send_char,
            add_decoders,
            remove_all_decoders,
            add_diagram_connectors,
            remove_all_diagram_connectors,
            notify_diagram_connector_text_change,
            open_konata_file,
            read_file,
            select_folder_to_open,
            file_tree,
        ])
        .setup(|app| {
            *APP_HANDLE.write().unwrap() = Some(app.handle().to_owned());
            println!("Setting up yay!");

            std::thread::spawn(move || {
                // Simulate emitting a message after a delay
                std::thread::sleep(std::time::Duration::from_secs(1));

                //tart term and send initial update to backend
                if let Some(app_handle) = crate::APP_HANDLE.read().unwrap().clone() {
                    let term = crate::TERM.lock().unwrap();
                    let content = crate::aterm::terminal_instance_to_string(&term);
                    let payload = TerminalScreen {
                        cols: term.cols,
                        rows: term.rows,
                        content: content,
                    };
                    let payload = TerminalDownMsg::FullTermUpdate(payload);
                    let payload = serde_json::json!(payload);
                    app_handle.emit("term_content", payload).unwrap();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
