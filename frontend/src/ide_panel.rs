use zoon::*;

mod monaco_editor;
use monaco_editor::MonacoEditor;
pub use monaco_editor::MonacoEditorController;

#[derive(Clone)]
pub struct IdePanel {
    monaco_editor_controller: Mutable<Mutable<Option<SendWrapper<MonacoEditorController>>>>,
}

impl IdePanel {
    pub fn new(
        monaco_editor_controller: Mutable<Mutable<Option<SendWrapper<MonacoEditorController>>>>,
    ) -> impl Element {
        Self { monaco_editor_controller }.root()
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
        let monaco_editor_controller = self.monaco_editor_controller.clone();
        MonacoEditor::new()
            .s(Align::new().top())
            .s(Width::fill())
            .s(Height::fill())
            .task_with_controller(move |controller| {
                monaco_editor_controller.set(controller.clone());
                async {}
            })
    }
}
