use crate::windows::logger;

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue;
}

pub fn send_mail() {
    logger::log(&logger::WARNING, "Sending mail");
}
