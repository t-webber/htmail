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

struct Input<'a> {
    title: &'a str,
    id: &'a str,
    _info: &'a str,
    placeholder: &'a str,
    intype: &'a str,
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
use std::cell::RefCell;
use std::rc::Rc;

fn input(
    Input {
        title,
        id,
        _info,
        placeholder,
        intype,
    }: Input,
    buffer: Rc<RefCell<String>>,
) -> yew::Html {
    let current = id
        .get_element()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();
    yew::html!(
        <tr>
            <th class="field-form-title">{title}</th>
            <th>
            <input type={intype.to_owned()} placeholder={placeholder.to_owned()} id={id.to_owned()} oninput={yew::Callback::from(move |_| {
                *buffer.borrow_mut() = current.value();
            })} />
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
    let global_buffer: Vec<Rc<RefCell<String>>> = inputs
        .iter()
        .map(|_| Rc::new(RefCell::new(String::new())))
        .collect();
    let lines: Vec<yew::virtual_dom::VNode> = inputs
        .into_iter()
        .enumerate()
        .map(|(idx, inpt)| {
            let buffer = Rc::clone(&global_buffer[idx]);
            let node = input(inpt, buffer);
            node
        })
        .collect();
    yew::html!(
        <form class="add-forms">
            <table class="field-form">
                {lines}
            </table>
            <a href="#" onclick={move |_| {
                    let values: Vec<String> = global_buffer.iter().map(|b| b.borrow().clone()).collect();
                    logger::log(&logger::SUCCESS, &format!("G = {:?}", &values))
                }
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
            id: "profilename",
            title: "Profile name",
            _info: "info",
            placeholder: "john-gmail",
            intype: "text",
        },
        Input {
            id: "displayname",
            title: "Display name",
            _info: "b",
            placeholder: "John Doe",
            intype: "text",
        },
        Input {
            id: "email",
            title: "Email address",
            _info: "b",
            placeholder: "john.doe@gmail.com",
            intype: "email",
        },
        Input {
            id: "pasword",
            title: "SMTP password",
            _info: "b",
            placeholder: "abcd efgh ikjl mnop",
            intype: "password",
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
            id: "recipient_name",
            title: "Name",
            _info: "info",
            placeholder: "john-gmail",
            intype: "text",
        },
        Input {
            id: "email",
            title: "Email",
            _info: "b",
            placeholder: "john.doe@gmail.com",
            intype: "email",
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
