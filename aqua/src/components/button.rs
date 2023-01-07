use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or(String::from("primary"))]
    pub color: String,
    pub children: Children,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    html! {
        <button>{props.children.clone()}</button>
    }
}
