use yew::prelude::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use crate::context::UserContext;

#[derive(Clone, PartialEq, Deserialize)]
pub struct IngredientWithProgress {
    pub id: i32,
    pub name: String,
    pub quantity_needed: f64,
    pub unit: String,
    pub quantity_donated: f64,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub kermesse_id: i32,
}

#[derive(Serialize)]
struct DonateRequest {
    quantity: f64,
}

#[function_component(IngredientDonationsList)]
pub fn ingredient_donations_list(props: &Props) -> Html {
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let ingredients = use_state(|| vec![]);
    let kermesse_id = props.kermesse_id;

    {
        let ingredients = ingredients.clone();
        use_effect_with(kermesse_id, move |id| {
            let id = *id;
            let ingredients = ingredients.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://127.0.0.1:8080/kermesses/{}/ingredients/progress", id);
                if let Ok(resp) = Request::get(&url).send().await {
                    if let Ok(fetched) = resp.json::<Vec<IngredientWithProgress>>().await {
                        ingredients.set(fetched);
                    }
                }
            });
            || ()
        });
    }

    let on_donate = {
        let token = user_ctx.user.as_ref().map(|u| u.token.clone());
        let ingredients = ingredients.clone();
        
        Callback::from(move |ingredient_id: i32| {
            let token = token.clone();
            let ingredients = ingredients.clone();
            let kermesse_id = kermesse_id;
            
            if let Some(token) = token {
                let quantity_str = gloo_dialogs::prompt("¿Cuánto quieres donar? (número con decimales)", Some("1.0"));
                if let Some(qty_str) = quantity_str {
                    if let Ok(quantity) = qty_str.parse::<f64>() {
                        wasm_bindgen_futures::spawn_local(async move {
                            let body = serde_json::to_string(&DonateRequest { quantity }).unwrap();
                            let resp = Request::post(&format!("http://127.0.0.1:8080/ingredients/{}/donate", ingredient_id))
                                .header("Authorization", &format!("Bearer {}", token))
                                .header("Content-Type", "application/json")
                                .body(body)
                                .send()
                                .await;

                            if let Ok(resp) = resp {
                                if resp.ok() {
                                    gloo_dialogs::alert("¡Gracias por tu donación!");
                                    // Refresh list
                                    let url = format!("http://127.0.0.1:8080/kermesses/{}/ingredients/progress", kermesse_id);
                                    if let Ok(resp) = Request::get(&url).send().await {
                                        if let Ok(fetched) = resp.json::<Vec<IngredientWithProgress>>().await {
                                            ingredients.set(fetched);
                                        }
                                    }
                                } else {
                                    gloo_dialogs::alert("Error al registrar donación");
                                }
                            }
                        });
                    }
                }
            } else {
                gloo_dialogs::alert("Debes iniciar sesión para donar");
            }
        })
    };

    html! {
        <div class="bg-white rounded-xl shadow-lg p-6">
            <h3 class="text-2xl font-bold mb-6 text-gray-800 border-b pb-2">{ "Ingredientes Necesarios" }</h3>
            
            if ingredients.is_empty() {
                <p class="text-gray-500 italic text-center py-4">{ "No se necesitan ingredientes o no se han publicado aún." }</p>
            } else {
                <div class="space-y-4">
                    {
                        ingredients.iter().map(|ingredient| {
                            let percentage = if ingredient.quantity_needed > 0.0 {
                                (ingredient.quantity_donated / ingredient.quantity_needed * 100.0).min(100.0)
                            } else {
                                100.0
                            };
                            let is_complete = percentage >= 100.0;
                            let ingredient_id = ingredient.id;
                            let on_donate_click = on_donate.clone(); // Clone callback for use in closure
                            
                            html! {
                                <div class="border border-gray-200 rounded-lg p-4 hover:shadow-md transition">
                                    <div class="flex justify-between items-center mb-2">
                                        <h4 class="font-bold text-gray-900">{ &ingredient.name }</h4>
                                        <span class="text-sm text-gray-600">
                                            { format!("{:.1}/{:.1} {}", ingredient.quantity_donated, ingredient.quantity_needed, &ingredient.unit) }
                                        </span>
                                    </div>
                                    
                                    <div class="w-full bg-gray-200 rounded-full h-4 mb-3">
                                        <div 
                                            class={if is_complete { "bg-green-500" } else { "bg-blue-500" }}
                                            style={format!("width: {}%; height: 100%; border-radius: 9999px; transition: width 0.3s;", percentage)}
                                        >
                                        </div>
                                    </div>
                                    
                                    {
                                        if !is_complete {
                                            html! {
                                                <button 
                                                    onclick={move |_| on_donate_click.emit(ingredient_id)}
                                                    class="w-full bg-orange-500 text-white font-bold py-2 px-4 rounded-lg shadow hover:bg-orange-600 transition"
                                                >
                                                    { "💝 Donar" }
                                                </button>
                                            }
                                        } else {
                                            html! {
                                                <div class="w-full bg-green-100 text-green-700 font-bold py-2 px-4 rounded-lg text-center">
                                                    { "✓ ¡Ingrediente completo!" }
                                                </div>
                                            }
                                        }
                                    }
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        </div>
    }
}
