use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav class="navbar">
            <Link<Route> to={Route::Home} classes={classes!("navbar-link")}>
                {"Home"}
            </Link<Route>>
            <Link<Route> to={Route::About} classes={classes!("navbar-link")}>
                {"About"}
            </Link<Route>>
            <Link<Route> to={Route::Contact} classes={classes!("navbar-link")}>
                {"Contact"}
            </Link<Route>>
        </nav>
    }
}
