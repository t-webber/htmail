use crate::tools::fetch;
use crate::tools::html::GetElement;

use super::logger;

#[derive(PartialEq)]
pub enum PopupSpecies {
    Add,
    Select,
}

impl PopupSpecies {
    fn to_id(&self) -> String {
        match self {
            PopupSpecies::Add => "add",
            PopupSpecies::Select => "select",
        }
        .to_owned()
    }
}

#[derive(PartialEq, yew::Properties)]
struct PopupProps {
    id: String,
    title: String,
    content: yew::Html,
    species: PopupSpecies,
}

#[yew::function_component(Popup)]
fn popup(props: &PopupProps) -> yew::Html {
    let PopupProps {
        id,
        title,
        content,
        species,
    } = props;
    yew::html!(
        <div id={id.clone()} class={format!("popup-wrapper {}", species.to_id())}>
            <div class="popup">
                <h2>{title.clone()}</h2>
                <a class="close" href="#">{"x"}</a>
                <div class="content">
                {content.clone()}
                </div>
            </div>
        </div>
    )
}

#[derive(PartialEq, yew::Properties, Clone, Debug)]
struct Input {
    title: String,
    id: String,
    _info: String,
    placeholder: String,
    intype: String,
}

fn input(inpt: &Input) -> yew::Html {
    let Input {
        title,
        id,
        placeholder,
        intype,
        ..
    } = inpt;
    yew::html!(
        <tr>
            <th class="field-form-title">{title}</th>
            <th>
            <input type={intype.clone()} placeholder={placeholder.clone()} id={id.clone()} />
            </th>
        </tr>
    )
}

fn test(ids: Vec<String>, profile: bool) {
    wasm_bindgen_futures::spawn_local(async move {
        let values = ids
            .clone()
            .into_iter()
            .map(|id| id.get_element_cast::<web_sys::HtmlInputElement>().value())
            .collect::<Vec<_>>();
        if values.iter().all(|x| x.trim().is_empty()) {
            return;
        }
        let success = if profile {
            fetch::add_profile(&values).await
        } else {
            fetch::add_recipient(&values).await
        };
        if success {
            ids.iter().for_each(|id| {
                id.get_element_cast::<web_sys::HtmlInputElement>()
                    .set_value("");
            });
        } else {
            logger::log(&logger::FAILURE, "Failed to write to database");
        }
    });
}

fn add_forms(inputs: &[Input], species: &str) -> yew::Html {
    let ids = inputs
        .iter()
        .map(|input| input.id.clone())
        .collect::<Vec<_>>();
    let lines = inputs.iter().map(input).collect::<Vec<_>>();
    let profile = species == "profile";
    yew::html!(
        <form class="add-forms">
            <table class="field-form">
                {lines}
            </table>
            <a href="#" onclick={
                yew::Callback::from(move |_: yew::MouseEvent| {
                test(ids.clone(), profile);
            })
            }>{"Submit"}</a>
        </form>
    )
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug, Eq)]
pub struct Selection {
    pub name: String,
    pub email: String,
}
impl Ord for Selection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.email.cmp(&other.email)
    }
}
impl PartialOrd for Selection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.email.partial_cmp(&other.email)
    }
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct SelectionVec(pub Vec<Selection>);

impl SelectionVec {
    pub fn push(&mut self, selection: Selection) {
        match self.0.binary_search(&selection) {
            Ok(idx) | Err(idx) => self.0.insert(idx, selection),
        }
    }
}

impl From<Vec<Selection>> for SelectionVec {
    fn from(vec: Vec<Selection>) -> Self {
        let mut selection_vec = SelectionVec::default();
        vec.into_iter()
            .for_each(|selection| selection_vec.push(selection));
        selection_vec
    }
}

fn add_choice(id: &str, content: &str) {
    let to_edit = id.get_element_cast::<web_sys::HtmlTextAreaElement>();
    let value = to_edit.value();
    let trimmed = value.trim();
    let written = if trimmed.is_empty() {
        content.to_owned()
    } else {
        format!("{trimmed}, {content}")
    };
    to_edit.set_value(&written);
}

