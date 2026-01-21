use yew::prelude::*;
use yew_router::prelude::*;
use crate::context::{UserContext, CartContext, CartAction};
use crate::router::Route;
use reqwasm::http::Request;
use serde::Serialize;
use gloo_console::log;

#[derive(Serialize)]
struct CreateSaleRequest {
    kermesse_id: i32,
    customer_name: String,
    items: Vec<SaleItemRequest>,
    delivery_method: String,
    delivery_address: Option<String>,
    contact_phone: Option<String>,
    payment_method: String,
}

#[derive(Serialize)]
struct SaleItemRequest {
    dish_id: i32,
    quantity: i32,
}

#[function_component(Checkout)]
pub fn checkout() -> Html {
    let cart_ctx = use_context::<CartContext>().expect("No CartContext found");
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let navigator = use_navigator().unwrap();

    let name_ref = use_node_ref();
    let phone_ref = use_node_ref();
    let address_ref = use_node_ref();

    let delivery_method = use_state(|| "PICKUP".to_string());
    let payment_method = use_state(|| "QR".to_string());
    let is_submitting = use_state(|| false);

    if cart_ctx.state.items.is_empty() {
        return html! {
            <div class="min-h-screen flex flex-col items-center justify-center bg-gray-50 p-4">
                <p class="text-xl text-gray-600 mb-4">{ "Tu carrito está vacío." }</p>
                <button onclick={Callback::from(move |_| navigator.push(&Route::Home))} class="bg-primary text-white px-6 py-2 rounded-lg">
                    { "Volver al Inicio" }
                </button>
            </div>
        };
    }

    let on_submit = {
        let cart_ctx = cart_ctx.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let name_ref = name_ref.clone();
        let phone_ref = phone_ref.clone();
        let address_ref = address_ref.clone();
        let delivery_method = delivery_method.clone();
        let payment_method = payment_method.clone();
        let is_submitting = is_submitting.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            is_submitting.set(true);

            let name = name_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();
            let phone = phone_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();
            let address = if *delivery_method == "DELIVERY" {
                Some(address_ref.cast::<web_sys::HtmlTextAreaElement>().unwrap().value())
            } else {
                None
            };
            
            let items: Vec<SaleItemRequest> = cart_ctx.state.items.iter().map(|i| SaleItemRequest {
                dish_id: i.dish_id,
                quantity: i.quantity,
            }).collect();

            let kermesse_id = cart_ctx.state.items[0].kermesse_id;

            let request = CreateSaleRequest {
                kermesse_id,
                customer_name: name,
                items,
                delivery_method: (*delivery_method).clone(),
                delivery_address: address,
                contact_phone: Some(phone),
                payment_method: (*payment_method).clone(),
            };

            let cart_ctx = cart_ctx.clone();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();
            let is_submitting = is_submitting.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let body = serde_json::to_string(&request).unwrap();
                let mut req = Request::post("http://127.0.0.1:8080/sales")
                    .header("Content-Type", "application/json")
                    .body(body);

                if let Some(user) = &user_ctx.user {
                    req = req.header("Authorization", &format!("Bearer {}", user.token));
                }

                match req.send().await {
                   Ok(resp) => {
                       if resp.ok() {
                           gloo_dialogs::alert("¡Pedido Realizado con Éxito!");
                           cart_ctx.dispatch.emit(CartAction::Clear);
                           navigator.push(&Route::Home); // Or Order Success Page
                       } else {
                           gloo_dialogs::alert("Error al procesar el pedido.");
                       }
                   },
                   Err(_) => gloo_dialogs::alert("Error de conexión."),
                }
                is_submitting.set(false);
            });
        })
    };

    html! {
        <div class="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-3xl mx-auto bg-white rounded-2xl shadow-xl overflow-hidden">
                <div class="bg-primary px-8 py-6 text-white">
                    <h1 class="text-3xl font-bold">{ "Finalizar Compra" }</h1>
                    <p class="opacity-90">{ "Completa tus datos para confirmar el pedido" }</p>
                </div>

                <form onsubmit={on_submit} class="p-8 space-y-8">
                    // 1. Datos Personales
                    <section>
                        <h2 class="text-xl font-bold text-gray-800 mb-4 border-b pb-2">{ "1. Tus Datos" }</h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                            <div>
                                <label class="block text-sm font-medium text-gray-700 mb-1">{ "Nombre Completo" }</label>
                                <input ref={name_ref} type="text" required=true class="w-full rounded-lg border-gray-300 shadow-sm focus:ring-primary focus:border-primary p-2 border" placeholder="Juan Perez" value={user_ctx.user.as_ref().map(|u| u.username.clone()).unwrap_or_default()} />
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-700 mb-1">{ "Teléfono / WhatsApp" }</label>
                                <input ref={phone_ref} type="tel" required=true class="w-full rounded-lg border-gray-300 shadow-sm focus:ring-primary focus:border-primary p-2 border" placeholder="70012345" />
                            </div>
                        </div>
                    </section>

                    // 2. Método de Entrega
                    <section>
                        <h2 class="text-xl font-bold text-gray-800 mb-4 border-b pb-2">{ "2. Método de Entrega" }</h2>
                        <div class="grid grid-cols-3 gap-4 mb-4">
                            <button type="button" 
                                onclick={let dm = delivery_method.clone(); Callback::from(move |_| dm.set("PICKUP".to_string()))}
                                class={format!("p-4 rounded-xl border-2 text-center transition {}", if *delivery_method == "PICKUP" { "border-primary bg-red-50 text-primary font-bold" } else { "border-gray-200 text-gray-500 hover:border-gray-300" })}
                            >
                                <div class="text-2xl mb-2">{"🥡"}</div>
                                { "Para Llevar (Recojo)" }
                            </button>
                            <button type="button" 
                                onclick={let dm = delivery_method.clone(); Callback::from(move |_| dm.set("EAT_HERE".to_string()))}
                                class={format!("p-4 rounded-xl border-2 text-center transition {}", if *delivery_method == "EAT_HERE" { "border-primary bg-red-50 text-primary font-bold" } else { "border-gray-200 text-gray-500 hover:border-gray-300" })}
                            >
                                <div class="text-2xl mb-2">{"🍽️"}</div>
                                { "Comer Aquí" }
                            </button>
                            <button type="button" 
                                onclick={let dm = delivery_method.clone(); Callback::from(move |_| dm.set("DELIVERY".to_string()))}
                                class={format!("p-4 rounded-xl border-2 text-center transition {}", if *delivery_method == "DELIVERY" { "border-primary bg-red-50 text-primary font-bold" } else { "border-gray-200 text-gray-500 hover:border-gray-300" })}
                            >
                                <div class="text-2xl mb-2">{"🛵"}</div>
                                { "Delivery" }
                            </button>
                        </div>

                        if *delivery_method == "DELIVERY" {
                            <div class="animate-fade-in-down">
                                <label class="block text-sm font-medium text-gray-700 mb-1">{ "Dirección de Entrega" }</label>
                                <textarea ref={address_ref} required=true rows="2" class="w-full rounded-lg border-gray-300 shadow-sm focus:ring-primary focus:border-primary p-2 border" placeholder="Barrio X, Calle Y #123 (Referencia: Frente a la plaza)"></textarea>
                            </div>
                        }
                    </section>
                    
                    // 3. Método de Pago
                    <section>
                        <h2 class="text-xl font-bold text-gray-800 mb-4 border-b pb-2">{ "3. Método de Pago" }</h2>
                        <div class="grid grid-cols-2 gap-4 mb-4">
                            <button type="button" 
                                onclick={let pm = payment_method.clone(); Callback::from(move |_| pm.set("QR".to_string()))}
                                class={format!("p-4 rounded-xl border-2 text-center transition {}", if *payment_method == "QR" { "border-primary bg-red-50 text-primary font-bold" } else { "border-gray-200 text-gray-500 hover:border-gray-300" })}
                            >
                                <div class="text-2xl mb-2">{"📱"}</div>
                                { "Pago QR" }
                            </button>
                            <button type="button" 
                                onclick={let pm = payment_method.clone(); Callback::from(move |_| pm.set("CASH".to_string()))}
                                class={format!("p-4 rounded-xl border-2 text-center transition {}", if *payment_method == "CASH" { "border-primary bg-red-50 text-primary font-bold" } else { "border-gray-200 text-gray-500 hover:border-gray-300" })}
                            >
                                <div class="text-2xl mb-2">{"💵"}</div>
                                { "Efectivo (Contra-entrega)" }
                            </button>
                        </div>
                        
                         if *payment_method == "QR" {
                            <div class="bg-blue-50 border border-blue-200 rounded-xl p-4 flex gap-4 animate-fade-in-down">
                                <div class="flex-shrink-0 bg-white p-2 rounded-lg shadow-sm">
                                    // Placeholder QR. In real app, fetch from Kermesse
                                    <div class="w-24 h-24 bg-gray-200 flex items-center justify-center text-xs text-center text-gray-500">
                                        {"QR del Evento"}
                                    </div>
                                </div>
                                <div>
                                    <h4 class="font-bold text-blue-800">{ "Instrucciones QR" }</h4>
                                    <p class="text-sm text-blue-700 mb-2">{ "1. Escanea el QR y realiza el pago." }</p>
                                    <p class="text-sm text-blue-700 mb-2">{ "2. Envía el comprobante por WhatsApp al organizador." }</p>
                                    <a href="#" class="inline-flex items-center text-green-600 font-bold hover:underline">
                                        <span>{"💬 Enviar Comprobante"}</span>
                                    </a>
                                </div>
                            </div>
                        }
                    </section>

                    // Resumen
                    <div class="border-t pt-6">
                        <div class="flex justify-between items-center text-2xl font-bold text-gray-800 mb-6">
                            <span>{ "Total a Pagar" }</span>
                            <span>{ format!("Bs. {:.2}", cart_ctx.state.total()) }</span>
                        </div>
                        <button type="submit" disabled={*is_submitting} class="w-full bg-green-600 text-white font-bold py-4 rounded-xl shadow-lg hover:bg-green-700 transition transform hover:scale-[1.02] disabled:opacity-50 disabled:scale-100">
                            { if *is_submitting { "Procesando..." } else { "Confirmar Pedido" } }
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
