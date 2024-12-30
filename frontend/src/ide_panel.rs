use zoon::*;

mod code_editor;
use code_editor::CodeEditor;
pub use code_editor::CodeEditorController;

#[derive(Clone)]
pub struct IdePanel {
    code_editor_controller: Mutable<Mutable<Option<SendWrapper<CodeEditorController>>>>,
}

impl IdePanel {
    pub fn new(
        code_editor_controller: Mutable<Mutable<Option<SendWrapper<CodeEditorController>>>>,
    ) -> impl Element {
        Self { code_editor_controller }.root()
    }

    fn root(&self) -> impl Element {
        Column::new()
            .s(Padding::all(20))
            .s(Scrollbars::y_and_clip_x())
            .s(Width::fill())
            .s(Height::fill())
            .s(Gap::new().y(20))
            .item(self.code_editor())
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
