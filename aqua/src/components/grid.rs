use yew::prelude::*;
use yew::{classes, function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct GridProps {
    #[prop_or(false)]
    pub container: bool, // whether grid container or not
    // default to grid item because it is used more frequently than container
    #[prop_or(true)]
    pub item: bool, // whether grid item (under container) or not
    #[prop_or(12)]
    pub cols: u8, // how many columns should item take up (1 - 12)
    // how much space between items should there be
    #[prop_or(AttrValue::from("none"))]
    pub gutter: AttrValue, // "none" | "sm" | "md" | "lg"
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Grid(props: &GridProps) -> Html {
    // prefix for grid css classes
    let prefix = "grid";

    // grid classnames vector
    // initially includes column specific class and gutter class
    let mut css = vec![
        format!("{prefix}-cols-{}", props.cols),
        format!("{prefix}-gutter-{}", props.gutter),
    ];

    // handle container class
    if props.container {
        css.push(format!("{prefix}-container"));
    } else {
        // if not container, assume grid item (default)
        css.push(format!("{prefix}-item"));
    }

    html! {
        <div class={classes!(css)}>{props.children.clone()}</div>
    }
}
