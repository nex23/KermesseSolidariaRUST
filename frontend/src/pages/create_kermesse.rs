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
    pub financial_goal: Option<f64>,
    pub qr_code_url: Option<String>,
    pub department: Option<String>,
    pub city: Option<String>,
}

#[function_component(CreateKermesse)]
pub fn create_kermesse() -> Html {
    let name_ref = use_node_ref();
    let desc_ref = use_node_ref();
    let date_ref = use_node_ref();
    let ben_name_ref = use_node_ref();
    let ben_reason_ref = use_node_ref();
    let start_time_ref = use_node_ref();
    let end_time_ref = use_node_ref();
    let img_url_ref = use_node_ref();
    let financial_goal_ref = use_node_ref();
    let qr_code_ref = use_node_ref();
    let dept_ref = use_node_ref();
    let city_ref = use_node_ref();
    
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
        let financial_goal_ref = financial_goal_ref.clone();
        let qr_code_ref = qr_code_ref.clone();
        let dept_ref = dept_ref.clone();
        let city_ref = city_ref.clone();
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
            let financial_goal_str = financial_goal_ref.cast::<HtmlInputElement>().unwrap().value();
            let qr_code_url = qr_code_ref.cast::<HtmlInputElement>().unwrap().value();
            let department_val = dept_ref.cast::<HtmlInputElement>().unwrap().value();
            let city_val = city_ref.cast::<HtmlInputElement>().unwrap().value();

            let start_time = if start_time.is_empty() { None } else { Some(start_time) };
            let end_time = if end_time.is_empty() { None } else { Some(end_time) };
            let beneficiary_image_url = if beneficiary_image_url.is_empty() { None } else { Some(beneficiary_image_url) };
            let financial_goal = if financial_goal_str.is_empty() { None } else { financial_goal_str.parse::<f64>().ok() };
            let qr_code_url = if qr_code_url.is_empty() { None } else { Some(qr_code_url) };
            let department = if department_val.is_empty() { None } else { Some(department_val) };
            let city = if city_val.is_empty() { None } else { Some(city_val) };

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
                    end_time,
                    financial_goal,
                    qr_code_url,
                    department,
                    city,
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
        <div class="min-h-screen bg-gray-50 font-sans pb-12">
            // Navbar placeholder if needed, or just padding
            <div class="bg-white shadow-sm border-b border-gray-100 py-4 px-6 mb-8 flex items-center justify-between">
                <button onclick={Callback::from(move |_| navigator.push(&Route::Home))} class="flex items-center text-gray-500 hover:text-orange-600 font-bold transition">
                    <span class="mr-2">{"←"}</span> { "Volver al Inicio" }
                </button>
                <h1 class="text-xl font-display font-bold text-gray-800 tracking-tight">{ "Panel de Organizador" }</h1>
            </div>

             <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="bg-white rounded-3xl shadow-xl overflow-hidden">
                    <div class="bg-gradient-to-r from-orange-500 to-red-600 p-8 text-white text-center">
                        <h2 class="text-3xl font-display font-bold mb-2">{ "Crear Nueva Kermesse" }</h2>
                        <p class="opacity-90">{ "Completa la información para lanzar tu evento solidario." }</p>
                    </div>
                
                    <form onsubmit={onsubmit} class="p-8 space-y-8">
                        // Section: Event Details
                        <div class="space-y-6">
                            <h3 class="text-lg font-bold text-gray-800 border-b pb-2 flex items-center gap-2">
                                <span class="text-orange-500">{"📅"}</span> { "Detalles del Evento" }
                            </h3>
                            
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div>
                                    <label class="block text-sm font-bold text-gray-700 mb-1">{ "Nombre del Evento" }</label>
                                    <input ref={name_ref} type="text" required=true placeholder="Ej. Festival Solidario por la Vida" class="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition outline-none" />
                                </div>
                                 <div>
                                    <label class="block text-sm font-bold text-gray-700 mb-1">{ "Fecha del Evento" }</label>
                                    <input ref={date_ref} type="date" required=true class="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition outline-none" />
                                </div>
                            </div>
    
                             <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div>
                                    <label class="block text-sm font-bold text-gray-700 mb-1">{ "Hora Inicio" }</label>
                                    <input ref={start_time_ref} type="time" class="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition outline-none" />
                                </div>
                                 <div>
                                    <label class="block text-sm font-bold text-gray-700 mb-1">{ "Hora Fin" }</label>
                                    <input ref={end_time_ref} type="time" class="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition outline-none" />
                                </div>
                            </div>
    
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div>
                                    <label class="block text-sm font-bold text-gray-700 mb-1">{ "Departamento" }</label>
                                    <div class="relative">
                                        <select ref={dept_ref} class="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition outline-none appearance-none">
                                            <option value="">{ "Seleccionar..." }</option>
                                            <option value="La Paz">{ "La Paz" }</option>
                                            <option value="Santa Cruz">{ "Santa Cruz" }</option>
                                            <option value="Cochabamba">{ "Cochabamba" }</option>
                                            <option value="Oruro">{ "Oruro" }</option>
                                            <option value="Potosí">{ "Potosí" }</option>
                                            <option value="Chuquisaca">{ "Chuquisaca" }</option>
                                            <option value="Tarija">{ "Tarija" }</option>
                                            <option value="Beni">{ "Beni" }</option>
                                            <option value="Pando">{ "Pando" }</option>
                                        </select>
                                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-4 text-gray-500">
                                            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                                        </div>
                                    </div>
                                </div>
                                <div>
                                    <label class="block text-sm font-bold text-gray-700 mb-1">{ "Ciudad / Localidad" }</label>
                                    <input ref={city_ref} type="text" placeholder="Ej. El Alto, Quillacollo" class="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition outline-none" />
                                </div>
                            </div>
                            
                            <div>
                                <label class="block text-sm font-bold text-gray-700 mb-1">{ "Descripción General" }</label>
                                <textarea ref={desc_ref} required=true rows="3" placeholder="Describe de qué trata el evento..." class="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition outline-none"></textarea>
                            </div>
                        </div>
    
                        // Section: Beneficiary
                        <div class="space-y-6">
                            <h3 class="text-lg font-bold text-gray-800 border-b pb-2 flex items-center gap-2">
                                <span class="text-orange-500">{"❤️"}</span> { "Información del Beneficiario" }
                            </h3>
                            <div class="bg-orange-50 p-6 rounded-2xl border border-orange-100 space-y-6">
                                <div>
                                    <label class="block text-sm font-bold text-gray-700 mb-1">{ "Nombre del Beneficiario" }</label>
                                    <input ref={ben_name_ref} type="text" required=true placeholder="Nombre de la persona o causa" class="w-full bg-white border border-orange-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition outline-none" />
                                </div>
                                 <div>
                                    <label class="block text-sm font-bold text-gray-700 mb-1">{ "URL Foto del Beneficiario" }</label>
                                    <input ref={img_url_ref} type="url" placeholder="https://ejemplo.com/foto.jpg" class="w-full bg-white border border-orange-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition outline-none" />
                                </div>
                                <div>
                                    <label class="block text-sm font-bold text-gray-700 mb-1">{ "Motivo / Historia" }</label>
                                    <textarea ref={ben_reason_ref} required=true rows="2" placeholder="¿Por qué necesitamos ayuda?" class="w-full bg-white border border-orange-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-orange-500 focus:border-orange-500 transition outline-none"></textarea>
                                </div>
                            </div>
                        </div>
    
                        // Section: Financial
                        <div class="space-y-6">
                            <h3 class="text-lg font-bold text-gray-800 border-b pb-2 flex items-center gap-2">
                                <span class="text-orange-500">{"💰"}</span> { "Meta Financiera" }
                            </h3>
                            <div class="bg-blue-50 p-6 rounded-2xl border border-blue-100 space-y-6">
                                <div>
                                    <label class="block text-sm font-bold text-blue-900 mb-1">{ "Meta a Recaudar (Bs)" }</label>
                                    <input ref={financial_goal_ref} type="number" step="0.01" placeholder="5000.00" class="w-full bg-white border border-blue-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition outline-none font-bold text-blue-800" />
                                </div>
                                <div>
                                    <label class="block text-sm font-bold text-blue-900 mb-1">{ "URL de QR para Pagos" }</label>
                                    <input ref={qr_code_ref} type="url" placeholder="https://ejemplo.com/qr.png" class="w-full bg-white border border-blue-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition outline-none" />
                                </div>
                            </div>
                        </div>
    
                        <div class="pt-6">
                            <button type="submit" class="w-full bg-gradient-to-r from-orange-500 to-red-600 text-white font-bold text-lg py-4 px-6 rounded-xl shadow-lg hover:shadow-orange-500/30 hover:to-red-700 transition transform hover:-translate-y-1 active:scale-95">
                                { "🚀 Publicar Kermesse" }
                            </button>
                        </div>
                    </form>
                </div>
             </div>
        </div>
    }
}
