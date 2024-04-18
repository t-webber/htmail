use crate::plugins::actions;
use crate::plugins::textarea;
use crate::tools::html::GetElement;
use crate::windows::logger;
use wasm_bindgen::JsCast;

struct ContainerTextArea {
    open: bool,
    other_open: bool,
    msg_parent: yew::Callback<ContainerMsg>,
    content: String,
}

impl From<&ContainerTextAreaProps> for ContainerTextArea {
    fn from(value: &ContainerTextAreaProps) -> Self {
        Self {
            open: value.open,
            other_open: value.other_open,
            msg_parent: value.msg_parent.clone(),
            content: value.content.clone(),
        }
    }
}

enum ContainerTextAreaMsg {}

#[derive(Clone, PartialEq, yew::Properties)]
struct ContainerTextAreaProps {
    open: bool,
    other_open: bool,
    msg_parent: yew::Callback<ContainerMsg>,
    content: String,
}

const EDITOR_ID: &str = "body-textarea-textarea";

impl yew::Component for ContainerTextArea {
    type Message = ContainerTextAreaMsg;
    type Properties = ContainerTextAreaProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self::from(ctx.props())
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        let callback_tab = self.msg_parent.clone();
        let callback_textarea = self.msg_parent.clone();
        let callback_one = self.msg_parent.clone();
        let callback_two = self.msg_parent.clone();
        if self.open {
            yew::html! {
                <div  id="body-textarea"  class="left" >
                    <textarea::ResponsiveTextarea
                        value={self.content.clone()}
                        name="bodyarea"
                        id={EDITOR_ID}
                        placeholder="Enter email body here..."
                        onkeydown={move |event: yew::KeyboardEvent| if event.key() =="Tab" && event.ctrl_key() {
                            event.prevent_default();
                            let textarea = event.target().unwrap().dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                            let start_u32: u32 = textarea.selection_start().unwrap().unwrap_or_default();
                            let end_u32: u32 = textarea.selection_end().unwrap().unwrap_or_default();
                            let start_usize = start_u32 as usize;
                            let end_usize = end_u32 as usize;
                            let value = textarea.value();
                            let new_value = format!("{}\t{}", &value.get(..start_usize).unwrap_or_default(), &value.get(end_usize..).unwrap_or_default());
                            textarea.set_value(&new_value);
                            textarea.set_selection_start(Some(start_u32.checked_add(1).unwrap_or(start_u32))).unwrap_or_else(|err| logger::log(&logger::FAILURE, &format!("Error while writing tab in body: {err:?}")));
                            textarea.set_selection_end(Some(start_u32.checked_add(1).unwrap_or(start_u32))).unwrap_or_else(|err| logger::log(&logger::FAILURE, &format!("Error while writing tab in body: {err:?}")));
                            callback_tab.emit(ContainerMsg::UpdateRender);
                        }}
                        oninput={Some(
                            yew::Callback::<yew::InputEvent>::from(move |_: yew::InputEvent| {
                                callback_textarea.emit(ContainerMsg::UpdateRender);
                        }))} />
                    <actions::ActionList actions={vec![
                        actions::ActionBtn{
                            condition: self.other_open,
                        content: yew::html!( <yew_icons::Icon icon_id={yew_icons::IconId::LucideX} class="action-icon" />),
                        action: yew::Callback::from(move |()| callback_one.emit(ContainerMsg::CloseEditor))
                    },
                    actions::ActionBtn{
                        condition: !self.other_open,
                        content: yew::html!( <yew_icons::Icon icon_id={yew_icons::IconId::LucideView} class="action-icon" />),
                        action: yew::Callback::from(move |()| {
                            callback_two.emit(ContainerMsg::OpenRender);
                                })
                        }
                    ]} />
                </div>
            }
        } else {
            yew::html!()
        }
    }

    fn changed(
        &mut self,
        ctx: &yew::prelude::Context<Self>,
        _old_props: &Self::Properties,
    ) -> bool {
        *self = Self::from(ctx.props());
        true
    }
}

