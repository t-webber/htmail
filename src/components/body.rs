use crate::plugins::actions;
use crate::plugins::textarea;
use crate::tools::html::GetElement;
use crate::windows::logger;
use wasm_bindgen::JsCast;

///[BodyTextArea]
struct BodyTextArea {
    open: bool,
    other_open: bool,
    msg_parent: yew::Callback<BodyMsg>,
    content: String,
}

impl From<&BodyTextAreaProps> for BodyTextArea {
    fn from(value: &BodyTextAreaProps) -> Self {
        Self {
            open: value.open,
            other_open: value.other_open,
            msg_parent: value.msg_parent.clone(),
            content: value.content.clone(),
        }
    }
}

enum BodyTextAreaMsg {}

#[derive(Clone, PartialEq, yew::Properties)]
struct BodyTextAreaProps {
    open: bool,
    other_open: bool,
    msg_parent: yew::Callback<BodyMsg>,
    content: String,
}

const EDITOR_ID: &str = "body-textarea-textarea";

impl yew::Component for BodyTextArea {
    type Message = BodyTextAreaMsg;
    type Properties = BodyTextAreaProps;

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
                            let _ = textarea.set_selection_start(Some((start_u32 + 1) as u32));
                            let _ = textarea.set_selection_end(Some((start_u32 + 1) as u32));
                            callback_tab.emit(BodyMsg::UpdateRender);
                        }}
                        oninput={Some(
                            yew::Callback::<yew::InputEvent>::from(move |_: yew::InputEvent| {
                                callback_textarea.emit(BodyMsg::UpdateRender);
                        }))} />
                    <actions::Actions actions={vec![
                        actions::ActionBtn{
                            condition: self.other_open,
                        content: yew::html!( <yew_icons::Icon icon_id={yew_icons::IconId::LucideX} class="action-icon" />),
                        action: yew::Callback::from(move |_| callback_one.emit(BodyMsg::CloseEditor))
                    },
                    actions::ActionBtn{
                        condition: !self.other_open,
                        content: yew::html!( <yew_icons::Icon icon_id={yew_icons::IconId::LucideView} class="action-icon" />),
                        action: yew::Callback::from(move |_| {
                            callback_two.emit(BodyMsg::OpenRender);
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

///[BodyRender]
#[derive(Debug)]
struct BodyRender {
    open: bool,
    other_open: bool,
    msg_parent: yew::Callback<BodyMsg>,
    content: String,
}

enum BodyRenderMsg {}

#[derive(Clone, PartialEq, yew::Properties)]
struct BodyRenderProps {
    open: bool,
    other_open: bool,
    msg_parent: yew::Callback<BodyMsg>,
    content: String,
}

impl From<&BodyRenderProps> for BodyRender {
    fn from(value: &BodyRenderProps) -> Self {
        Self {
            open: value.open,
            other_open: value.other_open,
            msg_parent: value.msg_parent.clone(),
            content: value.content.clone(),
        }
    }
}

impl yew::Component for BodyRender {
    type Message = BodyRenderMsg;
    type Properties = BodyRenderProps;

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
                <actions::Actions actions={vec![
                    actions::ActionBtn{
                        condition: self.other_open,
                        content: yew::html!( <yew_icons::Icon icon_id={yew_icons::IconId::LucideX} class="action-icon" />),
                        action: yew::Callback::from(move |_| {
                            callback_close.emit(BodyMsg::CloseRender);
                    })
                },
                    actions::ActionBtn{
                        condition: !self.other_open,
                        content: yew::html!( <yew_icons::Icon icon_id={yew_icons::IconId::FeatherEdit3} class="action-icon" />),
                        action: yew::Callback::from(move |_| {
                            callback_open.emit(BodyMsg::OpenEditor);
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
pub struct Body {
    render_open: bool,
    editor_open: bool,
    content: String,
}

#[allow(dead_code)]
pub enum BodyMsg {
    CloseEditor,
    OpenEditor,
    CloseRender,
    OpenRender,
    UpdateRender,
}
#[derive(Clone, PartialEq, yew::Properties)]
pub struct BodyProps {
    pub render_open: bool,
    pub editor_open: bool,
    pub content: String,
}

impl Default for BodyProps {
    fn default() -> Self {
        Self {
            render_open: true,
            editor_open: true,
            content: String::new(),
        }
    }
}

impl yew::Component for Body {
    type Message = BodyMsg;
    type Properties = BodyProps;

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
                <BodyTextArea open={self.editor_open} msg_parent={ctx.link().callback(|msg| msg)} other_open={self.render_open} content={self.content.clone()} />
                <BodyRender open={self.render_open} msg_parent={ctx.link().callback(|msg| msg)} content={self.content.clone()} other_open={self.editor_open} />
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BodyMsg::CloseEditor => self.editor_open = false,
            BodyMsg::OpenEditor => self.editor_open = true,

            BodyMsg::CloseRender => self.render_open = false,

            BodyMsg::OpenRender => {
                self.render_open = true;
                ctx.link().send_message(BodyMsg::UpdateRender);
            }
            BodyMsg::UpdateRender => {
                self.content = EDITOR_ID
                    .get_element_cast::<web_sys::HtmlTextAreaElement>()
                    .value();
            }
        }
        true
    }
}
