use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use crate::router::Route;
use crate::context::UserContext;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize)]
pub struct KermesseBasic {
    pub id: i32,
    pub name: String,
    pub event_date: String,
    pub status: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct MyKermesseResponse {
    #[serde(flatten)]
    pub kermesse: KermesseBasic,
    #[serde(deserialize_with = "deserialize_price")]
    pub total_raised: f64,
    pub total_orders: i64,
}

// Custom deserializer to handle both string and number formats
fn deserialize_price<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;
    
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Number(n) => n.as_f64().ok_or_else(|| Error::custom("Invalid number")),
        Value::String(s) => s.parse::<f64>().map_err(|e| Error::custom(format!("Invalid string number: {}", e))),
        _ => Err(Error::custom("Price must be number or string")),
    }
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let navigator = use_navigator().unwrap();
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let kermesses = use_state(|| Vec::<MyKermesseResponse>::new());
    let loading = use_state(|| true);

    if user_ctx.user.is_none() {
        navigator.push(&Route::Login);
        return html! {};
    }

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
                    let url = "http://127.0.0.1:8080/my-kermesses"; 
                    if let Ok(resp) = Request::get(url)
                        .header("Authorization", &format!("Bearer {}", token))
                        .send()
                        .await 
                    {
                        if resp.ok() {
                            if let Ok(data) = resp.json::<Vec<MyKermesseResponse>>().await {
                                kermesses.set(data);
                            }
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

    html! {
        <div class="min-h-screen bg-gray-50 font-sans pb-12">
            // Header
            <div class="bg-white shadow-sm border-b border-gray-100 py-4 px-6 mb-8 flex items-center justify-between">
                <button onclick={Callback::from(move |_| navigator.push(&Route::Home))} class="flex items-center text-gray-500 hover:text-orange-600 font-bold transition">
                    <span class="mr-2">{"←"}</span> { "Volver al Inicio" }
                </button>
                <div class="flex items-center gap-4">
                    <h1 class="text-xl font-display font-bold text-gray-800 tracking-tight hidden sm:block">{ "Panel de Organizador" }</h1>
                    <button 
                        onclick={Callback::from(move |_| navigator.push(&Route::CreateKermesse))} 
                        class="bg-gradient-to-r from-orange-500 to-red-600 text-white font-bold py-2 px-4 rounded-xl shadow hover:shadow-lg transition transform hover:-translate-y-0.5 flex items-center gap-2"
                    >
                        <span>{"+"}</span> { "Crear Kermesse" }
                    </button>
                </div>
            </div>

            <div class="container mx-auto px-4 sm:px-6 lg:px-8 max-w-6xl">
                // Banner Section
                <div class="bg-gradient-to-br from-gray-900 to-gray-800 rounded-3xl shadow-xl p-8 md:p-12 mb-10 text-white flex flex-col md:flex-row items-center justify-between gap-6 relative overflow-hidden">
                    <div class="absolute inset-0 z-0 opacity-20 bg-[url('/pattern.svg')]"></div>
                    <div class="relative z-10 text-center md:text-left">
                        <h2 class="text-3xl md:text-4xl font-display font-bold mb-2">{ "Tus Eventos Solidarios" }</h2>
                        <p class="text-lg opacity-90 max-w-xl">{ "Organiza, gestiona y haz seguimiento del impacto de tus kermesses desde un solo lugar." }</p>
                    </div>
                    <div class="relative z-10 bg-white/10 backdrop-blur-md px-6 py-4 rounded-2xl border border-white/20 text-center shrink-0">
                        <span class="block text-4xl mb-1">{"📊"}</span>
                        <span class="text-sm font-bold uppercase tracking-wider">{ "Resumen Global" }</span>
                    </div>
                </div>

                if *loading {
                     <div class="flex flex-col items-center justify-center py-20">
                         <div class="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-orange-500 mb-4"></div>
                         <p class="text-gray-500 font-medium">{ "Cargando tus kermesses..." }</p>
                     </div>
                } else if kermesses.is_empty() {
                     <div class="bg-white rounded-3xl shadow-xl p-12 text-center max-w-2xl mx-auto border border-gray-100 flex flex-col items-center">
                        <div class="w-24 h-24 bg-orange-50 text-orange-500 rounded-full flex items-center justify-center text-5xl mb-6">
                            {"📝"}
                        </div>
                        <h3 class="text-2xl font-bold text-gray-800 mb-3">{ "Aún no has organizado eventos" }</h3>
                        <p class="text-gray-500 mb-8 max-w-md">{ "Comienza a ayudar a los demás organizando tu primera kermesse solidaria. Es rápido, fácil y de gran impacto." }</p>
                        <button onclick={Callback::from(move |_| navigator.push(&Route::CreateKermesse))} class="bg-gradient-to-r from-orange-500 to-red-600 text-white font-bold px-8 py-4 rounded-xl shadow-lg hover:shadow-orange-500/30 hover:to-red-700 transition transform hover:-translate-y-1">
                            { "¡Crear mi primera Kermesse!" }
                        </button>
                     </div>
                } else {
                    <div class="mb-6 flex justify-between items-end">
                        <h3 class="text-2xl font-bold text-gray-800">{ "Historial de Eventos" }</h3>
                        <span class="bg-orange-100 text-orange-800 font-bold px-3 py-1 rounded-full text-sm">{ format!("{} Eventos", kermesses.len()) }</span>
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        {
                            kermesses.iter().map(|k| {
                                let id = k.kermesse.id;
                                let format_navigator = navigator.clone();
                                html! {
                                    <div class="bg-white rounded-2xl shadow-md hover:shadow-xl transition-all duration-300 transform hover:-translate-y-2 overflow-hidden border border-gray-100 flex flex-col group h-full">
                                        <div class="p-8 flex-grow flex flex-col">
                                            <div class="flex justify-between items-start mb-4">
                                                <div class={format!("w-12 h-12 rounded-xl flex items-center justify-center text-2xl {}", if k.kermesse.status == "ACTIVE" { "bg-green-50 text-green-600" } else { "bg-gray-100 text-gray-500" })}>
                                                    { if k.kermesse.status == "ACTIVE" { "🟢" } else { "🏁" } }
                                                </div>
                                                <span class={format!("inline-flex px-3 py-1 rounded-full font-bold text-xs uppercase tracking-wide border {}", 
                                                    if k.kermesse.status == "ACTIVE" { "bg-green-50 text-green-700 border-green-100" } else { "bg-gray-50 text-gray-600 border-gray-200" }
                                                )}>
                                                    { if k.kermesse.status == "ACTIVE" { "Activo" } else { "Finalizado" } }
                                                </span>
                                            </div>
                                            
                                            <h3 class="text-xl font-display font-bold text-gray-900 mb-3 line-clamp-2 leading-tight group-hover:text-orange-600 transition-colors">{ &k.kermesse.name }</h3>
                                            
                                            <div class="flex items-center gap-2 mb-6 text-sm text-gray-500 font-medium">
                                                <span>{"📅"}</span>
                                                <span>{ &k.kermesse.event_date }</span>
                                            </div>

                                            <div class="grid grid-cols-2 gap-4 mb-6 pt-4 border-t border-gray-50">
                                                <div>
                                                    <p class="text-xs text-gray-400 font-bold uppercase tracking-wider mb-1">{"Platos"}</p>
                                                    <p class="text-xl font-display font-bold text-gray-800">{ k.total_orders }</p>
                                                </div>
                                                <div>
                                                    <p class="text-xs text-gray-400 font-bold uppercase tracking-wider mb-1">{"Recaudado"}</p>
                                                    <p class="text-xl font-display font-bold text-green-600">{ format!("Bs. {:.0}", k.total_raised) }</p>
                                                </div>
                                            </div>
                                            
                                            <div class="mt-auto grid grid-cols-2 gap-3">
                                                <button 
                                                    onclick={Callback::from(move |_| format_navigator.push(&Route::KermesseDetail { id }))}
                                                    class="w-full bg-gray-50 text-gray-700 font-bold py-3 rounded-xl hover:bg-gray-200 hover:text-gray-900 transition flex items-center justify-center gap-2"
                                                >
                                                    { "Ver" }
                                                </button>
                                                <button 
                                                    onclick={Callback::from(move |_| format_navigator.push(&Route::EditKermesse { id }))}
                                                    class="w-full bg-orange-50 text-orange-700 font-bold py-3 rounded-xl hover:bg-orange-600 hover:text-white transition flex items-center justify-center gap-2"
                                                >
                                                    { "Editar" }
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
