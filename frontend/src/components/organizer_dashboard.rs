use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use crate::context::UserContext;
use crate::components::organizer_orders::OrganizerOrders;
use crate::components::organizer_collaborators::OrganizerCollaborators;

use serde::{de::Error, Deserializer, Deserialize};
use serde_json::Value;

// ... (keep deserialize helper functions if they are useful, or move to utils) ...
pub fn deserialize_price<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Number(n) => n.as_f64().ok_or_else(|| Error::custom("Invalid number")),
        Value::String(s) => s.parse::<f64>().map_err(|e| Error::custom(format!("Invalid string number: {}", e))),
        _ => Err(Error::custom("Price must be number or string")),
    }
}

pub fn deserialize_option_price<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<Value> = Option::deserialize(deserializer)?;
    match value {
        Some(Value::Number(n)) => Ok(Some(n.as_f64().ok_or_else(|| Error::custom("Invalid number"))?)),
        Some(Value::String(s)) => {
            if s.is_empty() {
                Ok(None)
            } else {
                Ok(Some(s.parse::<f64>().map_err(|e| Error::custom(format!("Invalid string number: {}", e)))?))
            }
        },
        Some(Value::Null) | None => Ok(None),
        _ => Err(Error::custom("Price must be number, string or null")),
    }
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct DashboardStats {
    #[serde(deserialize_with = "deserialize_option_price")]
    pub financial_goal: Option<f64>,
    #[serde(deserialize_with = "deserialize_price")]
    pub total_raised: f64,
    pub progress_percentage: f64,
    pub total_orders: i64,
    pub pending_orders: i64,
    pub paid_orders: i64,
    pub delivered_orders: i64,
    pub ingredient_coverage_percentage: f64,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub kermesse_id: i32,
}

#[derive(Clone, PartialEq)]
enum DashboardTab {
    Overview,
    Orders,
    Collaborators,
}

