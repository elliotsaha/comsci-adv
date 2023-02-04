use crate::components::Header;
use yew::prelude::*;

#[function_component]
pub fn About() -> Html {
    html! {
        <Header size="h1" class="mt-9">{"about"}</Header>
    }
}