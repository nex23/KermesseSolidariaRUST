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
    #[at("/checkout")]
    Checkout,
    #[at("/my-orders")]
    MyOrders,
    #[at("/kermesses/:id/edit")]
    EditKermesse { id: i32 },
    #[at("/collaborator-dashboard")]
    CollaboratorDashboard,
    #[at("/kermesses/:id/orders")]
    KermesseOrders { id: i32 },
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
        Route::Checkout => html! { <crate::pages::checkout::Checkout /> },
        Route::MyOrders => html! { <crate::pages::my_orders::MyOrders /> },
        Route::EditKermesse { id } => html! { <crate::pages::edit_kermesse::EditKermesse kermesse_id={id} /> },
        Route::CollaboratorDashboard => html! { <crate::pages::collaborator_dashboard::CollaboratorDashboard /> },
        Route::KermesseOrders { id } => html! { <crate::pages::kermesse_orders::KermesseOrders kermesse_id={id} /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}
