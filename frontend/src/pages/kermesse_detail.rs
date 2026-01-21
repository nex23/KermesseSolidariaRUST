use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;
use serde_json;
use crate::router::Route;
use crate::pages::home::Kermesse;
use crate::components::organizer_dashboard::OrganizerDashboardV2;
use crate::components::collaboration_form::CollaborationRequestForm;
use crate::components::ingredient_donations::IngredientDonationsList;
use crate::context::{CartContext, CartAction, CartItem};
use crate::components::cart_drawer::CartDrawer;
// use gloo_console;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Dish {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[serde(deserialize_with = "deserialize_price")]
    pub price: f64,
    pub quantity_available: i32,
    pub image_url: Option<String>,
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

#[derive(Clone, PartialEq, Deserialize)]
pub struct Collaborator {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub role: String,
    pub phone: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    #[serde(deserialize_with = "deserialize_price")]
    pub quantity_needed: f64,
    pub unit: String,
    #[serde(deserialize_with = "deserialize_price")]
    pub quantity_donated: f64,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct KermesseDetailData {
    #[serde(flatten)]
    pub kermesse: Kermesse,
    pub dishes: Vec<Dish>,
    pub collaborators: Vec<Collaborator>,
    pub ingredients: Vec<Ingredient>,
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
    let cart_ctx = use_context::<CartContext>().expect("No CartContext found");

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

    // State now stores: (id, name, price, quantity_available)
    let selected_dish = use_state(|| None::<(i32, String, f64, i32)>);
    let modal_quantity = use_state(|| 1);

    if let Some(detail_data) = &*detail {
        let kermesse = &detail_data.kermesse;
        let is_organizer = user_ctx.user.as_ref().map(|u| {
             // ... organizer check ...
             let uid = u.id;
             uid == kermesse.organizer_id && uid != 0
        }).unwrap_or(false); 
        // Assume id 0 is not valid or at least checking exists
        let user_token = user_ctx.user.as_ref().map(|u| u.token.clone());

        // on_add_dish callback
        let on_add_dish = {
            let navigator = navigator.clone();
            let id = id;
            Callback::from(move |_| navigator.push(&Route::AddDish { id }))
        };

        // Open Modal Logic
        let open_quantity_modal = {
             let selected_dish = selected_dish.clone();
             let modal_quantity = modal_quantity.clone();
             Callback::from(move |(id, name, price, available): (i32, String, f64, i32)| {
                 selected_dish.set(Some((id, name, price, available)));
                 modal_quantity.set(1);
             })
        };

        // Add to Cart Logic (Final Action from Modal)
        let add_to_cart_action = {
            let cart_ctx = cart_ctx.clone();
            let kermesse_id = id;
            let selected_dish = selected_dish.clone();
            let modal_quantity = modal_quantity.clone();
            Callback::from(move |_| {
                 if let Some((dish_id, name, price, _)) = &*selected_dish {
                     cart_ctx.dispatch.emit(CartAction::AddItem(CartItem {
                         dish_id: *dish_id,
                         dish_name: name.clone(),
                         price: *price,
                         quantity: *modal_quantity,
                         kermesse_id,
                     }));
                     gloo_dialogs::alert("¡Añadido al carrito!");
                     selected_dish.set(None); // Close modal
                 }
            })
        };

        html! {
            <div class="min-h-screen bg-gray-50 text-gray-800 font-sans p-6">
                <CartDrawer />
                
                // --- QUANTITY MODAL ---
                if let Some((_, name, price, available)) = &*selected_dish {
                    <div class="fixed inset-0 z-[60] flex items-center justify-center bg-black bg-opacity-50 animate-fade-in">
                        <div class="bg-white rounded-2xl shadow-2xl p-6 w-full max-w-sm m-4 transform transition-all scale-100">
                            <h3 class="text-2xl font-bold text-gray-800 mb-2">{ name }</h3>
                            <p class="text-gray-500 mb-2">{ format!("Bs. {:.2} / unidad", price) } </p>
                            <p class="text-sm font-bold text-teal-600 mb-6">{ format!("Disponibles: {}", available) }</p>
                            
                            <div class="flex items-center justify-center gap-6 mb-8">
                                <button 
                                    onclick={let mq = modal_quantity.clone(); Callback::from(move |_| if *mq > 1 { mq.set(*mq - 1) })}
                                    class="w-12 h-12 rounded-full bg-gray-100 text-gray-600 font-bold text-xl hover:bg-gray-200 flex items-center justify-center transition disabled:opacity-50"
                                    disabled={*modal_quantity <= 1}
                                >
                                    {"-"}
                                </button>
                                <span class="text-4xl font-bold text-primary w-12 text-center">{ *modal_quantity }</span>
                                <button 
                                    onclick={let mq = modal_quantity.clone(); let max = *available; Callback::from(move |_| if *mq < max { mq.set(*mq + 1) })}
                                    class="w-12 h-12 rounded-full bg-gray-100 text-gray-600 font-bold text-xl hover:bg-gray-200 flex items-center justify-center transition disabled:opacity-50"
                                    disabled={*modal_quantity >= *available}
                                >
                                    {"+"}
                                </button>
                            </div>
                            
                            <div class="flex gap-4">
                                <button 
                                    onclick={let sd = selected_dish.clone(); Callback::from(move |_| sd.set(None))}
                                    class="flex-1 bg-gray-100 text-gray-700 font-bold py-3 rounded-xl hover:bg-gray-200 transition"
                                >
                                    { "Cancelar" }
                                </button>
                                <button 
                                    onclick={add_to_cart_action}
                                    class="flex-1 bg-primary text-white font-bold py-3 rounded-xl hover:bg-red-600 transition shadow-lg"
                                >
                                    { format!("Agregar Bs. {:.2}", *price * (*modal_quantity as f64)) }
                                </button>
                            </div>
                        </div>
                    </div>
                }

                <button onclick={Callback::from(move |_| navigator.push(&Route::Home))} class="mb-4 flex items-center text-primary hover:text-red-700 font-medium">
                     { "← Volver a Eventos" }
                </button>
                // ... header ...
                <div class="bg-white rounded-3xl shadow-xl overflow-hidden mb-8">
                     <div class="bg-gradient-to-r from-primary to-secondary p-8 text-white relative">
                        <div class="flex flex-col md:flex-row items-center">
                            if let Some(img_url) = &kermesse.beneficiary_image_url {
                                <img src={img_url.clone()} alt={kermesse.beneficiary_name.clone()} class="w-32 h-32 rounded-full border-4 border-white shadow-lg mb-4 md:mb-0 md:mr-8 object-cover" />
                            }
                            <div>
                                <h1 class="text-4xl font-bold mb-2">{ &kermesse.name }</h1>
                                <p class="text-lg opacity-90 mb-4">{ &kermesse.description }</p>
                                // ... metadata ...
                                <div class="flex flex-wrap items-center gap-4">
                                     <span class="bg-white bg-opacity-20 px-3 py-1 rounded-full text-sm font-semibold flex items-center">
                                        <span class="mr-2">{"📅"}</span> { &kermesse.event_date }
                                     </span>
                                     // ...
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
                                    let name = dish.name.clone();
                                    let price = dish.price;
                                    let available = dish.quantity_available;
                                    let on_open_modal = open_quantity_modal.clone();
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
                                                 <button 
                                                    onclick={Callback::from(move |e: MouseEvent| {
                                                        e.stop_propagation();
                                                        if available > 0 {
                                                            on_open_modal.emit((dish_id, name.clone(), price, available));
                                                        }
                                                    })}
                                                    disabled={available == 0}
                                                    class={format!("w-full text-white py-2 rounded-lg font-bold shadow-md transition {}", if available > 0 { "bg-secondary hover:bg-teal-500" } else { "bg-gray-400 cursor-not-allowed" })}
                                                 >
                                                    { if available > 0 { "Pedir Ahora" } else { "Agotado" } }
                                                 </button>
                                             </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>

                    <div>
                        // Social Sharing Buttons
                        <div class="bg-gradient-to-r from-purple-50 to-pink-50 rounded-xl shadow-md p-6 mb-6">
                            <h3 class="text-xl font-bold mb-4 text-gray-800">{ "Compartir en Redes" }</h3>
                            <div class="flex gap-3">
                                <a 
                                    href={format!("https://wa.me/?text=¡Ayuda a {}! {}", &kermesse.beneficiary_name, format!("http://127.0.0.1:8000/kermesses/{}", id))}
                                    target="_blank"
                                    class="flex-1 bg-green-500 text-white font-bold py-3 px-4 rounded-lg text-center hover:bg-green-600 transition shadow"
                                >
                                    { "📱 WhatsApp" }
                                </a>
                                <a 
                                    href={format!("https://www.facebook.com/sharer/sharer.php?u={}", format!("http://127.0.0.1:8000/kermesses/{}", id))}
                                    target="_blank"
                                    class="flex-1 bg-blue-600 text-white font-bold py-3 px-4 rounded-lg text-center hover:bg-blue-700 transition shadow"
                                >
                                    { "👍 Facebook" }
                                </a>
                            </div>
                        </div>

                        // Organizer Dashboard (only for organizer)
                        if is_organizer {
                            <OrganizerDashboardV2 kermesse_id={id} />
                        }

                        // Collaboration Request Form
                        if user_ctx.user.is_some() {
                            <div class="mb-6">
                                <CollaborationRequestForm kermesse_id={id} />
                            </div>
                        }

                        <h2 class="text-2xl font-bold mb-6 text-gray-800 border-b pb-2">{ "Colaboradores (Vendedores)" }</h2>
                        <div class="bg-white rounded-xl shadow-lg p-6 mb-6">
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
