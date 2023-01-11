use crate::components::Button;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Button disabled={true}>{ "Hello World" }</Button>
        </main>
    }
}
