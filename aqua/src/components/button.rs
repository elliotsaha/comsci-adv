use gloo_console::log; // browser logging
use yew::prelude::*;
use yew::{classes, function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or(String::from("primary"))] // default option
    pub color: String, // "primary" | "secondary"
    #[prop_or(false)]
    pub disabled: bool, // false | true
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

    html! {
            <button
    class={classes!(css)}
                onclick={Callback::from(|_| log!("Hello World"))}
                disabled={props.disabled}
            >
                {props.children.clone()}
            </button>
    }
}
