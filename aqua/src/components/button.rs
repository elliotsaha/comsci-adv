use yew::prelude::*;
use yew::{classes, function_component, html, Callback, Children, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or(String::from("primary"))] // default option
    pub color: String, // "primary" | "secondary"
    #[prop_or(false)]
    pub disabled: bool, // false | true
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    // prefix for all css classes
    let prefix = "btn";

    // css class list
    let mut css = vec![format!("{prefix}-{}", props.color.clone())];

    // disabled prop
    if props.disabled {
        css.push(format!("{prefix}-disabled"));
    }

    // need to clone on_click closure so props struct is not consumed on call
    let on_click_callback = props.on_click.clone();

    html! {
            <button
                class={classes!(css)}
                disabled={props.disabled}
                onclick={move |e: MouseEvent| on_click_callback.emit(e)}
            >
                {props.children.clone()}
            </button>
    }
}
