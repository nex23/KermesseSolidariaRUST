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
            <div class="min-h-screen bg-gray-50 text-gray-800 font-sans pb-12">
                <CartDrawer />
                
                // --- QUANTITY MODAL ---
                if let Some((_, name, price, available)) = &*selected_dish {
                    <div class="fixed inset-0 z-[60] flex items-center justify-center bg-black/60 backdrop-blur-sm animate-fade-in p-4">
                        <div class="bg-white rounded-3xl shadow-2xl p-8 w-full max-w-sm transform transition-all scale-100 border border-gray-100">
                            <div class="text-center mb-6">
                                <h3 class="text-3xl font-display font-bold text-gray-900 mb-2">{ name }</h3>
                                <p class="text-gray-500 font-medium">{ format!("Bs. {:.2} / unidad", price) } </p>
                            </div>
                            
                            <div class="bg-teal-50 rounded-xl p-4 mb-8 flex flex-col items-center">
                                <span class="text-sm font-bold text-teal-800 uppercase tracking-wider mb-1">{"Disponibles"}</span>
                                <span class="text-2xl font-bold text-teal-600">{ available }</span>
                            </div>
                            
                            <div class="flex items-center justify-center gap-6 mb-8">
                                <button 
                                    onclick={let mq = modal_quantity.clone(); Callback::from(move |_| if *mq > 1 { mq.set(*mq - 1) })}
                                    class="w-14 h-14 rounded-full bg-gray-100 text-gray-600 font-bold text-2xl hover:bg-orange-100 hover:text-orange-600 flex items-center justify-center transition disabled:opacity-50 disabled:cursor-not-allowed"
                                    disabled={*modal_quantity <= 1}
                                >
                                    {"-"}
                                </button>
                                <span class="text-5xl font-display font-bold text-gray-800 w-16 text-center">{ *modal_quantity }</span>
                                <button 
                                    onclick={let mq = modal_quantity.clone(); let max = *available; Callback::from(move |_| if *mq < max { mq.set(*mq + 1) })}
                                    class="w-14 h-14 rounded-full bg-gray-100 text-gray-600 font-bold text-2xl hover:bg-orange-100 hover:text-orange-600 flex items-center justify-center transition disabled:opacity-50 disabled:cursor-not-allowed"
                                    disabled={*modal_quantity >= *available}
                                >
                                    {"+"}
                                </button>
                            </div>
                            
                            <div class="flex gap-4">
                                <button 
                                    onclick={let sd = selected_dish.clone(); Callback::from(move |_| sd.set(None))}
                                    class="flex-1 bg-white border-2 border-gray-200 text-gray-600 font-bold py-3 rounded-xl hover:bg-gray-50 transition"
                                >
                                    { "Cancelar" }
                                </button>
                                <button 
                                    onclick={add_to_cart_action}
                                    class="flex-1 bg-gradient-to-r from-orange-500 to-red-600 text-white font-bold py-3 rounded-xl hover:shadow-lg hover:to-red-700 transition transform active:scale-95"
                                >
                                    { format!("Agregar Bs. {:.2}", *price * (*modal_quantity as f64)) }
                                </button>
                            </div>
                        </div>
                    </div>
                }

                // --- HEADER / BANNER ---
                <div class="relative bg-gradient-to-br from-gray-900 to-gray-800 text-white pb-32 pt-24 overflow-hidden">
                    <div class="absolute inset-0 opacity-20 bg-[url('/pattern.svg')]"></div>
                    <div class="container mx-auto px-6 relative z-10">
                        <button onclick={Callback::from(move |_| navigator.push(&Route::Home))} class="absolute top-8 left-6 md:left-0 flex items-center text-white/70 hover:text-white font-medium transition">
                             <span class="mr-2">{"←"}</span> { "Volver a Eventos" }
                        </button>

                        <div class="flex flex-col md:flex-row items-center md:items-start gap-8">
                            if let Some(img_url) = &kermesse.beneficiary_image_url {
                                <img src={img_url.clone()} alt={kermesse.beneficiary_name.clone()} class="w-40 h-40 md:w-48 md:h-48 rounded-full border-4 border-white/20 shadow-2xl object-cover shrink-0" />
                            }
                            <div class="text-center md:text-left flex-grow">
                                <div class="flex flex-col md:flex-row items-center md:items-start justify-between gap-4">
                                    <h1 class="text-4xl md:text-6xl font-display font-extrabold mb-4 leading-tight">{ &kermesse.name }</h1>
                                    if is_organizer {
                                        <button onclick={on_add_dish} class="bg-white/10 backdrop-blur border border-white/30 text-white font-bold py-2 px-6 rounded-full hover:bg-white hover:text-gray-900 transition flex items-center gap-2">
                                            <span>{"+"}</span> { "Agregar Plato" }
                                        </button>
                                    }
                                </div>
                                <p class="text-xl md:text-2xl text-gray-300 font-light max-w-2xl mb-8 leading-relaxed">{ &kermesse.description }</p>
                                
                                <div class="flex flex-wrap justify-center md:justify-start gap-3">
                                     <span class="bg-white/10 backdrop-blur px-4 py-2 rounded-full text-sm font-semibold flex items-center gap-2 border border-white/10">
                                        <span>{"📅"}</span> { &kermesse.event_date }
                                     </span>
                                     <span class="bg-white/10 backdrop-blur px-4 py-2 rounded-full text-sm font-semibold flex items-center gap-2 border border-white/10">
                                        <span>{"👤"}</span> { &kermesse.beneficiary_name }
                                     </span>
                                     if let (Some(c), Some(d)) = (&kermesse.city, &kermesse.department) {
                                        <span class="bg-white/10 backdrop-blur px-4 py-2 rounded-full text-sm font-semibold flex items-center gap-2 border border-white/10">
                                            <span>{"📍"}</span> { format!("{}, {}", c, d) }
                                        </span>
                                     }
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="container mx-auto px-6 -mt-20 relative z-20">
                    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                         // --- MENU ---
                         <div class="lg:col-span-2">
                            <div class="bg-white rounded-3xl shadow-xl p-8 mb-8">
                                <h2 class="text-3xl font-display font-bold mb-8 text-gray-800 flex items-center gap-3">
                                    <span class="text-orange-500">{"🍽️"}</span> { "Menú del Día" }
                                </h2>
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                    {
                                        detail_data.dishes.iter().map(|dish| {
                                            let dish_id = dish.id;
                                            let name = dish.name.clone();
                                            let price = dish.price;
                                            let available = dish.quantity_available;
                                            let on_open_modal = open_quantity_modal.clone();
                                            html! {
                                                <div class="bg-white rounded-2xl shadow-lg hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1 overflow-hidden border border-gray-100 flex flex-col group h-full">
                                                     <div class="h-48 bg-gray-100 overflow-hidden relative">
                                                        if let Some(img) = &dish.image_url { 
                                                            <img src={img.clone()} class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"/> 
                                                        } else {
                                                            <div class="w-full h-full flex items-center justify-center text-6xl bg-gradient-to-br from-orange-50 to-orange-100 text-orange-200">
                                                                {"🍲"}
                                                            </div>
                                                        }
                                                        if available == 0 {
                                                            <div class="absolute inset-0 bg-black/50 flex items-center justify-center backdrop-blur-sm">
                                                                <span class="text-white font-bold text-xl uppercase tracking-widest border-2 border-white px-4 py-2 rounded">{ "Agotado" }</span>
                                                            </div>
                                                        }
                                                     </div>
                                                     
                                                     <div class="p-6 flex flex-col flex-grow">
                                                         <div class="flex justify-between items-start mb-2">
                                                            <h3 class="text-xl font-bold text-gray-900 leading-tight">{ &dish.name }</h3>
                                                            <span class="font-display font-bold text-lg text-primary">{ format!("Bs. {:.0}", dish.price) }</span>
                                                         </div>
                                                         
                                                         <p class="text-gray-500 text-sm mb-6 line-clamp-2 flex-grow">{ &dish.description }</p>
                                                         
                                                         <button 
                                                            onclick={Callback::from(move |e: MouseEvent| {
                                                                e.stop_propagation();
                                                                if available > 0 {
                                                                    on_open_modal.emit((dish_id, name.clone(), price, available));
                                                                }
                                                            })}
                                                            disabled={available == 0}
                                                            class={format!("w-full py-3 rounded-xl font-bold transition flex items-center justify-center gap-2 group-btn {}", 
                                                                if available > 0 { 
                                                                    "bg-orange-50 text-orange-600 hover:bg-orange-500 hover:text-white" 
                                                                } else { 
                                                                    "bg-gray-100 text-gray-400 cursor-not-allowed" 
                                                                }
                                                            )}
                                                         >
                                                            { if available > 0 { "Agregar al Pedido" } else { "No disponible" } }
                                                            if available > 0 { <span class="group-btn-hover:translate-x-1 transition-transform">{"→"}</span> }
                                                         </button>
                                                     </div>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            </div>
                        </div>

                        // --- SIDEBAR ---
                        <div class="space-y-6">
                            // Share Card
                            <div class="bg-gradient-to-br from-indigo-500 to-purple-600 rounded-3xl shadow-xl p-8 text-white text-center">
                                <h3 class="text-2xl font-bold mb-2">{ "¡Comparte y Ayuda!" }</h3>
                                <p class="mb-6 opacity-90 text-sm">{ "Invita a tus amigos y familiares a participar en esta noble causa." }</p>
                                <div class="grid grid-cols-2 gap-3">
                                    <a 
                                        href={format!("https://wa.me/?text=¡Ayuda a {}! {}", &kermesse.beneficiary_name, format!("http://127.0.0.1:8000/kermesses/{}", id))}
                                        target="_blank"
                                        class="bg-white/20 hover:bg-white/30 backdrop-blur border border-white/20 text-white font-bold py-3 px-4 rounded-xl transition flex items-center justify-center gap-2"
                                    >
                                        <span>{"📱"}</span> { "WhatsApp" }
                                    </a>
                                    <a 
                                        href={format!("https://www.facebook.com/sharer/sharer.php?u={}", format!("http://127.0.0.1:8000/kermesses/{}", id))}
                                        target="_blank"
                                        class="bg-white/20 hover:bg-white/30 backdrop-blur border border-white/20 text-white font-bold py-3 px-4 rounded-xl transition flex items-center justify-center gap-2"
                                    >
                                        <span>{"Fb"}</span> { "Facebook" }
                                    </a>
                                </div>
                            </div>

                            // Collaborators
                            <div class="bg-white rounded-3xl shadow-xl p-8">
                                <h3 class="text-xl font-bold mb-6 text-gray-800 flex items-center gap-2">
                                    <span class="text-secondary">{"👥"}</span> { "Vendedores" }
                                </h3>
                                
                                <div class="space-y-4">
                                    if detail_data.collaborators.is_empty() {
                                        <div class="text-center py-8 bg-gray-50 rounded-2xl border border-dashed border-gray-200">
                                            <p class="text-gray-400 font-medium">{ "No hay vendedores aún." }</p>
                                        </div>
                                    } else {
                                        {
                                            detail_data.collaborators.iter().filter(|c| c.role == "SELLER").map(|collaborator| {
                                                html! {
                                                    <div class="flex items-center space-x-4 p-4 bg-gray-50 rounded-2xl hover:bg-white hover:shadow-md transition border border-transparent hover:border-gray-100">
                                                        <div class="w-12 h-12 bg-gradient-to-br from-secondary to-teal-400 text-white rounded-full flex items-center justify-center font-bold text-lg shadow-md shrink-0">
                                                            { collaborator.full_name.chars().next().unwrap_or('?') }
                                                        </div>
                                                        <div class="overflow-hidden">
                                                            <p class="font-bold text-gray-900 truncate">{ &collaborator.full_name }</p>
                                                            <p class="text-xs text-gray-500 flex items-center gap-1">
                                                                <span>{"📞"}</span>
                                                                { &collaborator.phone }
                                                            </p>
                                                        </div>
                                                        <a href={format!("https://wa.me/{}", collaborator.phone.replace(" ", ""))} target="_blank" class="ml-auto bg-green-100 text-green-600 w-10 h-10 rounded-full flex items-center justify-center hover:bg-green-500 hover:text-white transition">
                                                            {"💬"}
                                                        </a>
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    }
                                </div>
                                <div class="mt-6 p-4 bg-amber-50 rounded-xl text-xs text-amber-800 border border-amber-100 flex gap-3">
                                    <span class="text-xl">{"ℹ️"}</span>
                                    <p>{ "Contacta a uno de los colaboradores listados arriba para realizar tu pedido o reserva directamente." }</p>
                                </div>
                            </div>

                            // Organizer Dashboard (only for organizer)
                            if is_organizer {
                                <OrganizerDashboardV2 kermesse_id={id} />
                            }

                            // Collaboration Request Form
                            if user_ctx.user.is_some() {
                                <div class="bg-blue-50 rounded-3xl p-6 border border-blue-100">
                                    <h4 class="font-bold text-blue-900 mb-2">{ "¿Quieres ayudar?" }</h4>
                                    <CollaborationRequestForm kermesse_id={id} />
                                </div>
                            }
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
