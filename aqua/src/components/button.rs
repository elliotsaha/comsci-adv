use yew::{classes, function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or(String::from("primary"))]
    pub color: String, // primary | secondary
    pub children: Children,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    html! {
        <button
            class={classes!(
                    "btn",
                    "px-5",
                    "py-3",
                    "br-pill",
                    format!("{}-bg", props.color.clone())
                )}
        >
            {props.children.clone()}
        </button>
    }
}
