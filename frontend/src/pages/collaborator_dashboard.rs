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
        <div class="min-h-screen bg-gray-50 font-sans pb-12">
            // Header / Navbar Placeholder
            <div class="bg-white shadow-sm border-b border-gray-100 py-4 px-6 mb-8 flex items-center justify-between">
                <button onclick={Callback::from(move |_| navigator.push(&Route::Home))} class="flex items-center text-gray-500 hover:text-teal-600 font-bold transition">
                    <span class="mr-2">{"←"}</span> { "Volver al Inicio" }
                </button>
                <h1 class="text-xl font-display font-bold text-gray-800 tracking-tight">{ "Mi Panel de Colaborador" }</h1>
            </div>

            <div class="container mx-auto px-4 sm:px-6 lg:px-8 max-w-6xl">
                // Banner Section
                <div class="bg-gradient-to-r from-teal-500 to-cyan-600 rounded-3xl shadow-lg p-8 md:p-12 mb-10 text-white flex flex-col md:flex-row items-center justify-between gap-6 relative overflow-hidden">
                    <div class="absolute inset-0 z-0 opacity-10 bg-[url('/pattern.svg')]"></div>
                    <div class="relative z-10 text-center md:text-left">
                        <h2 class="text-3xl md:text-4xl font-display font-bold mb-2">{ "¡Gracias por tu apoyo!" }</h2>
                        <p class="text-lg opacity-90 max-w-xl">{ "Tu colaboración hace posible que estas causas alcancen sus metas. Gestiona tus actividades desde aquí." }</p>
                    </div>
                    <div class="relative z-10 bg-white/20 backdrop-blur-md px-6 py-4 rounded-2xl border border-white/20 text-center shrink-0">
                        <span class="block text-4xl mb-1">{"🤝"}</span>
                        <span class="text-sm font-bold uppercase tracking-wider">{ "Eventos Activos" }</span>
                    </div>
                </div>
                
                if *loading {
                     <div class="flex flex-col items-center justify-center py-20">
                         <div class="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-teal-500 mb-4"></div>
                         <p class="text-gray-500 font-medium">{ "Cargando tus eventos..." }</p>
                     </div>
                } else if kermesses.is_empty() {
                     <div class="bg-white rounded-3xl shadow-xl p-12 text-center max-w-2xl mx-auto border border-gray-100 flex flex-col items-center">
                        <div class="w-24 h-24 bg-gray-100 rounded-full flex items-center justify-center text-5xl mb-6">
                            {"😅"}
                        </div>
                        <h3 class="text-2xl font-bold text-gray-800 mb-3">{ "Aún no colaboras en eventos" }</h3>
                        <p class="text-gray-500 mb-8 max-w-md">{ "Puedes buscar eventos activos en tu comunidad y solicitar unirte como colaborador para ayudar a recaudar fondos." }</p>
                        <button onclick={Callback::from(move |_| navigator.push(&Route::Home))} class="bg-gradient-to-r from-teal-500 to-cyan-600 text-white font-bold px-8 py-4 rounded-xl shadow-lg hover:shadow-teal-500/30 hover:to-cyan-700 transition transform hover:-translate-y-1">
                            { "Explorar Eventos" }
                        </button>
                     </div>
                } else {
                    <div class="mb-6 flex justify-between items-end">
                        <h3 class="text-2xl font-bold text-gray-800">{ "Tus Kermesses" }</h3>
                        <span class="bg-teal-100 text-teal-800 font-bold px-3 py-1 rounded-full text-sm">{ format!("{} Eventos", kermesses.len()) }</span>
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        {
                            kermesses.iter().map(|k| {
                                let id = k.id;
                                let format_navigator = navigator.clone();
                                html! {
                                    <div class="bg-white rounded-2xl shadow-md hover:shadow-xl transition-all duration-300 transform hover:-translate-y-2 overflow-hidden border border-gray-100 flex flex-col group h-full">
                                        <div class="p-8 flex-grow flex flex-col">
                                            <div class="flex justify-between items-start mb-4">
                                                <div class="w-12 h-12 bg-teal-50 text-teal-600 rounded-xl flex items-center justify-center text-2xl">
                                                    {"🎪"}
                                                </div>
                                                <span class="inline-flex px-3 py-1 rounded-full bg-blue-50 text-blue-700 font-bold text-xs uppercase tracking-wide border border-blue-100">
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
                                            
                                            <h3 class="text-xl font-display font-bold text-gray-900 mb-3 line-clamp-2 leading-tight group-hover:text-teal-600 transition-colors">{ &k.name }</h3>
                                            
                                            <div class="flex items-center gap-2 mb-6 text-sm text-gray-500 font-medium bg-gray-50 px-3 py-2 rounded-lg w-fit">
                                                <span>{"📅"}</span>
                                                <span>{ &k.event_date }</span>
                                            </div>
                                            
                                            <div class="mt-auto pt-4 border-t border-gray-100">
                                                <button 
                                                    onclick={Callback::from(move |_| format_navigator.push(&Route::KermesseOrders { id }))}
                                                    class="w-full bg-gray-50 text-gray-700 font-bold py-3 rounded-xl hover:bg-teal-50 hover:text-teal-700 transition flex items-center justify-center gap-2 group-btn"
                                                >
                                                    <span class="group-btn-hover:-rotate-12 transition-transform">{"📋"}</span> 
                                                    { "Gestionar Pedidos" }
                                                </button>
                                            </div>
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
