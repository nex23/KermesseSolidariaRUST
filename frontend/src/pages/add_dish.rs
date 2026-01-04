use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use serde::Serialize;
use web_sys::HtmlInputElement;
use crate::context::UserContext;
use crate::router::Route;

#[derive(Serialize)]
struct CreateDishRequest {
    name: String,
    description: String,
    price: f64,
    quantity_available: i32,
    image_url: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub kermesse_id: i32,
}

#[function_component(AddDish)]
pub fn add_dish(props: &Props) -> Html {
    let name_ref = use_node_ref();
    let desc_ref = use_node_ref();
    let price_ref = use_node_ref();
    let qty_ref = use_node_ref();
    
    let navigator = use_navigator().unwrap();
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let kermesse_id = props.kermesse_id;

    if user_ctx.user.is_none() {
        navigator.push(&Route::Login);
        return html! {};
    }
    
    let token = user_ctx.user.as_ref().unwrap().token.clone();

    let onsubmit = {
        let name_ref = name_ref.clone();
        let desc_ref = desc_ref.clone();
        let price_ref = price_ref.clone();
        let qty_ref = qty_ref.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name_ref.cast::<HtmlInputElement>().unwrap().value();
            let description = desc_ref.cast::<HtmlInputElement>().unwrap().value();
            let price = price_ref.cast::<HtmlInputElement>().unwrap().value().parse::<f64>().unwrap_or(0.0);
            let quantity_available = qty_ref.cast::<HtmlInputElement>().unwrap().value().parse::<i32>().unwrap_or(0);
            let token = token.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let request = CreateDishRequest { name, description, price, quantity_available, image_url: None };
                let body = serde_json::to_string(&request).unwrap();
                let url = format!("http://127.0.0.1:8080/kermesses/{}/dishes", kermesse_id);
                let resp = Request::post(&url)
                    .header("Authorization", &format!("Bearer {}", token))
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await;

                if let Ok(resp) = resp {
                    if resp.ok() {
                        gloo_dialogs::alert("Plato agregado!");
                        navigator.push(&Route::KermesseDetail { id: kermesse_id });
                    } else {
                        gloo_dialogs::alert("Error al agregar plato");
                    }
                } else {
                     gloo_dialogs::alert("Error de conexión");
                }
            });
        })
    };

    html! {
        <div class="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-sans">
             <div class="max-w-md mx-auto bg-white p-8 rounded-xl shadow-lg">
                <h2 class="text-2xl font-bold text-gray-900 mb-6">{ "Agregar Plato" }</h2>
                
                <form onsubmit={onsubmit} class="space-y-6">
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{ "Nombre del Plato" }</label>
                        <input ref={name_ref} type="text" required=true class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border" />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{ "Descripción" }</label>
                        <textarea ref={desc_ref} required=true rows="2" class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border"></textarea>
                    </div>

                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700">{ "Precio (Bs)" }</label>
                            <input ref={price_ref} type="number" step="0.5" required=true class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border" />
                        </div>
                         <div>
                            <label class="block text-sm font-medium text-gray-700">{ "Cantidad" }</label>
                            <input ref={qty_ref} type="number" required=true class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border" />
                        </div>
                    </div>

                    <div class="pt-4">
                        <button type="submit" class="w-full bg-primary text-white font-bold py-2 px-4 rounded-xl shadow hover:bg-red-500 transition">
                            { "Guardar Plato" }
                        </button>
                    </div>
                </form>
             </div>
        </div>
    }
}
