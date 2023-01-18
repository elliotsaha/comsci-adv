use crate::components::{Header, Text};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    #[prop_or_default]
    pub class: String, // custom css class from user
    #[prop_or(String::from(""))] // initialize to empty string
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    html! {
        <div class="home-card br-lg">
            <Header size="h6" class="home-card-header">{props.title.clone()}</Header>
            <Text class="home-card-text">
                    { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
            </Text>
        </div>
    }
}
