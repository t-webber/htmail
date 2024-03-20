use crate::tools::html::GetElement;
use crate::windows::logger;
use wasm_bindgen::JsCast;

#[derive(yew::Properties, PartialEq)]
struct PopupProps {
    id: String,
    title: String,
    content: yew::Html,
}

#[yew::function_component(Popup)]
fn popup(PopupProps { id, title, content }: &PopupProps) -> yew::Html {
    yew::html!(
        <div id={id.clone()} class="popup-wrapper">
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

#[derive(Clone)]
struct Input {
    title: String,
    id: String,
    _info: String,
    placeholder: String,
    intype: String,
}

// fn input(
//     Input {
//         title,
//         id,
//         _info,
//         placeholder,
//         intype,
//     }: Input,
//     buffer: &mut String,
// ) -> yew::Html {
//     let current = id
//         .get_element()
//         .dyn_into::<web_sys::HtmlInputElement>()
//         .unwrap();
//     yew::html!(
//         <tr>
//             <th class="field-form-title">{title}</th>
//             <th>
//             <input type={intype.to_owned()} placeholder={placeholder.to_owned()} id={id.to_owned()} oninput={yew::Callback::from(move |_| {(*buffer)  = current.value()})} />
//             </th>
//         </tr>
//     )
// }
// use std::cell::RefCell;
// use std::rc::Rc;

fn input(
    Input {
        title,
        id,
        _info,
        placeholder,
        intype,
    }: Input,
) -> yew::Html {
    yew::html!(
        <tr>
            <th class="field-form-title">{title}</th>
            <th>
            <input type={intype.to_owned()} placeholder={placeholder.to_owned()} id={id.to_owned()} />
            </th>
        </tr>
    )
}
// fn add_forms(inputs: Vec<Input>) -> yew::Html {
//     let mut global_buffer = vec![];
//     let lines: Vec<yew::virtual_dom::VNode> = inputs
//         .into_iter()
//         .map(|inpt| {
//             let mut buffer = String::new();
//             let node = input(inpt, &mut buffer);
//             global_buffer.push(buffer);
//             node
//         })
//         .collect();
//     yew::html!(
//         <form class="add-forms">
//             <table class="field-form">
//                 {lines}
//             </table>
//             <a href="#" onclick={move |_| {logger::log(&logger::SUCCESS, &format!("G = {:?}", &global_buffer))}}>{"Submit"}</a>
//         </form>
//     )
// }

fn add_forms(inputs: Vec<Input>) -> yew::Html {
    let lines = inputs
        .clone()
        .into_iter()
        .map(|inpt| input(inpt.clone()))
        .collect::<Vec<_>>();
    let ids = inputs
        .clone()
        .into_iter()
        .map(|input| input.id)
        .collect::<Vec<_>>();
    yew::html!(
        <form class="add-forms">
            <table class="field-form">
                {lines}
            </table>
            <a href="#" onclick={yew::Callback::from(move |_: yew::MouseEvent| {
                    let value = ids.clone().into_iter().map(|id| {
                            id
                            .get_element()
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .unwrap().value()
                    }).collect::<Vec<_>>();
                    logger::log(&logger::SUCCESS, &format!("G = {:?}", &value))
            })
            }>{"Submit"}</a>
        </form>
    )
}

struct Selection<'a> {
    name: &'a str,
    email: &'a str,
}

fn add_choice(id: String, content: &str) {
    let to_edit = id
        .get_element()
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap();
    let value = to_edit.value();
    let trimmed = value.trim();
    let written = if trimmed.chars().last().unwrap().is_ascii_alphabetic() {
        format!("{trimmed} {}", content)
    } else {
        format!("{trimmed}; {}", content)
    };
    to_edit.set_value(&written)
}

fn field_choice(choice: Selection, id: String) -> yew::Html {
    let email = choice.email.to_owned();
    let name = choice.name.to_owned();
    yew::html!(
        <button class="choice" onclick={yew::Callback::from(move |_| add_choice(id.clone(), &email))}>
            {name}
        </button>
    )
}

fn fields_selection(inputs: Vec<Selection>, id: &str) -> yew::Html {
    let lines: Vec<yew::virtual_dom::VNode> = inputs
        .into_iter()
        .map(|inputs| field_choice(inputs, id.to_owned()))
        .collect();
    yew::html!(
        <div class="field-selection">
            {lines}
        </div>
    )
}

#[yew::function_component(Windows)]
pub fn windows() -> yew::Html {
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
            id: "pasword".to_owned(),
            title: "SMTP password".to_owned(),
            _info: "b".to_owned(),
            placeholder: "abcd efgh ikjl mnop".to_owned(),
            intype: "password".to_owned(),
        },
    ];
    let from_drop_selection = vec![
        Selection {
            name: "Bob",
            email: "john.doe@gmail.com",
        },
        Selection {
            name: "Joe",
            email: "john.doe@gmail.com",
        },
    ];
    let to_add_form = vec![
        Input {
            id: "recipient_name".to_owned(),
            title: "Name".to_owned(),
            _info: "info".to_owned(),
            placeholder: "john-gmail".to_owned(),
            intype: "text".to_owned(),
        },
        Input {
            id: "email".to_owned(),
            title: "Email".to_owned(),
            _info: "b".to_owned(),
            placeholder: "john.doe@gmail.com".to_owned(),
            intype: "email".to_owned(),
        },
    ];
    let to_drop_selection = vec![
        Selection {
            name: "Bob",
            email: "john.doe@gmail.com",
        },
        Selection {
            name: "Joe",
            email: "john.doe@gmail.com",
        },
    ];
    yew::html!(
        <div class="windows-container">
            <Popup id={"from-add-form"} title={"Add a profile"} content={add_forms(from_add_form)} />
            <Popup id={"from-drop-selection"} title={"Select a profile"} content={fields_selection(from_drop_selection, "from-field")} />
            <Popup id={"to-add-form"} title={"Add a recipient"} content={add_forms(to_add_form)} />
            <Popup id={"to-drop-selection"} title={"Select a recipient"} content={fields_selection(to_drop_selection, "to-field")} />
        </div>
    )
}
