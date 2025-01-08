use crate::theme::*;
use std::path::PathBuf;
use zoon::*;

mod code_editor;
use code_editor::CodeEditor;
pub use code_editor::CodeEditorController;

use crate::platform;

#[derive(Clone)]
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
        }
        .root(selected_file_path_change_handler)
    }

    fn root(&self, selected_file_path_change_handler: TaskHandle) -> impl Element {
        Row::new()
            .s(Padding::all(20).left(0))
            .s(Scrollbars::y_and_clip_x())
            .s(Width::fill())
            .s(Height::fill())
            .s(Gap::new().y(20))
            .item(
                Column::new()
                    .s(Align::new().top())
                    .s(Scrollbars::y_and_clip_x())
                    .s(Width::exact(300))
                    .s(Padding::new().right(20))
                    .s(Gap::new().y(10))
                    .item(self.open_folder_button())
                    .item(self.file_tree_view()),
            )
            .item(
                Column::new()
                    .s(Scrollbars::y_and_clip_x())
                    .s(Width::fill())
                    .s(Height::fill())
                    .item(self.file_path_input())
                    .item(self.code_editor()),
            )
            .after_remove(move |_| drop(selected_file_path_change_handler))
    }

    fn file_path_input(&self) -> impl Element {
        TextInput::new()
            .s(RoundedCorners::new().top(10))
            .s(Padding::new().x(20).y(6))
            .s(Background::new().color(COLOR_SLATE_BLUE_WITH_ALPHA))
            .s(Font::default().color(COLOR_LIGHT_BLUE))
            .label_hidden("selected file path")
            .text_signal(
                self.selected_file_path
                    .signal_cloned()
                    .map_option(|path| path.to_string_lossy().to_string(), String::new),
            )
            .read_only(true)
    }

    fn file_tree_view(&self) -> impl Element {
        let this = self.clone();
        El::new().child_signal(
            self.selected_folder_path
                .signal_cloned()
                .map_future(move |folder_path| {
                    let this = this.clone();
                    async move {
                        let Some(folder_path) = folder_path else {
                            return None;
                        };
                        let root = platform::file_tree(folder_path).await;
                        Some(this.file_tree_view_item(root))
                    }
                })
                .boxed_local()
                .map(Option::flatten),
        )
    }

    fn file_tree_view_item(&self, item: shared::FileTreeItem) -> impl Element {
        let left_padding = 20;
        let top_padding = 5;
        let inner_padding = Padding::new().x(10).y(3);
        match item {
            shared::FileTreeItem::Folder {
                name,
                path: _,
                children,
            } => Column::with_tag(Tag::Custom("details"))
                .s(Padding::new().left(left_padding).top(top_padding))
                .s(Width::fill())
                .s(Cursor::new(CursorIcon::Pointer))
                .item(
                    El::with_tag(Tag::Custom("summary"))
                        .update_raw_el(|raw_el| raw_el.style("display", "list-item"))
                        .s(inner_padding)
                        .child(name),
                )
                .items(
                    children
                        .into_iter()
                        .map(|item| self.file_tree_view_item(item).unify()),
                )
                .left_either(),
            shared::FileTreeItem::File { name, path } => {
                let is_selected =
                    self.selected_file_path
                        .signal_ref(clone!((path) move |selected_path| {
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
                            .s(Background::new()
                                .color_signal(is_selected.map_true(|| COLOR_SLATE_BLUE_WITH_ALPHA)))
                            .s(Width::default())
                            .on_click(move || {
                                selected_file_path.set_neq(Some(path.clone()));
                            })
                            .child(name),
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

    #[cfg(FASTWAVE_PLATFORM = "TAURI")]
    fn open_folder_button(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        let selected_folder_path = self.selected_folder_path.clone();
        Button::new()
            .s(Padding::new().x(20).y(4))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| COLOR_MEDIUM_SLATE_BLUE, || COLOR_SLATE_BLUE),
            ))
            .s(Align::new().center_x())
            .s(RoundedCorners::all(10))
            .label(El::new().s(Font::new().no_wrap()).child("Open folder.."))
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || {
                let selected_folder_path = selected_folder_path.clone();
                Task::start(async move {
                    if let Some(folder_path) = platform::select_folder_to_open().await {
                        selected_folder_path.set_neq(Some(folder_path.into()));
                    }
                })
            })
    }
}
