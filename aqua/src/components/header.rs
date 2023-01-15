use yew::prelude::*;
use yew::{classes, function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct HeaderProps {
    // String must be used here instead of AttrValue because dynamic
    // tag names do not support AttrValue
    #[prop_or(String::from("h1"))] // default to h1
    pub size: String, // "h1" | "h2" | "h3" | "h4" | 'h5' | 'h6'
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    // prefix for header css classes
    let prefix = "header";

    // header size class (controls font-size and line-height)
    let css = format!("{prefix}-{}", &props.size);

    html! {
        // dynamic tag name (tag name = props.size)
        <@{props.size.clone()} class={classes!(css)}>
            {props.children.clone()}
        </@>
    }
}
