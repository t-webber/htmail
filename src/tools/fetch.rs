use crate::windows::{fieldpopups, logger};

use super::html::GetElement;

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue;
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SendMailArgs<'fetch> {
    to: &'fetch str,
    from: &'fetch str,
    subject: &'fetch str,
    body: &'fetch str,
}

pub fn send_mail() {
    let from = "from-field"
        .get_element_cast::<web_sys::HtmlTextAreaElement>()
        .value();

    let to = "to-field"
        .get_element_cast::<web_sys::HtmlTextAreaElement>()
        .value();

    let subject = "subject-field"
        .get_element_cast::<web_sys::HtmlTextAreaElement>()
        .value();

    let body = "body-textarea-textarea"
        .get_element_cast::<web_sys::HtmlTextAreaElement>()
        .value();

    let args = match serde_wasm_bindgen::to_value(&SendMailArgs {
        to: &to,
        from: &from,
        subject: &subject,
        body: &body,
    }) {
        Ok(val) => val,
        Err(err) => {
            logger::log(
                &logger::FAILURE,
                &format!("Error serializing email: {err:?}"),
            );
            return;
        }
    };

    wasm_bindgen_futures::spawn_local(async move {
        let invoke_result = invoke("send_mail", args)
            .await
            .as_string()
            .unwrap_or_else(|| "Unexpected error: found None!".to_owned());
        logger::log(&logger::FAILURE, &invoke_result);
    });
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AddProfileArgs<'fetch> {
    profilename: &'fetch str,
    displayname: &'fetch str,
    email: &'fetch str,
    smtppass: &'fetch str,
}

#[macro_export]
macro_rules! get_log {
    ($($args:tt), *) => {
        if get_log($($args), *).is_empty() {
            return false;
        } else {
            get_log($($args), *)
        }
    };
}

fn get_log<'fetch_func>(
    values: &'fetch_func [String],
    index: usize,
    name: &'fetch_func str,
) -> &'fetch_func str {
    values.get(index).map_or_else(
        || {
            logger::log(
                &logger::FAILURE,
                &format!("Field {name} was found empty but is required"),
            );
            ""
        },
        |val| val,
    )
}

pub async fn add_profile(values: &[String]) -> bool {
    let args = match serde_wasm_bindgen::to_value(&AddProfileArgs {
        profilename: crate::get_log!(values, 0, "profile name"),
        displayname: crate::get_log!(values, 1, "display name"),
        email: crate::get_log!(values, 2, "email"),
        smtppass: crate::get_log!(values, 3, "smtp password"),
    }) {
        Ok(val) => val,
        Err(err) => {
            logger::log(
                &logger::FAILURE,
                &format!("Error serializing email: {err:?}"),
            );
            return false;
        }
    };

    // wasm_bindgen_futures::spawn_local(async move {
    let invoke_result = invoke("add_profile", args).await.as_string();
    match invoke_result {
        Some(err) if !err.is_empty() => logger::log(
            &logger::FAILURE,
            &format!("Error occured while adding profile: {err}"),
        ),
        Some(_) => logger::log(&logger::SUCCESS, "Profile was successfully added"),
        None => logger::log(&logger::WARNING, "Unexpected behaviour"),
    }
    // });

    true
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AddRecipientArgs<'fetch> {
    name: &'fetch str,
    email: &'fetch str,
}

pub async fn add_recipient(values: &[String]) -> bool {
    let args: wasm_bindgen::prelude::JsValue =
        match serde_wasm_bindgen::to_value(&AddRecipientArgs {
            name: get_log!(values, 0, "name"),
            email: get_log!(values, 1, "email"),
        }) {
            Ok(val) => val,
            Err(err) => {
                logger::log(
                    &logger::FAILURE,
                    &format!("Error serializing email: {err:?}"),
                );
                return false;
            }
        };

    // wasm_bindgen_futures::spawn_local(async move {
    let invoke_result = invoke("add_recipient", args).await.as_string();
    match invoke_result {
        Some(err) if !err.is_empty() => logger::log(
            &logger::FAILURE,
            &format!("Error occured while adding recipient: {err}"),
        ),
        Some(_) => logger::log(&logger::SUCCESS, "Recipient was successfully added"),
        None => logger::log(&logger::WARNING, "Unexpected behaviour"),
    }

    true
}

pub async fn get_profiles() -> fieldpopups::SelectionVec {
    let invoke_result = invoke("get_profiles", wasm_bindgen::JsValue::NULL).await;
    // web_sys::console::log_1(&invoke_result);
    let stringified = invoke_result.as_string().unwrap_or_default();
    let result: Vec<fieldpopups::Selection> = serde_json::from_str(&stringified).unwrap();
    fieldpopups::SelectionVec::from(result)
}
pub async fn get_recipients() -> fieldpopups::SelectionVec {
    let invoke_result = invoke("get_recipients", wasm_bindgen::JsValue::NULL).await;
    // web_sys::console::log_1(&invoke_result);
    let stringified = invoke_result.as_string().unwrap_or_default();
    let result: Vec<fieldpopups::Selection> = serde_json::from_str(&stringified).unwrap();
    fieldpopups::SelectionVec::from(result)
}
