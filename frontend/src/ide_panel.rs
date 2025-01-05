use std::path::PathBuf;
use zoon::*;
use crate::theme::*;

mod code_editor;
use code_editor::CodeEditor;
pub use code_editor::CodeEditorController;

use crate::platform;

pub struct IdePanel {
    code_editor_controller: Mutable<Mutable<Option<SendWrapper<CodeEditorController>>>>,
    selected_file_path: Mutable<Option<PathBuf>>,
    selected_folder_path: Mutable<Option<PathBuf>>,
}

impl IdePanel {
    pub fn new(
        code_editor_controller: Mutable<Mutable<Option<SendWrapper<CodeEditorController>>>>,
        selected_folder_path: Mutable<Option<PathBuf>>,
    ) -> impl Element {
        let selected_file_path = Mutable::new(None::<PathBuf>);

        let selected_file_path_change_handler = Task::start_droppable(
            selected_file_path.signal_cloned().for_each(clone!((code_editor_controller) move |path| {
                clone!((code_editor_controller) async move {
                    let path = path.map(|path| path.into_os_string().into_string().unwrap_throw());
                    if let Some(controller) = code_editor_controller.lock_ref().lock_ref().as_ref() {
                        let content = if let Some(path) = &path {
                            match platform::read_file(path).await {
                                Ok(content) => content,
                                Err(error) => { 
                                    zoon::eprintln!("Failed to load file '{path}': {error:#}");
                                    String::new()
                                }
                            }
                        } else {
                            String::new()
                        };
                        controller.set_selected_file(path, content);
                    }
                })
            }))
        );

        Self { 
            code_editor_controller,
            selected_file_path,
            selected_folder_path,
        }.root(selected_file_path_change_handler)
    }

    fn root(&self, selected_file_path_change_handler: TaskHandle) -> impl Element {
        Row::new()
            .s(Padding::all(20).left(0))
            .s(Scrollbars::y_and_clip_x())
            .s(Width::fill())
            .s(Height::fill())
            .s(Gap::new().y(20))
            .item(self.file_tree_view())
            .item(
                Column::new()
                    .s(Scrollbars::y_and_clip_x())
                    .s(Width::fill())
                    .s(Height::fill())
                    .item(self.file_path_input())
                    .item(self.code_editor())
            )
            .after_remove(move |_| {
                drop(selected_file_path_change_handler)
            })
    }

    fn file_path_input(&self) -> impl Element {
        TextInput::new()
            .s(RoundedCorners::new().top(10))
            .s(Padding::new().x(20).y(6))
            .s(Background::new().color(COLOR_SLATE_BLUE_WITH_ALPHA))
            .s(Font::default().color(COLOR_LIGHT_BLUE))
            .label_hidden("selected file path")
            .text_signal(
                self
                    .selected_file_path
                    .signal_cloned()
                    .map_option(|path| path.to_string_lossy().to_string(), String::new)
            )
            .read_only(true)
    }

    fn file_tree_view(&self) -> impl Element {
        let test_root = FileTreeItem::new_folder(
            "D:/repos/FastWave2.0/test_files/ide".into(),
            vec![
                FileTreeItem::new_folder(
                    "D:/repos/FastWave2.0/test_files/ide/ide_example_rust".into(),
                    vec![
                        FileTreeItem::new_folder(
                            "D:/repos/FastWave2.0/test_files/ide/ide_example_rust/src".into(),
                            vec![
                                FileTreeItem::new_file("D:/repos/FastWave2.0/test_files/ide/ide_example_rust/src/main.rs".into())
                            ]
                        ),
                        FileTreeItem::new_file("D:/repos/FastWave2.0/test_files/ide/ide_example_rust/Cargo.toml".into())
                    ]
                ),
                FileTreeItem::new_folder(
                    "D:/repos/FastWave2.0/test_files/ide/ide_example_verilog".into(),
                    vec![
                        FileTreeItem::new_file("D:/repos/FastWave2.0/test_files/ide/ide_example_verilog/example.v".into())
                    ]
                )
            ]
        );

        zoon::println!("{test_root:#?}");

        El::new()
            .s(Align::new().top())
            .s(Width::exact(300))
            .s(Padding::new().right(20))
            .child(self.file_tree_view_item(test_root))
    }

    fn file_tree_view_item(&self, item: FileTreeItem) -> impl Element {
        let left_padding = 20;
        let top_padding = 5;
        let inner_padding = Padding::new().x(10).y(3);
        match item {
            FileTreeItem::Folder { name, path, children } => {
                Column::with_tag(Tag::Custom("details"))
                    .s(Padding::new().left(left_padding).top(top_padding))
                    .s(Width::fill())
                    .s(Cursor::new(CursorIcon::Pointer))
                    .item(
                        El::with_tag(Tag::Custom("summary"))
                            .update_raw_el(|raw_el| {
                                raw_el.style("display", "list-item")
                            })
                            .s(inner_padding)
                            .child(name)
                    )
                    .items(children.into_iter().map(|item| {
                        self.file_tree_view_item(item).unify()
                    }))
                    .left_either()
            }
            FileTreeItem::File { name, path } => {
                let is_selected = self.selected_file_path.signal_ref(clone!((path) move |selected_path| {
                    if let Some(selected_path) = selected_path.as_ref() {
                        return selected_path == &path
                    }
                    false
                }));
                let selected_file_path = self.selected_file_path.clone();
                El::new()
                    .s(Padding::new().left(left_padding).top(top_padding))
                    .s(Width::fill())
                    .s(Cursor::new(CursorIcon::Pointer))
                    .child(
                        El::new()
                            .s(inner_padding)
                            .s(RoundedCorners::all(10))
                            .s(Background::new().color_signal(is_selected.map_true(|| COLOR_SLATE_BLUE_WITH_ALPHA)))
                            .s(Width::default())
                            .on_click(move || {
                                selected_file_path.set_neq(Some(path.clone()));
                            })
                            .child(name)
                    )
                    .right_either()
            }
        }
    }

    fn code_editor(&self) -> impl Element {
        let code_editor_controller = self.code_editor_controller.clone();
        CodeEditor::new()
            .s(Align::new().top())
            .s(Width::fill())
            .s(Height::fill())
            .s(Scrollbars::both())
            .task_with_controller(move |controller| {
                code_editor_controller.set(controller.clone());
                async {}
            })
    }
}

#[derive(Debug)]
enum FileTreeItem {
    Folder { name: String, path: PathBuf, children: Vec<FileTreeItem> },
    File { name: String, path: PathBuf }
}

impl FileTreeItem {
    pub fn new_folder(path: PathBuf, children: Vec<FileTreeItem>) -> Self {
        Self::Folder { name: path.file_name().unwrap().to_string_lossy().to_string(), path, children }
    }

    pub fn new_file(path: PathBuf) -> Self {
        Self::File { name: path.file_name().unwrap().to_string_lossy().to_string(), path }
    }
}
