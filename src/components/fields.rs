use crate::plugins::textarea;
use crate::windows::fieldpopups;

#[derive(Default, yew::Properties)]
pub struct FieldProps {
    pub id: String,
    pub placeholder: String,
    pub add_dest: String,
    pub drop_dest: String,
    pub species: fieldpopups::SelectionSpecies,
    pub msg_parent: yew::Callback<fieldpopups::SelectionSpecies>,
}

impl PartialEq for FieldProps {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.placeholder == other.placeholder
            && self.add_dest == other.add_dest
            && self.drop_dest == other.drop_dest
            && self.species == other.species
    }
}

#[derive(Default)]
pub struct Field;

impl yew::Component for Field {
    type Message = ();
    type Properties = FieldProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let profiles = ctx.props().species == fieldpopups::SelectionSpecies::Profile;
        let callback = ctx.props().msg_parent.clone();
        yew::html! {
           <div class="field">
            <div class="input-wrapper">
            <textarea::Input id={ctx.props().id.clone()} placeholder={ctx.props().placeholder.clone()} ..textarea::ResponsiveTextareaProps::default() />
            </div>
            <div class="input-menus">
            if !ctx.props().drop_dest.is_empty() {
                <a class="dropdown-button" href={ctx.props().drop_dest.clone()} onclick={move |_| callback.emit(
                    if profiles {
                        fieldpopups::SelectionSpecies::Profile
                    } else {
                        fieldpopups::SelectionSpecies::Recipient
                    })
                } >

                <div class="triangle"></div>
                </a>
            }
            if !ctx.props().add_dest.is_empty() {
                <a class="add-button" href={ctx.props().add_dest.clone()}>{"+"}</a>
            }
            </div>
        </div>
        }
    }
}

pub struct FieldSet;

#[derive(Clone, yew::Properties)]
pub struct FieldSetProps {
    pub msg_parent: yew::Callback<fieldpopups::SelectionSpecies>,
}

impl PartialEq for FieldSetProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl yew::Component for FieldSet {
    type Message = ();
    type Properties = FieldSetProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
           <div class="fields-container">
            <Field id={"from-field"} placeholder={""} add_dest={"#from-add-form"} drop_dest={"#from-drop-selection"}
                    species={fieldpopups::SelectionSpecies::Profile} msg_parent={ctx.props().msg_parent.clone()} />
            <Field id={"to-field"} placeholder={"To..."} add_dest={"#to-add-form"} drop_dest={"#to-drop-selection"}
                    species={fieldpopups::SelectionSpecies::Recipient} msg_parent={ctx.props().msg_parent.clone()} />
            <Field id={"subject-field"} placeholder={"Subject..."} ..FieldProps::default()  />
        </div>
        }
    }
}
