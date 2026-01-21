use yew::prelude::*;
use yew_router::prelude::*;
use crate::context::UserContext;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use crate::router::Route;

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct MySaleResponse {
    pub id: i32,
    pub kermesse_name: String,
    pub event_date: String,
    pub total_amount: f64,
    pub status: String,
    pub payment_method: String,
    pub created_at: String,
}

#[function_component(MyOrders)]
pub fn my_orders() -> Html {
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let orders = use_state(|| Vec::<MySaleResponse>::new());
    let loading = use_state(|| true);
    let navigator = use_navigator().unwrap();

    {
        let orders = orders.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        use_effect_with((), move |_| {
            if let Some(user) = &user_ctx.user {
                let token = user.token.clone();
                let orders = orders.clone();
                let loading = loading.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let url = "http://127.0.0.1:8080/my-orders";
                    let resp = Request::get(url)
                        .header("Authorization", &format!("Bearer {}", token))
                        .send()
                        .await;

                    if let Ok(resp) = resp {
                         if let Ok(data) = resp.json::<Vec<MySaleResponse>>().await {
                             orders.set(data);
                         } else {
                             gloo_console::error!("Error parsing my-orders");
                         }
                    } else {
                         gloo_console::error!("Error fetching my-orders");
                    }
                    loading.set(false);
                });
            } else {
                loading.set(false);
            }
            || ()
        });
    }
    
    // Status Badge Helper
    let status_badge = |status: &str| -> Html {
        let color_class = match status {
            "CONFIRMED" | "PAID" => "bg-green-100 text-green-800",
            "PENDING" | "PENDING_PAYMENT" => "bg-yellow-100 text-yellow-800",
            "DELIVERED" => "bg-blue-100 text-blue-800",
            "CANCELLED" => "bg-red-100 text-red-800",
            _ => "bg-gray-100 text-gray-800",
        };
        html! {
            <span class={format!("px-2 py-1 rounded-full text-xs font-bold {}", color_class)}>
                { status }
            </span>
        }
    };

    if user_ctx.user.is_none() {
         return html! {
             <div class="p-8 text-center">
                 <h2 class="text-xl font-bold mb-4">{"Acceso Restringido"}</h2>
                 <p class="mb-4">{"Debes iniciar sesión para ver tus pedidos."}</p>
                 <button onclick={Callback::from(move |_| navigator.push(&Route::Login))} class="bg-primary text-white py-2 px-6 rounded-lg font-bold">{"Iniciar Sesión"}</button>
             </div>
         }
    }

    html! {
        <div class="min-h-screen bg-gray-50 text-gray-800 font-sans p-6 sm:p-10">
            <h1 class="text-3xl font-bold mb-8 text-primary border-b border-gray-200 pb-4">{ "Mis Pedidos" }</h1>
            
            if *loading {
                <div class="flex justify-center py-10">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
                </div>
            } else if orders.is_empty() {
                <div class="text-center py-12 bg-white rounded-xl shadow-md">
                    <span class="text-6xl mb-4 block">{"🍽️"}</span>
                    <h3 class="text-xl font-bold text-gray-700 mb-2">{ "Aún no has realizado pedidos" }</h3>
                    <p class="text-gray-500 mb-6">{ "Ve a la sección de eventos y pide algo delicioso." }</p>
                    <button onclick={Callback::from(move |_| navigator.push(&Route::Home))} class="bg-secondary text-white py-3 px-8 rounded-lg font-bold shadow-lg hover:bg-teal-600 transition">
                        { "Explorar Eventos" }
                    </button>
                </div>
            } else {
                <div class="bg-white rounded-xl shadow-lg overflow-hidden">
                    <div class="overflow-x-auto">
                        <table class="w-full text-left border-collapse">
                            <thead>
                                <tr class="bg-gray-100 text-gray-600 text-sm uppercase tracking-wider">
                                    <th class="p-4 font-semibold">{ "ID" }</th>
                                    <th class="p-4 font-semibold">{ "Evento" }</th>
                                    <th class="p-4 font-semibold">{ "Fecha" }</th>
                                    <th class="p-4 font-semibold text-right">{ "Total" }</th>
                                    <th class="p-4 font-semibold text-center">{ "Estado" }</th>
                                    // <th class="p-4 font-semibold text-center">{ "Detalles" }</th>
                                </tr>
                            </thead>
                            <tbody class="divide-y divide-gray-100">
                                {
                                    orders.iter().map(|order| {
                                        html! {
                                            <tr class="hover:bg-gray-50 transition">
                                                <td class="p-4 font-mono text-gray-500 font-bold">{ format!("#{}", order.id) }</td>
                                                <td class="p-4 font-medium text-gray-900">{ &order.kermesse_name }</td>
                                                <td class="p-4 text-gray-500 text-sm">{ &order.event_date }</td>
                                                <td class="p-4 text-right font-bold text-gray-800">{ format!("Bs. {:.2}", order.total_amount) }</td>
                                                <td class="p-4 text-center">{ status_badge(&order.status) }</td>
                                                // <td class="p-4 text-center">
                                                //     <button class="text-blue-600 hover:text-blue-800 font-bold text-sm">{ "Ver" }</button>
                                                // </td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                }
                            </tbody>
                        </table>
                    </div>
                </div>
            }
        </div>
    }
}
