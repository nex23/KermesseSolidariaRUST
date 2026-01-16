use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use crate::context::UserContext;

use serde::{de::Error, Deserializer, Deserialize};
use serde_json::Value;

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

#[function_component(OrganizerDashboardV2)]
pub fn organizer_dashboard_v2(props: &Props) -> Html {
    let _navigator = use_navigator().unwrap();
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let stats = use_state(|| None::<DashboardStats>);
    let kermesse_id = props.kermesse_id;

    {
        let stats = stats.clone();
        let token = user_ctx.user.as_ref().map(|u| u.token.clone());
        use_effect_with(kermesse_id, move |id| {
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
            || ()
        });
    }

    if let Some(stats_data) = &*stats {
        html! {
            <div class="bg-white rounded-xl shadow-lg p-6 mb-8">
                <h2 class="text-2xl font-bold mb-6 text-gray-800 border-b pb-2">{ "Dashboard del Organizador" }</h2>
                
                // Financial Progress
                <div class="mb-8">
                    <h3 class="text-lg font-semibold mb-4 text-gray-700">{ "Progreso Financiero" }</h3>
                    {
                        if let Some(goal) = stats_data.financial_goal {
                            html! {
                                <div>
                                    <div class="flex justify-between mb-2">
                                        <span class="text-sm font-medium text-gray-600">{ format!("Recaudado: Bs. {:.2}", stats_data.total_raised) }</span>
                                        <span class="text-sm font-medium text-gray-600">{ format!("Meta: Bs. {:.2}", goal) }</span>
                                    </div>
                                    <div class="w-full bg-gray-200 rounded-full h-6">
                                        <div 
                                            class="bg-gradient-to-r from-green-400 to-green-600 h-6 rounded-full flex items-center justify-center text-white text-xs font-bold"
                                            style={format!("width: {}%", stats_data.progress_percentage)}
                                        >
                                            { format!("{:.0}%", stats_data.progress_percentage) }
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {
                                <p class="text-gray-500 italic">{ "No se ha definido una meta financiera." }</p>
                            }
                        }
                    }
                </div>

                // Order Statistics
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                    <div class="bg-blue-50 p-4 rounded-lg">
                        <p class="text-sm text-gray-600">{ "Total Pedidos" }</p>
                        <p class="text-2xl font-bold text-blue-600">{ stats_data.total_orders }</p>
                    </div>
                    <div class="bg-yellow-50 p-4 rounded-lg">
                        <p class="text-sm text-gray-600">{ "Pendientes" }</p>
                        <p class="text-2xl font-bold text-yellow-600">{ stats_data.pending_orders }</p>
                    </div>
                    <div class="bg-green-50 p-4 rounded-lg">
                        <p class="text-sm text-gray-600">{ "Pagados" }</p>
                        <p class="text-2xl font-bold text-green-600">{ stats_data.paid_orders }</p>
                    </div>
                    <div class="bg-purple-50 p-4 rounded-lg">
                        <p class="text-sm text-gray-600">{ "Entregados" }</p>
                        <p class="text-2xl font-bold text-purple-600">{ stats_data.delivered_orders }</p>
                    </div>
                </div>

                // Ingredient Coverage
                <div>
                    <h3 class="text-lg font-semibold mb-4 text-gray-700">{ "Cobertura de Ingredientes" }</h3>
                    <div class="flex items-center gap-4">
                        <div class="flex-grow bg-gray-200 rounded-full h-4">
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
        }
    } else {
        html! {
            <div class="bg-white rounded-xl shadow-lg p-6 mb-8 text-center text-gray-500">
                { "Cargando estadísticas..." }
            </div>
        }
    }
}
