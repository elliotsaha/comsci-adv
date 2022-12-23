use yew::prelude::*;
use yew::{html, Html};

pub mod components;

use components::Button;

#[function_component(App)]
fn app() -> Html {
    html! {
        <body>
            <Button />
        </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
