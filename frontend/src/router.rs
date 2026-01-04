use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{home::Home, kermesse_detail::KermesseDetail, auth::{Login, Register}, dashboard::Dashboard, add_dish::AddDish};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/kermesses/:id")]
    KermesseDetail { id: i32 },
    #[at("/kermesses/:id/add-dish")]
    AddDish { id: i32 },
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/dashboard")]
    Dashboard,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::KermesseDetail { id } => html! { <KermesseDetail id={id} /> },
        Route::AddDish { id } => html! { <AddDish kermesse_id={id} /> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}
