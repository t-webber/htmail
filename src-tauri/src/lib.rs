#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::question_mark_used)]
// BAD
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::print_stderr)]
#![allow(clippy::use_debug)]

mod db;
mod mail;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn send_mail(from: &str, to: &str, subject: &str, body: &str) -> String {
    eprintln!("IN: {from} => {to} => {subject} =>\n{body}\n!!!");
    match mail::send(from, to, subject, body) {
        Ok(_) => String::new(),
        Err(err) => err,
    }
}

#[tauri::command]
fn add_profile(profilename: &str, displayname: &str, email: &str, smtppass: &str) -> String {
    eprintln!("Adding profile {profilename} {displayname} {email} {smtppass}");
    match db::create_profile(
        profilename.to_owned(),
        displayname.to_owned(),
        email.to_owned(),
        smtppass.to_owned(),
    ) {
        Ok(()) => String::new(),
        Err(err) => format!("ERROR : {err}"),
    }
}
#[tauri::command]
fn add_recipient(name: &str, email: &str) -> String {
    eprintln!("Adding recipient {name} || {email}.");
    match db::create_recipient(name.to_owned(), email.to_owned()) {
        Ok(()) => String::new(),
        Err(err) => format!("ERROR : {err}"),
    }
}

#[derive(serde::Serialize, Debug)]
struct FrontDropDownSelection {
    name: String,
    email: String,
}

#[tauri::command]
fn get_profiles() -> String {
    eprintln!("Getting profiles.");
    let res = db::read_profiles()
        .into_iter()
        .map(|(name, cred)| FrontDropDownSelection {
            name,
            email: cred.email,
        })
        .collect::<Vec<_>>();
    eprintln!("Profiles: {res:?}");
    serde_json::to_string(&res).unwrap_or_else(|_| String::new())
}

#[tauri::command]
fn get_recipients() -> String {
    eprintln!("Getting recipients.");
    let res = db::read_recipients()
        .into_iter()
        .map(|(name, email)| FrontDropDownSelection { name, email })
        .collect::<Vec<_>>();
    eprintln!("Recipients: {res:?}");
    serde_json::to_string(&res).unwrap_or_else(|_| String::new())
}

#[inline]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(clippy::str_to_string)]
    if let Err(err) = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            send_mail,
            add_profile,
            add_recipient,
            get_recipients,
            get_profiles
        ])
        .run(tauri::generate_context!())
    {
        eprintln!(
            "\n-------------------------------------------------------\n
             \n>>>>>>>>>> Error while running Tauri: {err}. <<<<<<<<<<\n
             \n-------------------------------------------------------\n"
        );
    }
}
