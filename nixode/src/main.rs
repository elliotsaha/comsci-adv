use yew::prelude::*;
use yew::{html, Component, Context, Html};

enum Msg {
    AddOne,
}

struct Button {
    count: i64,
}

impl Component for Button {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true // re-render after increment
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <h1>{"Your component: "}{self.count}</h1>
                <button onclick={link.callback(|_| Msg::AddOne )}>{ "+1" }</button>
            </div>
        }
    }
}

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