#[derive(Debug)]
struct ContainerRender {
    open: bool,
    other_open: bool,
    msg_parent: yew::Callback<ContainerMsg>,
    content: String,
}

enum ContainerRenderMsg {}

#[derive(Clone, PartialEq, yew::Properties)]
struct ContainerRenderProps {
    open: bool,
    other_open: bool,
    msg_parent: yew::Callback<ContainerMsg>,
    content: String,
}

impl From<&ContainerRenderProps> for ContainerRender {
    fn from(value: &ContainerRenderProps) -> Self {
        Self {
            open: value.open,
            other_open: value.other_open,
            msg_parent: value.msg_parent.clone(),
            content: value.content.clone(),
        }
    }
}

impl yew::Component for ContainerRender {
    type Message = ContainerRenderMsg;
    type Properties = ContainerRenderProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self::from(ctx.props())
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        let callback_open = self.msg_parent.clone();
        let callback_close = self.msg_parent.clone();

        if self.open {
            yew::html! (
                <div class="right" id="body-render-container" >
                <div id="body-render" class="body-render">{self.content.clone()}</div>
                <actions::ActionList actions={vec![
                    actions::ActionBtn{
                        condition: self.other_open,
                        content: yew::html!( <yew_icons::Icon icon_id={yew_icons::IconId::LucideX} class="action-icon" />),
                        action: yew::Callback::from(move |()| {
                            callback_close.emit(ContainerMsg::CloseRender);
                    })
                },
                    actions::ActionBtn{
                        condition: !self.other_open,
                        content: yew::html!( <yew_icons::Icon icon_id={yew_icons::IconId::FeatherEdit3} class="action-icon" />),
                        action: yew::Callback::from(move |()| {
                            callback_open.emit(ContainerMsg::OpenEditor);
                    })
                }]} />
                    </div>
            )
        } else {
            yew::html!()
        }
    }

    fn changed(
        &mut self,
        ctx: &yew::prelude::Context<Self>,
        _old_props: &Self::Properties,
    ) -> bool {
        *self = Self::from(ctx.props());
        true
    }
}

#[derive(Debug)]
pub struct Container {
    render_open: bool,
    editor_open: bool,
    content: String,
}

#[allow(dead_code)]
pub enum ContainerMsg {
    CloseEditor,
    OpenEditor,
    CloseRender,
    OpenRender,
    UpdateRender,
}
#[derive(Clone, PartialEq, Eq, yew::Properties)]
pub struct ContainerProps {
    pub render_open: bool,
    pub editor_open: bool,
    pub content: String,
}

impl Default for ContainerProps {
    fn default() -> Self {
        Self {
            render_open: true,
            editor_open: true,
            content: String::new(),
        }
    }
}

impl yew::Component for Container {
    type Message = ContainerMsg;
    type Properties = ContainerProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            render_open: ctx.props().render_open,
            editor_open: ctx.props().editor_open,
            content: ctx.props().content.clone(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <div class="body-container" >
                <ContainerTextArea open={self.editor_open} msg_parent={ctx.link().callback(|msg| msg)} other_open={self.render_open} content={self.content.clone()} />
                <ContainerRender open={self.render_open} msg_parent={ctx.link().callback(|msg| msg)} content={self.content.clone()} other_open={self.editor_open} />
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ContainerMsg::CloseEditor => self.editor_open = false,
            ContainerMsg::OpenEditor => self.editor_open = true,

            ContainerMsg::CloseRender => self.render_open = false,

            ContainerMsg::OpenRender => {
                self.render_open = true;
                ctx.link().send_message(ContainerMsg::UpdateRender);
            }
            ContainerMsg::UpdateRender => {
                self.content = EDITOR_ID
                    .get_element_cast::<web_sys::HtmlTextAreaElement>()
                    .value();
            }
        }
        true
    }
}
