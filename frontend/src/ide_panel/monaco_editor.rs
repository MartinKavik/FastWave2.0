pub use js_bridge::MonacoEditorController;
use zoon::*;

pub struct MonacoEditor {
    raw_el: RawHtmlEl<web_sys::HtmlElement>,
    controller: Mutable<Option<SendWrapper<js_bridge::MonacoEditorController>>>,
    task_with_controller: Mutable<Option<TaskHandle>>,
}

impl Element for MonacoEditor {}

impl RawElWrapper for MonacoEditor {
    type RawEl = RawHtmlEl<web_sys::HtmlElement>;
    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

impl Styleable<'_> for MonacoEditor {}
impl KeyboardEventAware for MonacoEditor {}
impl MouseEventAware for MonacoEditor {}
impl PointerEventAware for MonacoEditor {}
impl TouchEventAware for MonacoEditor {}
impl AddNearbyElement<'_> for MonacoEditor {}
impl HasIds for MonacoEditor {}

impl MonacoEditor {
    pub fn new() -> Self {
        let controller: Mutable<Option<SendWrapper<js_bridge::MonacoEditorController>>> =
            Mutable::new(None);
        let task_with_controller = Mutable::new(None);
        Self {
            controller: controller.clone(),
            task_with_controller: task_with_controller.clone(),
            raw_el: El::new()
                .s(RoundedCorners::all(10))
                .s(Clip::both())
                .after_insert(clone!((controller) move |element| {
                    Task::start(async move {
                        let excalidraw_controller = SendWrapper::new(js_bridge::MonacoEditorController::new());
                        excalidraw_controller.init(&element).await;
                        controller.set(Some(excalidraw_controller));
                    });
                }))
                .after_remove(move |_| {
                    drop(task_with_controller);
                })
                .into_raw_el(),
        }
    }

    pub fn task_with_controller<FUT: Future<Output = ()> + 'static>(
        self,
        f: impl FnOnce(Mutable<Option<SendWrapper<js_bridge::MonacoEditorController>>>) -> FUT,
    ) -> Self {
        self.task_with_controller
            .set(Some(Task::start_droppable(f(self.controller.clone()))));
        self
    }
}

mod js_bridge {
    use zoon::*;

    // Note: Add all corresponding methods to `frontend/typescript/monaco_editor/monaco_editor.tsx`
    #[wasm_bindgen(module = "/typescript/bundles/monaco_editor.js")]
    extern "C" {
        #[derive(Clone)]
        pub type MonacoEditorController;

        #[wasm_bindgen(constructor)]
        pub fn new() -> MonacoEditorController;

        #[wasm_bindgen(method)]
        pub async fn init(this: &MonacoEditorController, parent_element: &JsValue);

        // #[wasm_bindgen(method)]
        // pub fn draw_diagram_element(this: &MonacoEditorController, excalidraw_element: JsValue);

        // #[wasm_bindgen(method)]
        // pub fn listen_for_component_text_changes(
        //     this: &MonacoEditorController,
        //     component_id: &str,
        //     on_change: &Closure<dyn Fn(String)>,
        // );

        // #[wasm_bindgen(method)]
        // pub fn set_component_text(this: &MonacoEditorController, component_id: &str, text: &str);
    }
}
