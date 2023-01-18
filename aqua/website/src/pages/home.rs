use crate::app::Route;
use crate::components::{Button, Card, Grid, Header, Text};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Home() -> Html {
    let card_titles = [
        "Robust Infrastructure",
        "Unique Design",
        "Lasting Impact",
        "Business Costs",
    ];

    html! {
        <>
            // Homepage banner
            <div class="home-banner">
                <Grid container={true} xs={6} align_items="center" class="home-grid-container-banner">
                    <Grid>
                        <div class="home-text m-9">
                            <Header size="h1">{"Make Your Website Stand Out"}</Header>
                            <Text class="mt-3 mb-7">
                            { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas et enim est. Nullam justo enim, porta nec urna et, tempus luctus nibh. Sed cursus." }
                            </Text>
                            <Link<Route> to={Route::Contact}>
                                <Button class="mr-5">{"Contact"}</Button>
                            </Link<Route>>
                            <Link<Route> to={Route::About}>
                                <Button color="secondary">{"Read about us"}</Button>
                            </Link<Route>>
                        </div>
                    </Grid>
                    <Grid>
                        <img src="public/home.jpg" alt="banner" class="br-lg"/>
                    </Grid>
                </Grid>
            </div>

            // Card Screen
            <div class="home-card-title-center">
                <Header size="h2">{"Make your dream business today"}</Header>
                <Text>
                                { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas et enim est. Nullam justo enim, porta nec urna et, tempus luctus nibh. Sed cursus." }
                </Text>
            </div>
            <div class="py-8 home-cards">
                {
                    card_titles.into_iter().map(|title| {
                        html! {
                            <Card title={title} />
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}
