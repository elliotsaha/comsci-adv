use yew::{classes, function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct TextProps {
    // neutral grey color
    #[prop_or(true)]
    pub neutral: bool,
    // if the text element render as span instead of p tag
    #[prop_or(false)]
    pub span: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Text(props: &TextProps) -> Html {
    // prefix for text css classes
    let prefix = "txt";

    // text color class (may either be txt-neutral or txt-black)
    let css = format!(
        "{prefix}-{}",
        if props.neutral { "neutral" } else { "black" },
    );

    // what DOM element should be rendered (default to p tag)
    let tag = if props.span { "span" } else { "p" };

    html! {
        <@{tag} class={classes!(css)}>{props.children.clone()}</@>
    }
}
