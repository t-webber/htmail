#[derive(PartialEq, yew::Properties, Clone)]
pub struct ActionBtn {
    pub condition: bool,
    pub content: yew::Html,
    pub action: yew::Callback<()>,
}

#[derive(PartialEq, yew::Properties, Clone)]
pub struct ActionsProps {
    pub actions: Vec<ActionBtn>,
}

#[yew::function_component]
pub fn Actions(props: &ActionsProps) -> yew::Html {
    let cloned = (*props).clone();
    yew::html! {
        <div class="actions">
            {cloned.actions.into_iter().filter_map(move |ActionBtn {content, action, condition}: ActionBtn| {
                if !condition {
                    None
                } else {
                    Some(yew::html!(
                        <button onclick={move |_| action.emit(())}>{content}</button>
                    ))
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
