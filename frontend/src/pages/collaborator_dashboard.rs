use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use crate::router::Route;
use crate::context::UserContext;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize)]
struct CollaboratedKermesse {
    id: i32,
    name: String,
    role: String,
    event_date: String,
}

#[function_component(CollaboratorDashboard)]
pub fn collaborator_dashboard() -> Html {
    let navigator = use_navigator().unwrap();
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let kermesses = use_state(|| Vec::<CollaboratedKermesse>::new());
    let loading = use_state(|| true);

    {
        let kermesses = kermesses.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        use_effect_with((), move |_| {
            if let Some(user) = &user_ctx.user {
                let token = user.token.clone();
                let kermesses = kermesses.clone();
                let loading = loading.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    // TODO: create this endpoint or use existing logic
                    // For now, we might need a new endpoint `GET /my-collaborations`
                    // Or reuse `GET /kermesses` and filter? No, privacy.
                    // Let's assume we create `GET /my-collaborations` in backend.
                    
                    let url = "http://127.0.0.1:8080/my-collaborations"; 
                    if let Ok(resp) = Request::get(url)
                        .header("Authorization", &format!("Bearer {}", token))
                        .send()
                        .await 
                    {
                        if let Ok(data) = resp.json::<Vec<CollaboratedKermesse>>().await {
                            kermesses.set(data);
                        }
                    }
                    loading.set(false);
                });
            } else {
                loading.set(false);
            }
            || ()
        });
    }
    
    // Logic to select a kermesse and view orders
    // For simplicity, we can just navigate to a "Manage Orders" page for that kermesse
    // e.g. /kermesses/:id/orders
    
    html! {
        <div class="min-h-screen bg-gray-50 p-6 font-sans">
            <div class="container mx-auto max-w-4xl">
                <h1 class="text-3xl font-bold text-gray-800 mb-8 border-b pb-4">{ "Mi Panel de Colaborador" }</h1>
                
                if *loading {
                     <div class="text-center py-10">{ "Cargando tus eventos..." }</div>
                } else if kermesses.is_empty() {
                     <div class="bg-white rounded-xl shadow p-10 text-center">
                        <p class="text-xl text-gray-600 mb-4">{ "No eres colaborador en ninguna kermesse activa." }</p>
                        <button onclick={Callback::from(move |_| navigator.push(&Route::Home))} class="bg-primary text-white px-6 py-2 rounded-lg hover:bg-orange-600">
                            { "Buscar Eventos para Unirte" }
                        </button>
                     </div>
                } else {
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {
                            kermesses.iter().map(|k| {
                                let id = k.id;
                                let format_navigator = navigator.clone();
                                html! {
                                    <div class="bg-white rounded-xl shadow-md overflow-hidden hover:shadow-lg transition border border-gray-100">
                                        <div class="p-6">
                                            <h3 class="text-xl font-bold text-gray-800 mb-2 truncate">{ &k.name }</h3>
                                            <div class="flex items-center gap-2 mb-4 text-sm text-gray-500">
                                                <span>{"📅 "}{ &k.event_date }</span>
                                                <span class="px-2 py-0.5 rounded bg-blue-100 text-blue-800 font-bold text-xs">
                                                    { 
                                                        match k.role.as_str() {
                                                            "SELLER" => "Vendedor",
                                                            "KITCHEN" => "Cocina",
                                                            "DELIVERY" => "Delivery",
                                                            "INGREDIENT_GETTER" => "Acopio",
                                                            _ => &k.role
                                                        }
                                                    }
                                                </span>
                                            </div>
                                            <button 
                                                onclick={Callback::from(move |_| format_navigator.push(&Route::KermesseOrders { id }))}
                                                class="w-full bg-secondary text-white font-bold py-2 rounded-lg hover:bg-teal-600 transition flex items-center justify-center gap-2"
                                            >
                                                <span>{"📋"}</span> { "Gestionar Pedidos" }
                                            </button>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                }
            </div>
        </div>
    }
}
