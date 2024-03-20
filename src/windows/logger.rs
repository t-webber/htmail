use crate::tools::html::GetElement;
use chrono::Timelike;

#[allow(dead_code)]
pub enum LoggingSpecies {
    Warning,
    Failure,
    Success,
}

trait ToID {
    fn to_id(self) -> String;
}

impl ToID for &LoggingSpecies {
    fn to_id(self) -> String {
        match self {
            LoggingSpecies::Warning => "logger-warning".to_owned(),
            LoggingSpecies::Failure => "logger-failure".to_owned(),
            LoggingSpecies::Success => "logger-success".to_owned(),
        }
    }
}

#[allow(dead_code)]
pub const SUCCESS: LoggingSpecies = LoggingSpecies::Success;
pub const WARNING: LoggingSpecies = LoggingSpecies::Warning;
pub const FAILURE: LoggingSpecies = LoggingSpecies::Failure;

pub fn log(species: &LoggingSpecies, message: &str) {
    let local_time = chrono::Local::now();
    let time_string = format!(
        "{}:{}:{}",
        local_time.hour(),
        local_time.minute(),
        local_time.second()
    );

    let elt = "logger".get_element();
    elt.set_inner_html(&format!(
        "{}<div class=\"{}\">&nbsp;[{}]&nbsp;{}.</div>",
        elt.inner_html(),
        species.to_id(),
        time_string,
        message
    ));
}

#[yew::function_component(Logger)]
pub fn logger() -> yew::Html {
    yew::html!(
        <div id="logger"></div>
    )
}