#[function_component(OrganizerDashboardV2)]
pub fn organizer_dashboard_v2(props: &Props) -> Html {
    let _navigator = use_navigator().unwrap();
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let stats = use_state(|| None::<DashboardStats>);
    let kermesse_id = props.kermesse_id;
    let active_tab = use_state(|| DashboardTab::Overview);

    // Fetch Stats Effect
    {
        let stats = stats.clone();
        let token = user_ctx.user.as_ref().map(|u| u.token.clone());
        use_effect_with((kermesse_id, *active_tab == DashboardTab::Overview), move |(id, is_overview)| {
            if *is_overview {
                let id = *id;
                let stats = stats.clone();
                if let Some(token) = token {
                    wasm_bindgen_futures::spawn_local(async move {
                        let url = format!("http://127.0.0.1:8080/kermesses/{}/dashboard/stats", id);
                        if let Ok(resp) = Request::get(&url)
                            .header("Authorization", &format!("Bearer {}", token))
                            .send()
                            .await
                        {
                            if let Ok(fetched) = resp.json().await {
                                stats.set(Some(fetched));
                            }
                        }
                    });
                }
            }
            || ()
        });
    }

    let render_overview = |stats_data: &DashboardStats| html! {
        <div class="animate-fade-in">
            <h2 class="text-xl font-bold mb-6 text-gray-800">{ "Resumen del Evento" }</h2>
             // Financial Progress
            <div class="mb-8 p-6 bg-gradient-to-br from-white to-green-50 rounded-xl shadow-sm border border-green-100">
                <h3 class="text-lg font-semibold mb-4 text-green-800">{ "💰 Progreso Financiero" }</h3>
                {
                    if let Some(goal) = stats_data.financial_goal {
                        html! {
                            <div>
                                <div class="flex justify-between mb-2">
                                    <span class="text-sm font-medium text-gray-600">{ format!("Recaudado: Bs. {:.2}", stats_data.total_raised) }</span>
                                    <span class="text-sm font-medium text-gray-600">{ format!("Meta: Bs. {:.2}", goal) }</span>
                                </div>
                                <div class="w-full bg-gray-200 rounded-full h-6 border border-gray-100 overflow-hidden">
                                     <div 
                                        class="bg-gradient-to-r from-green-400 to-green-600 h-6 rounded-full flex items-center justify-center text-white text-xs font-bold shadow-inner"
                                        style={format!("width: {}%", stats_data.progress_percentage)}
                                    >
                                        { format!("{:.1}%", stats_data.progress_percentage) }
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! { <p class="text-gray-500 italic">{ "No se ha definido una meta financiera." }</p> }
                    }
                }
            </div>

            // Order Statistics
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                <div class="bg-blue-50 p-4 rounded-xl border border-blue-100 text-center">
                    <p class="text-sm text-gray-500 uppercase tracking-wide">{ "Total Pedidos" }</p>
                    <p class="text-3xl font-extrabold text-blue-600">{ stats_data.total_orders }</p>
                </div>
                <div class="bg-yellow-50 p-4 rounded-xl border border-yellow-100 text-center">
                    <p class="text-sm text-gray-500 uppercase tracking-wide">{ "Pendientes" }</p>
                    <p class="text-3xl font-extrabold text-yellow-600">{ stats_data.pending_orders }</p>
                </div>
                <div class="bg-green-50 p-4 rounded-xl border border-green-100 text-center">
                    <p class="text-sm text-gray-500 uppercase tracking-wide">{ "Confirmados" }</p>
                    <p class="text-3xl font-extrabold text-green-600">{ stats_data.paid_orders }</p>
                </div>
                <div class="bg-purple-50 p-4 rounded-xl border border-purple-100 text-center">
                    <p class="text-sm text-gray-500 uppercase tracking-wide">{ "Entregados" }</p>
                    <p class="text-3xl font-extrabold text-purple-600">{ stats_data.delivered_orders }</p>
                </div>
            </div>

            // Ingredient Coverage
            <div class="p-6 bg-white rounded-xl border border-gray-100 shadow-sm">
                <h3 class="text-lg font-semibold mb-4 text-gray-700">{ "🥕 Cobertura de Insumos" }</h3>
                <div class="flex items-center gap-4">
                    <div class="flex-grow bg-gray-200 rounded-full h-4 overflow-hidden">
                        <div 
                            class="bg-gradient-to-r from-orange-400 to-orange-600 h-4 rounded-full"
                            style={format!("width: {}%", stats_data.ingredient_coverage_percentage)}
                        >
                        </div>
                    </div>
                    <span class="text-sm font-bold text-gray-700">{ format!("{:.0}%", stats_data.ingredient_coverage_percentage) }</span>
                </div>
            </div>
        </div>
    };

    html! {
        <div class="bg-white rounded-2xl shadow-xl border border-gray-100 overflow-hidden mb-8 transform transition-all">
            <div class="flex justify-between items-center bg-white rounded-t-2xl p-6 border-b border-gray-100">
                <h2 class="text-2xl font-bold text-gray-800">{ "Dashboard del Organizador" }</h2>
                <div>
                    <button 
                        onclick={Callback::from(move |_| {
                            let window = web_sys::window().unwrap();
                            let _ = window.location().set_href(&format!("/kermesses/{}/edit", kermesse_id)); 
                        })} 
                        class="bg-gray-100 text-gray-700 hover:bg-gray-200 px-4 py-2 rounded-lg font-medium transition flex items-center gap-2"
                    >
                        <span>{"✏️"}</span>
                        { "Editar Kermesse" }
                    </button>
                </div>
            </div>

            // Tabs Header
            <div class="flex border-b border-gray-100 bg-gray-50/50">
                <button 
                    onclick={let at = active_tab.clone(); Callback::from(move |_| at.set(DashboardTab::Overview))}
                    class={format!("flex-1 py-4 text-sm font-bold uppercase tracking-wide transition border-b-2 hover:bg-gray-50 {}", 
                        if *active_tab == DashboardTab::Overview { "text-primary border-primary bg-white" } else { "text-gray-500 border-transparent hover:text-gray-700" })}
                >
                    { "📊 Resumen" }
                </button>
                <button 
                    onclick={let at = active_tab.clone(); Callback::from(move |_| at.set(DashboardTab::Orders))}
                    class={format!("flex-1 py-4 text-sm font-bold uppercase tracking-wide transition border-b-2 hover:bg-gray-50 {}", 
                        if *active_tab == DashboardTab::Orders { "text-primary border-primary bg-white" } else { "text-gray-500 border-transparent hover:text-gray-700" })}
                >
                    { "📦 Gestión Pedidos" }
                </button>
                <button 
                    onclick={let at = active_tab.clone(); Callback::from(move |_| at.set(DashboardTab::Collaborators))}
                    class={format!("flex-1 py-4 text-sm font-bold uppercase tracking-wide transition border-b-2 hover:bg-gray-50 {}", 
                        if *active_tab == DashboardTab::Collaborators { "text-primary border-primary bg-white" } else { "text-gray-500 border-transparent hover:text-gray-700" })}
                >
                    { "👥 Equipo" }
                </button>
            </div>

            <div class="p-6">
                {
                    match *active_tab {
                        DashboardTab::Overview => {
                            if let Some(s) = &*stats {
                                render_overview(s)
                            } else {
                                html! { <div class="text-center py-10 text-gray-400 animate-pulse">{ "Cargando estadísticas..." }</div> }
                            }
                        },
                        DashboardTab::Orders => html! { <OrganizerOrders kermesse_id={kermesse_id} /> },
                        DashboardTab::Collaborators => html! { <OrganizerCollaborators kermesse_id={kermesse_id} /> },
                    }
                }
            </div>
        </div>
    }
}
