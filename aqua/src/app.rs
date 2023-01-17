use crate::components::{Button, Grid, Text};
use gloo_console::log;
use yew::prelude::*;
use yew::Callback;

#[function_component(App)]
pub fn app() -> Html {
    let on_click_handler = Callback::from(move |_: MouseEvent| {
        log!("Hello");
    });

    html! {
        <main>
            <Button disabled={false} on_click={&on_click_handler}>{ "Hello World" }</Button>
            <Text span={true} neutral={true}>{ "Text" }</Text>
            <Grid container={true} xs={12} sm={6} md={3} gutter="none">
                <Grid>{"hi"}</Grid>
                <Grid>{"hi"}</Grid>
                <Grid>{"hi"}</Grid>
                <Grid>{"hi"}</Grid>
            </Grid>
        </main>
    }
}
