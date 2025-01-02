use std::{path::PathBuf, str::FromStr};

use zoon::*;

mod code_editor;
use code_editor::CodeEditor;
pub use code_editor::CodeEditorController;

// #[derive(Clone)]
pub struct IdePanel {
    code_editor_controller: Mutable<Mutable<Option<SendWrapper<CodeEditorController>>>>,
    selected_file_path: Mutable<Option<PathBuf>>,
    #[allow(dead_code)]
    selected_file_path_change_handler: TaskHandle,
}

impl IdePanel {
    pub fn new(
        code_editor_controller: Mutable<Mutable<Option<SendWrapper<CodeEditorController>>>>,
    ) -> impl Element {
        let selected_file_path = Mutable::new(None::<PathBuf>);

        let selected_file_path_change_handler = Task::start_droppable(
            selected_file_path.signal_cloned().for_each_sync(clone!((code_editor_controller) move |path| {
                let path = path.map(|path| path.into_os_string().into_string().unwrap_throw());
                if let Some(controller) = code_editor_controller.lock_ref().lock_ref().as_ref() {
                    controller.set_selected_file_path(path);
                }
            }))
        );

        Self { 
            code_editor_controller,
            selected_file_path,
            selected_file_path_change_handler,
        }.root()
    }

    fn root(&self) -> impl Element {
        Column::new()
            .s(Padding::all(20))
            .s(Scrollbars::y_and_clip_x())
            .s(Width::fill())
            .s(Height::fill())
            .s(Gap::new().y(20))
            .item(self.file_path_input())
            .item(self.code_editor())
    }

    fn file_path_input(&self) -> impl Element {
        let selected_file_path = self.selected_file_path.clone();
        let input_file_path = Mutable::new(String::new());
        TextInput::new()
            .label_hidden("file path")
            .on_change(clone!((input_file_path) move |new_text| {
                input_file_path.set(new_text);
            }))
            .on_key_down_event(move |event| {
                event.if_key(Key::Enter, || {
                    let input_file_path = PathBuf::from_str(&input_file_path.lock_ref()).unwrap_throw();
                    selected_file_path.set(Some(input_file_path));
                    zoon::println!("New path set!");
                });
            })
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
