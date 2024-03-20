use crate::plugins::textarea::{ResponsiveTextarea, ResponsiveTextareaProps};

#[derive(yew::Properties, PartialEq, Default)]
struct FieldProps {
    id: String,
    placeholder: String,
    add_dest: String,
    drop_dest: String,
}

#[yew::function_component(Field)]
fn field(
    FieldProps {
        id,
        placeholder,
        add_dest,
        drop_dest,
    }: &FieldProps,
) -> yew::Html {
    yew::html!(
        <div class="field">
            <div class="input-wrapper">
            <ResponsiveTextarea id={id.to_owned()} placeholder={placeholder.to_owned()} ..ResponsiveTextareaProps::default()/>
            </div>
            <div class="input-menus">
            if !drop_dest.is_empty() {
                <a class="dropdown-button" href={drop_dest.to_owned()} >
                <div class="triangle"></div>
                </a>
            }
            if !add_dest.is_empty() {
                <a class="add-button" href={add_dest.to_owned()}>{"+"}</a>
            }
            </div>
        </div>
    )
}

#[yew::function_component(Fields)]
pub fn fields() -> yew::Html {
    yew::html!(
        <div class="fields-container">
            <Field id={"from-field"} placeholder={"From..."} add_dest={"#from-add-form"} drop_dest={"#from-drop-selection"} />
            <Field id={"to-field"} placeholder={"To..."} add_dest={"#to-add-form"} drop_dest={"#to-drop-selection"} />
            <Field id={"subject-field"} placeholder={"Subject..."} ..FieldProps::default()  />
        </div>
    )
}
