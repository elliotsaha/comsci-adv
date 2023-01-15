use yew::prelude::*;
use yew::{
    classes, function_component, html, virtual_dom::AttrValue, Callback, Children, Html, Properties,
};

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    // AttrValue is cheap to clone compared to String
    #[prop_or(AttrValue::from("primary"))] // default option
    pub color: AttrValue, // "primary" | "secondary"
    #[prop_or(false)]
    pub disabled: bool, // false | true
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>, // function that is called when button is pressed
    #[prop_or_default]
    pub children: Children, // text inside of button
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    // prefix for button css classes
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
