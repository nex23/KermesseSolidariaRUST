use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;
use crate::router::Route;
use crate::pages::home::Kermesse;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Dish {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64, // Using f64 for simplicity in frontend display
    pub quantity_available: i32,
    pub image_url: Option<String>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Collaborator {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub role: String,
    pub phone: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct KermesseDetailData {
    #[serde(flatten)]
    pub kermesse: Kermesse,
    pub dishes: Vec<Dish>,
    pub collaborators: Vec<Collaborator>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

use crate::context::UserContext;

#[function_component(KermesseDetail)]
pub fn kermesse_detail(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let id = props.id;
    let detail = use_state(|| None::<KermesseDetailData>);
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");

    {
        let detail = detail.clone();
        use_effect_with(id, move |id| {
            let id = *id;
            let detail = detail.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://127.0.0.1:8080/kermesses/{}", id);
                if let Ok(resp) = Request::get(&url).send().await {
                    if let Ok(fetched) = resp.json().await {
                        detail.set(Some(fetched));
                    }
                }
            });
            || ()
        });
    }

    if let Some(detail_data) = &*detail {
        let kermesse = &detail_data.kermesse;
        let is_organizer = user_ctx.user.as_ref().map(|u| u.id == kermesse.organizer_id && u.id != 0).unwrap_or(false); // Assume id 0 is not valid or at least checking exists
        let user_token = user_ctx.user.as_ref().map(|u| u.token.clone());

        let on_add_dish = {
            let navigator = navigator.clone();
            let id = id;
            Callback::from(move |_| navigator.push(&Route::AddDish { id }))
        };

        // Simple sale simulation for now (expandable)
        let perform_sale = {
             let token = user_token.clone();
             let kermesse_id = id;
             Callback::from(move |dish_id: i32| {
                 if let Some(t) = &token {
                     let t = t.clone();
                     wasm_bindgen_futures::spawn_local(async move {
                        // Dummy Sale Item for simplicity: buying 1 unit
                        // In real app, we need a cart or modal.
                        // We will just hit the sales endpoint with hardcoded JSON to prove connectivity
                        let body = serde_json::json!({
                            "kermesse_id": kermesse_id,
                            "client_name": "Cliente Web",
                            "items": [
                                { "dish_id": dish_id, "quantity": 1 }
                            ]
                        });
                        
                        let resp = Request::post("http://127.0.0.1:8080/sales")
                            .header("Authorization", &format!("Bearer {}", t))
                            .header("Content-Type", "application/json")
                            .body(body.to_string())
                            .send()
                            .await;

                        if let Ok(resp) = resp {
                             if resp.ok() {
                                 gloo_dialogs::alert("Pedido Realizado! (Venta registrada)");
                             } else {
                                  gloo_dialogs::alert("Error al realizar pedido");
                             }
                        } else {
                             gloo_dialogs::alert("Error de conexión");
                        }
                     });
                 } else {
                     gloo_dialogs::alert("Debes iniciar sesión para pedir.");
                 }
             })
        };

        html! {
            <div class="min-h-screen bg-gray-50 text-gray-800 font-sans p-6">
                <button onclick={Callback::from(move |_| navigator.push(&Route::Home))} class="mb-4 flex items-center text-primary hover:text-red-700 font-medium">
                     { "← Volver a Eventos" }
                </button>
                
                <div class="bg-white rounded-3xl shadow-xl overflow-hidden mb-8">
                     <div class="bg-gradient-to-r from-primary to-secondary p-8 text-white relative">
                        <div class="flex flex-col md:flex-row items-center">
                            if let Some(img_url) = &kermesse.beneficiary_image_url {
                                <img src={img_url.clone()} alt={kermesse.beneficiary_name.clone()} class="w-32 h-32 rounded-full border-4 border-white shadow-lg mb-4 md:mb-0 md:mr-8 object-cover" />
                            }
                            <div>
                                <h1 class="text-4xl font-bold mb-2">{ &kermesse.name }</h1>
                                <p class="text-lg opacity-90 mb-4">{ &kermesse.description }</p>
                                <div class="flex flex-wrap items-center gap-4">
                                     <span class="bg-white bg-opacity-20 px-3 py-1 rounded-full text-sm font-semibold flex items-center">
                                        <span class="mr-2">{"📅"}</span> { &kermesse.event_date }
                                     </span>
                                     if let (Some(start), Some(end)) = (&kermesse.start_time, &kermesse.end_time) {
                                         <span class="bg-white bg-opacity-20 px-3 py-1 rounded-full text-sm font-semibold flex items-center">
                                            <span class="mr-2">{"⏰"}</span> { format!("{} - {}", start, end) }
                                         </span>
                                     }
                                     <span class="bg-white bg-opacity-20 px-3 py-1 rounded-full text-sm font-semibold">{ format!("Beneficiario: {}", &kermesse.beneficiary_name) }</span>
                                </div>
                            </div>
                        </div>
                        if is_organizer {
                            <button onclick={on_add_dish} class="absolute top-8 right-8 bg-white text-secondary font-bold py-2 px-4 rounded-xl hover:bg-gray-100 transition shadow-lg hidden md:block">
                                { "+ Agregar Plato" }
                            </button>
                        }
                     </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    <div class="lg:col-span-2">
                        <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">{ "Menú del Día" }</h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                            {
                                detail_data.dishes.iter().map(|dish| {
                                    let dish_id = dish.id;
                                    let on_buy = perform_sale.clone();
                                    html! {
                                        <div class="bg-white rounded-xl shadow-md p-6 flex flex-col items-center text-center hover:shadow-lg transition cursor-pointer group">
                                             <div class="w-full h-40 bg-gray-200 rounded-lg mb-4 flex items-center justify-center text-6xl group-hover:scale-105 transition transform duration-300">
                                                { if let Some(img) = &dish.image_url { html!{ <img src={img.clone()} class="w-full h-full object-cover rounded-lg"/> } } else { html!{"🍛"} } }
                                             </div>
                                             
                                             <h3 class="text-xl font-bold text-gray-900 mb-1">{ &dish.name }</h3>
                                             <p class="text-gray-500 text-sm mb-4 line-clamp-2">{ &dish.description }</p>
                                             
                                             <div class="mt-auto w-full">
                                                 <div class="flex justify-between items-center mb-4 px-2">
                                                     <span class="text-2xl font-bold text-primary">{ format!("Bs. {:.2}", dish.price) }</span>
                                                     <span class="text-xs text-gray-400 border border-gray-200 px-2 py-1 rounded">{ format!("Disp: {}", dish.quantity_available) }</span>
                                                 </div>
                                                 <button onclick={move |_| on_buy.emit(dish_id)} class="w-full bg-secondary text-white py-2 rounded-lg font-bold shadow-md hover:bg-teal-500 transition">
                                                    { "Pedir Ahora" }
                                                 </button>
                                             </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>

                    <div>
                        <h2 class="text-2xl font-bold mb-6 text-gray-800 border-b pb-2">{ "Colaboradores (Vendedores)" }</h2>
                        <div class="bg-white rounded-xl shadow-lg p-6">
                            if detail_data.collaborators.is_empty() {
                                <p class="text-gray-500 italic text-center py-4">{ "Aún no hay colaboradores registrados." }</p>
                            } else {
                                <ul class="space-y-4">
                                    {
                                        detail_data.collaborators.iter().filter(|c| c.role == "SELLER").map(|collaborator| {
                                            html! {
                                                <li class="flex items-center space-x-4 p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition">
                                                    <div class="w-10 h-10 bg-primary text-white rounded-full flex items-center justify-center font-bold">
                                                        { collaborator.full_name.chars().next().unwrap_or('?') }
                                                    </div>
                                                    <div>
                                                        <p class="font-bold text-gray-900">{ &collaborator.full_name }</p>
                                                        <p class="text-xs text-gray-500">{ format!("Tel: {}", &collaborator.phone) }</p>
                                                    </div>
                                                    <a href={format!("https://wa.me/{}", collaborator.phone.replace(" ", ""))} target="_blank" class="ml-auto text-green-500 hover:text-green-600">
                                                        <span class="text-xl">{"💬"}</span>
                                                    </a>
                                                </li>
                                            }
                                        }).collect::<Html>()
                                    }
                                </ul>
                            }
                            <div class="mt-6 p-4 bg-yellow-50 rounded-lg text-sm text-yellow-800">
                                <p class="font-bold mb-1">{ "ℹ️ ¿Cómo comprar?" }</p>
                                { "Contacta a uno de los colaboradores listados arriba para realizar tu pedido o reserva." }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="min-h-screen flex items-center justify-center">
                <div class="animate-spin rounded-full h-16 w-16 border-t-4 border-primary"></div>
            </div>
        }
    }
}
