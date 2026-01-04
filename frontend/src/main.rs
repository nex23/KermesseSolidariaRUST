mod router;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;
use wasm_logger;
use router::{Route, switch};

mod context;
use context::UserContextProvider;

#[function_component(App)]
fn app() -> Html {
    html! {
        <UserContextProvider>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </UserContextProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
