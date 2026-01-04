use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use serde::Serialize;
use web_sys::HtmlInputElement;
use crate::context::UserContext;
use crate::router::Route;

#[derive(Serialize)]
struct CreateKermesseRequest {
    name: String,
    description: String,
    event_date: String,
    pub beneficiary_name: String,
    pub beneficiary_reason: String,
    pub beneficiary_image_url: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let name_ref = use_node_ref();
    let desc_ref = use_node_ref();
    let date_ref = use_node_ref();
    let ben_name_ref = use_node_ref();
    let ben_reason_ref = use_node_ref();
    let start_time_ref = use_node_ref();
    let end_time_ref = use_node_ref();
    let img_url_ref = use_node_ref();
    
    let navigator = use_navigator().unwrap();
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");

    if user_ctx.user.is_none() {
        navigator.push(&Route::Login);
        return html! {};
    }
    
    let token = user_ctx.user.as_ref().unwrap().token.clone();

    let onsubmit = {
        let name_ref = name_ref.clone();
        let desc_ref = desc_ref.clone();
        let date_ref = date_ref.clone();
        let ben_name_ref = ben_name_ref.clone();
        let ben_reason_ref = ben_reason_ref.clone();
        let start_time_ref = start_time_ref.clone();
        let end_time_ref = end_time_ref.clone();
        let img_url_ref = img_url_ref.clone();
        let token = token.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name_ref.cast::<HtmlInputElement>().unwrap().value();
            let description = desc_ref.cast::<HtmlInputElement>().unwrap().value();
            let event_date = date_ref.cast::<HtmlInputElement>().unwrap().value();
            let beneficiary_name = ben_name_ref.cast::<HtmlInputElement>().unwrap().value();
            let beneficiary_reason = ben_reason_ref.cast::<HtmlInputElement>().unwrap().value();
            let start_time = start_time_ref.cast::<HtmlInputElement>().unwrap().value();
            let end_time = end_time_ref.cast::<HtmlInputElement>().unwrap().value();
            let beneficiary_image_url = img_url_ref.cast::<HtmlInputElement>().unwrap().value();

            let start_time = if start_time.is_empty() { None } else { Some(start_time) };
            let end_time = if end_time.is_empty() { None } else { Some(end_time) };
            let beneficiary_image_url = if beneficiary_image_url.is_empty() { None } else { Some(beneficiary_image_url) };

            let token = token.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let request = CreateKermesseRequest { 
                    name, 
                    description, 
                    event_date, 
                    beneficiary_name, 
                    beneficiary_reason,
                    beneficiary_image_url,
                    start_time,
                    end_time
                };
                let body = serde_json::to_string(&request).unwrap();
                let resp = Request::post("http://127.0.0.1:8080/kermesses")
                    .header("Authorization", &format!("Bearer {}", token))
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await;

                if let Ok(resp) = resp {
                    if resp.ok() {
                        gloo_dialogs::alert("Kermesse creada exitosamente!");
                        navigator.push(&Route::Home);
                    } else {
                        gloo_dialogs::alert("Error al crear kermesse");
                    }
                } else {
                     gloo_dialogs::alert("Error de conexión");
                }
            });
        })
    };

    html! {
        <div class="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-sans">
             <div class="max-w-2xl mx-auto bg-white p-8 rounded-xl shadow-lg">
                <h2 class="text-3xl font-bold text-gray-900 mb-6 border-b pb-4">{ "Crear Nueva Kermesse" }</h2>
                
                <form onsubmit={onsubmit} class="space-y-6">
                    <div class="grid grid-cols-1 md::grid-cols-2 gap-6">
                        <div>
                            <label class="block text-sm font-medium text-gray-700">{ "Nombre del Evento" }</label>
                            <input ref={name_ref} type="text" required=true class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border" />
                        </div>
                         <div>
                            <label class="block text-sm font-medium text-gray-700">{ "Fecha del Evento" }</label>
                            <input ref={date_ref} type="date" required=true class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border" />
                        </div>
                    </div>

                     <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label class="block text-sm font-medium text-gray-700">{ "Hora Inicio" }</label>
                            <input ref={start_time_ref} type="time" class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border" />
                        </div>
                         <div>
                            <label class="block text-sm font-medium text-gray-700">{ "Hora Fin" }</label>
                            <input ref={end_time_ref} type="time" class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border" />
                        </div>
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{ "Descripción General" }</label>
                        <textarea ref={desc_ref} required=true rows="3" class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border"></textarea>
                    </div>

                    <div class="bg-gray-50 p-4 rounded-lg border border-gray-200">
                        <h3 class="text-lg font-medium text-gray-900 mb-4">{ "Información del Beneficiario" }</h3>
                        <div class="space-y-4">
                            <div>
                                <label class="block text-sm font-medium text-gray-700">{ "Nombre del Beneficiario" }</label>
                                <input ref={ben_name_ref} type="text" required=true class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border" />
                            </div>
                             <div>
                                <label class="block text-sm font-medium text-gray-700">{ "URL Foto del Beneficiario" }</label>
                                <input ref={img_url_ref} type="url" placeholder="https://ejemplo.com/foto.jpg" class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border" />
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-700">{ "Motivo / Historia" }</label>
                                <textarea ref={ben_reason_ref} required=true rows="2" class="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-primary focus:border-primary sm:text-sm p-2 border"></textarea>
                            </div>
                        </div>
                    </div>

                    <div class="pt-4">
                        <button type="submit" class="w-full bg-primary text-white font-bold py-3 px-4 rounded-xl shadow hover:bg-red-500 transition transform hover:scale-[1.02]">
                            { "Publicar Kermesse" }
                        </button>
                    </div>
                </form>
             </div>
        </div>
    }
}
