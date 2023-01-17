use yew::prelude::*;
use yew::{classes, function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct GridProps {
    #[prop_or(false)]
    pub container: bool, // whether grid container or not
    // default to grid item because it is used more frequently than container
    #[prop_or(true)]
    pub item: bool, // whether grid item (under container) or not
    #[prop_or_default]
    pub xs: u8, // how many columns should item take up (1 - 12) initially
    #[prop_or_default]
    pub sm: u8, // how many columns should item take up (1 - 12) from sm - md breakpoints
    #[prop_or_default]
    pub md: u8, // how many columns should item take up (1 - 12) from md - lg breakpoints
    #[prop_or_default]
    pub lg: u8, // how many columns should item take up (1 - 12) after lg breakpoint
    #[prop_or_default]
    pub justify_content: AttrValue, // justifyContent css option for container
    #[prop_or_default]
    pub align_items: AttrValue, // alignItems css option for container
    // how much space between items should there be
    #[prop_or(AttrValue::from("none"))]
    pub gutter: AttrValue, // "none" | "sm" | "md" | "lg"
    #[prop_or_default]
    pub class: String, // custom css class from user
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Grid(props: &GridProps) -> Html {
    // prefix for grid css classes
    let prefix = "grid";

    // initially includes column specific class and gutter class
    let mut css = vec![
        format!("{prefix}-gutter-{}", props.gutter),
        // breakpoint classes
        format!("{prefix}-cols-xs-{}", props.xs),
        format!("{prefix}-cols-sm-{}", props.sm),
        format!("{prefix}-cols-md-{}", props.md),
        format!("{prefix}-cols-lg-{}", props.lg),
        // flexbox container props
        format!("{prefix}-justify-content-{}", props.justify_content),
        format!("{prefix}-align-items-{}", props.align_items),
    ];

    // handle container class
    if props.container {
        css.push(format!("{prefix}-container"));
    } else {
        // if not container, assume grid item (default)
        css.push(format!("{prefix}-item"));
    }

    html! {
        <div class={classes!(css, &props.class)}>{props.children.clone()}</div>
    }
}
