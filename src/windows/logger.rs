use crate::tools::html::GetElement;
use chrono::Timelike;

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum LoggingSpecies {
    Warning,
    Failure,
    Success,
    Debug,
}

impl LoggingSpecies {
    const fn to_id(&self) -> &'static str {
        match *self {
            Self::Warning => "logger-warning",
            Self::Failure => "logger-failure",
            Self::Success | Self::Debug => "logger-success",
        }
    }
}

pub const SUCCESS: LoggingSpecies = LoggingSpecies::Success;
pub const WARNING: LoggingSpecies = LoggingSpecies::Warning;
pub const FAILURE: LoggingSpecies = LoggingSpecies::Failure;
pub const DEBUG: LoggingSpecies = LoggingSpecies::Debug;

pub fn log(species: &LoggingSpecies, message: &str) {
    let local_time = chrono::Local::now();
    let time_string = if *species == DEBUG {
        format!(
            "[{}:{}:{}]&nbsp;",
            local_time.hour(),
            local_time.minute(),
            local_time.second()
        )
    } else {
        String::new()
    };

    let elt = "logger".get_element();
    elt.set_inner_html(&format!(
        "{}<div class=\"{}\">&nbsp;{}{}.</div>",
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