fn field_choice(choice: &Selection, id: String) -> yew::Html {
    let email = choice.email.to_owned();
    let name = choice.name.to_owned();
    yew::html!(
        <a class="choice" href="#" onclick={yew::Callback::from(move |_| add_choice(&id, &name))}>
            {email}
        </a>
    )
}

fn vec2lines(inputs: &SelectionVec, species: &SelectionSpecies) -> yew::Html {
    yew::html!(
        <div class="field-selection">
        {
        inputs
            .0.iter()
            .map(|selecitem| field_choice(selecitem, species.to_dest()))
            .collect::<yew::Html>()
    }</div>)
}

fn fields_selection(items: &SelectionVec, species: &SelectionSpecies) -> yew::Html {
    yew::html! {
        <div id={species.to_id()}>
            {vec2lines(items, species)}
        </div>
    }
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub enum SelectionSpecies {
    #[default]
    None,
    Profile,
    Recipient,
}

impl SelectionSpecies {
    pub fn to_id(&self) -> String {
        match self {
            Self::Profile => "profile-popup-field-selection",
            Self::Recipient => "recipient-popup-field-selection",
            Self::None => {
                logger::log(
                    &logger::FAILURE,
                    "Unexpected behaviour: found none species in to_id",
                );
                ""
            }
        }
        .to_owned()
    }

    pub fn to_dest(&self) -> String {
        match self {
            Self::Profile => "from-field",
            Self::Recipient => "to-field",
            Self::None => {
                logger::log(
                    &logger::FAILURE,
                    "Unexpected behaviour: found none species in to_dest",
                );
                ""
            }
        }
        .to_owned()
    }
}

#[derive(PartialEq, yew::Properties)]
pub struct WindowsProps {
    pub to_drop_selection: SelectionVec,
    pub from_drop_selection: SelectionVec,
}

#[yew::function_component(Windows)]
pub fn windows(
    WindowsProps {
        to_drop_selection,
        from_drop_selection,
    }: &WindowsProps,
) -> yew::Html {
    let from_add_form = vec![
        Input {
            id: "profilename".to_owned(),
            title: "Profile name".to_owned(),
            _info: "info".to_owned(),
            placeholder: "john-gmail".to_owned(),
            intype: "text".to_owned(),
        },
        Input {
            id: "displayname".to_owned(),
            title: "Display name".to_owned(),
            _info: "b".to_owned(),
            placeholder: "John Doe".to_owned(),
            intype: "text".to_owned(),
        },
        Input {
            id: "email".to_owned(),
            title: "Email address".to_owned(),
            _info: "b".to_owned(),
            placeholder: "john.doe@gmail.com".to_owned(),
            intype: "email".to_owned(),
        },
        Input {
            id: "password".to_owned(),
            title: "SMTP password".to_owned(),
            _info: "b".to_owned(),
            placeholder: "abcd efgh ikjl mnop".to_owned(),
            intype: "password".to_owned(),
        },
    ];
    let to_add_form = vec![
        Input {
            id: "recipientname".to_owned(),
            title: "Name".to_owned(),
            _info: "info".to_owned(),
            placeholder: "john-gmail".to_owned(),
            intype: "text".to_owned(),
        },
        Input {
            id: "recipientemail".to_owned(),
            title: "Email".to_owned(),
            _info: "b".to_owned(),
            placeholder: "john.doe@gmail.com".to_owned(),
            intype: "email".to_owned(),
        },
    ];
    yew::html!(
        <div class="windows-container">
            <Popup id={"from-add-form"} title={"Add a profile"} content={add_forms(&from_add_form, "profile")} species={PopupSpecies::Add} />
            <Popup id={"from-drop-selection"} title={"Select a profile"} content={fields_selection(&from_drop_selection, &SelectionSpecies::Profile)}
                species={PopupSpecies::Select} />

            <Popup id={"to-add-form"} title={"Add a recipient"} content={add_forms(&to_add_form, "recipient")} species={PopupSpecies::Add} />
            <Popup id={"to-drop-selection"} title={"Select a recipient"} content={fields_selection(&to_drop_selection, &SelectionSpecies::Recipient)}
                species={PopupSpecies::Select} />
        </div>
    )
}
