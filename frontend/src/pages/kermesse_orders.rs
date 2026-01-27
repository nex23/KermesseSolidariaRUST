use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::organizer_orders::OrganizerOrders;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub kermesse_id: i32,
}

#[function_component(KermesseOrders)]
pub fn kermesse_orders(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();

    html! {
        <div class="min-h-screen bg-gray-50 p-4 md:p-8">
            <div class="container mx-auto max-w-5xl">
                <button 
                    onclick={Callback::from(move |_| navigator.back())}
                    class="mb-6 text-gray-600 hover:text-gray-900 font-medium flex items-center gap-2"
                >
                    <span>{"←"}</span> { "Volver al Panel" }
                </button>
                
                <div class="bg-white rounded-xl shadow-lg p-6">
                    <h1 class="text-2xl font-bold text-gray-800 mb-6 border-b pb-4">{ "Gestión de Pedidos" }</h1>
                    <OrganizerOrders kermesse_id={props.kermesse_id} />
                </div>
            </div>
        </div>
    }
}
