mod router;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;
use wasm_logger;
use router::{Route, switch};

mod context;
use context::UserContextProvider;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <UserContextProvider>
            <context::CartProvider>
                <BrowserRouter>
                    <div class="flex flex-col min-h-screen">
                        <main class="flex-grow">
                            <Switch<Route> render={switch} />
                        </main>
                        <components::footer::Footer />
                    </div>
                </BrowserRouter>
            </context::CartProvider>
        </UserContextProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
