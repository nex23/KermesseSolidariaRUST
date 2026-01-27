use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use crate::router::Route;
use crate::context::UserContext;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
struct EditKermesseForm {
    name: String,
    description: String,
    event_date: String,
    beneficiary_name: String,
    beneficiary_reason: String,
    beneficiary_image_url: String, // Optional but simplified for form
    start_time: String,
    end_time: String,
    financial_goal: String,
    qr_code_url: String,
    department: String,
    city: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub kermesse_id: i32,
}

#[function_component(EditKermesse)]
pub fn edit_kermesse(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let kermesse_id = props.kermesse_id;
    
    let form_data = use_state(|| EditKermesseForm::default());
    let loading = use_state(|| true);
    let error_msg = use_state(|| None::<String>);

    // Fetch existing data
    {
        let form_data = form_data.clone();
        let loading = loading.clone();
        use_effect_with(kermesse_id, move |id| {
            let id = *id;
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://127.0.0.1:8080/kermesses/{}", id);
                if let Ok(resp) = Request::get(&url).send().await {
                   if let Ok(data) = resp.json::<serde_json::Value>().await {
                       // KermesseDetailResponse flattens kermesse fields into root
                       let k = &data;
                       let mut form = EditKermesseForm::default();
                       form.name = k["name"].as_str().unwrap_or("").to_string();
                       form.description = k["description"].as_str().unwrap_or("").to_string();
                       form.event_date = k["event_date"].as_str().unwrap_or("").to_string();
                       form.beneficiary_name = k["beneficiary_name"].as_str().unwrap_or("").to_string();
                       form.beneficiary_reason = k["beneficiary_reason"].as_str().unwrap_or("").to_string();
                       form.beneficiary_image_url = k["beneficiary_image_url"].as_str().unwrap_or("").to_string();
                       form.start_time = k["start_time"].as_str().unwrap_or("").to_string();
                       form.end_time = k["end_time"].as_str().unwrap_or("").to_string();
                       // Financial goal can be null or string or number
                       if let Some(goal) = k["financial_goal"].as_f64() {
                            form.financial_goal = goal.to_string();
                       } else if let Some(goal_str) = k["financial_goal"].as_str() {
                            form.financial_goal = goal_str.to_string();
                       }
                       form.qr_code_url = k["qr_code_url"].as_str().unwrap_or("").to_string();
                       form.department = k["department"].as_str().unwrap_or("").to_string();
                       form.city = k["city"].as_str().unwrap_or("").to_string();
                       
                       form_data.set(form);
                   }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_submit = {
        let form_data = form_data.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let error_msg = error_msg.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let form = (*form_data).clone();
            let user_token = user_ctx.user.as_ref().map(|u| u.token.clone());
            let navigator = navigator.clone();
            let error_msg = error_msg.clone();

            if let Some(token) = user_token {
                wasm_bindgen_futures::spawn_local(async move {
                    gloo_console::log!("Submitting update for kermesse:", kermesse_id);
                    let financial_goal_decimal = if form.financial_goal.is_empty() { 
                        None 
                    } else { 
                        match form.financial_goal.parse::<f64>() {
                            Ok(val) => {
                                match rust_decimal::Decimal::try_from(val) {
                                    Ok(d) => Some(d),
                                    Err(e) => {
                                        gloo_console::error!("Decimal conversion error:", e.to_string());
                                        None
                                    }
                                }
                            },
                            Err(e) => {
                                gloo_console::error!("Float parse error:", e.to_string());
                                None
                            }
                        }
                    };

                    let body = serde_json::json!({
                        "name": if form.name.is_empty() { None } else { Some(form.name) },
                        "description": if form.description.is_empty() { None } else { Some(form.description) },
                        "event_date": if form.event_date.is_empty() { None } else { Some(form.event_date) },
                        "beneficiary_name": if form.beneficiary_name.is_empty() { None } else { Some(form.beneficiary_name) },
                        "beneficiary_reason": if form.beneficiary_reason.is_empty() { None } else { Some(form.beneficiary_reason) },
                        "beneficiary_image_url": if form.beneficiary_image_url.is_empty() { None } else { Some(form.beneficiary_image_url) },
                        "start_time": if form.start_time.is_empty() { None } else { Some(form.start_time) },
                        "end_time": if form.end_time.is_empty() { None } else { Some(form.end_time) },
                        "financial_goal": financial_goal_decimal,
                        "qr_code_url": if form.qr_code_url.is_empty() { None } else { Some(form.qr_code_url) },
                        "department": if form.department.is_empty() { None } else { Some(form.department) },
                        "city": if form.city.is_empty() { None } else { Some(form.city) },
                    });

                    gloo_console::log!("Sending body:", body.to_string());

                    let url = format!("http://127.0.0.1:8080/kermesses/{}", kermesse_id);
                    let resp = Request::put(&url)
                        .header("Authorization", &format!("Bearer {}", token))
                        .header("Content-Type", "application/json")
                        .body(body.to_string())
                        .send()
                        .await;

                    match resp {
                        Ok(r) => {
                            if r.ok() {
                                gloo_console::log!("Update successful");
                                navigator.push(&Route::KermesseDetail { id: kermesse_id });
                            } else {
                                let text = r.text().await.unwrap_or("Error desconocido".into());
                                gloo_console::error!("Server error:", &text);
                                error_msg.set(Some(format!("Error: {}", text)));
                            }
                        },
                        Err(e) => {
                            gloo_console::error!("Network error:", e.to_string());
                            error_msg.set(Some("Error de conexión".into()));
                        },
                    }
                });
            } else {
                gloo_console::error!("No token found");
            }
        })
    };

    let on_input_change = |field: &'static str, form_data: UseStateHandle<EditKermesseForm>| {
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut new_data = (*form_data).clone();
            let value = input.value();
            match field {
                "name" => new_data.name = value,
                "description" => new_data.description = value,
                "event_date" => new_data.event_date = value,
                "beneficiary_name" => new_data.beneficiary_name = value,
                "beneficiary_reason" => new_data.beneficiary_reason = value,
                "beneficiary_image_url" => new_data.beneficiary_image_url = value,
                "start_time" => new_data.start_time = value,
                "end_time" => new_data.end_time = value,
                "financial_goal" => new_data.financial_goal = value,
                "qr_code_url" => new_data.qr_code_url = value,
                "department" => new_data.department = value,
                "city" => new_data.city = value,
                _ => (),
            }
            form_data.set(new_data);
        })
    };

    if *loading {
        return html! { <div class="text-center py-20">{ "Cargando datos..." }</div> };
    }

    html! {
        <div class="min-h-screen bg-gray-50 p-6 flex justify-center">
            <div class="w-full max-w-2xl bg-white rounded-xl shadow-lg p-8">
                 <div class="flex justify-between items-center mb-6">
                    <h2 class="text-2xl font-bold text-gray-800">{ "Editar Kermesse" }</h2>
                    <button onclick={Callback::from(move |_| navigator.push(&Route::KermesseDetail { id: kermesse_id }))} class="text-gray-500 hover:text-gray-700">
                        { "Cancelar" }
                    </button>
                 </div>

                 if let Some(msg) = &*error_msg {
                     <div class="bg-red-100 border-l-4 border-red-500 text-red-700 p-4 mb-4" role="alert">
                         <p>{ msg }</p>
                     </div>
                 }
                 
                 <form onsubmit={on_submit} class="space-y-4">
                    <div>
                        <label class="block text-gray-700 font-bold mb-2">{ "Nombre de la Kermesse" }</label>
                        <input type="text" value={form_data.name.clone()} oninput={on_input_change("name", form_data.clone())} class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary" required=true />
                    </div>
                     <div>
                        <label class="block text-gray-700 font-bold mb-2">{ "Descripción" }</label>
                        <textarea value={form_data.description.clone()} oninput={on_input_change("description", form_data.clone())} class="w-full border rounded px-3 py-2 h-24 focus:outline-none focus:ring-2 focus:ring-primary" required=true></textarea>
                    </div>
                     <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-gray-700 font-bold mb-2">{ "Fecha del Evento" }</label>
                            <input type="date" value={form_data.event_date.clone()} oninput={on_input_change("event_date", form_data.clone())} class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary" required=true />
                        </div>
                         <div>
                            <label class="block text-gray-700 font-bold mb-2">{ "Meta Financiera (Bs)" }</label>
                            <input type="number" step="0.01" value={form_data.financial_goal.clone()} oninput={on_input_change("financial_goal", form_data.clone())} class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary" />
                        </div>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-gray-700 font-bold mb-2">{ "Departamento" }</label>
                            <select value={form_data.department.clone()} oninput={on_input_change("department", form_data.clone())} class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary">
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
                        </div>
                        <div>
                            <label class="block text-gray-700 font-bold mb-2">{ "Ciudad" }</label>
                            <input type="text" value={form_data.city.clone()} oninput={on_input_change("city", form_data.clone())} class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary" />
                        </div>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-gray-700 font-bold mb-2">{ "Hora Inicio" }</label>
                            <input type="time" value={form_data.start_time.clone()} oninput={on_input_change("start_time", form_data.clone())} class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary" />
                        </div>
                        <div>
                            <label class="block text-gray-700 font-bold mb-2">{ "Hora Fin" }</label>
                            <input type="time" value={form_data.end_time.clone()} oninput={on_input_change("end_time", form_data.clone())} class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary" />
                        </div>
                    </div>

                    <div class="border-t pt-4 mt-4">
                        <h3 class="text-lg font-semibold mb-3 text-gray-600">{ "Información del Beneficiario" }</h3>
                         <div>
                            <label class="block text-gray-700 font-bold mb-2">{ "Nombre Beneficiario" }</label>
                            <input type="text" value={form_data.beneficiary_name.clone()} oninput={on_input_change("beneficiary_name", form_data.clone())} class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary" required=true />
                        </div>
                         <div class="mt-4">
                            <label class="block text-gray-700 font-bold mb-2">{ "Motivo / Causa" }</label>
                            <input type="text" value={form_data.beneficiary_reason.clone()} oninput={on_input_change("beneficiary_reason", form_data.clone())} class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary" required=true />
                        </div>
                         <div class="mt-4">
                            <label class="block text-gray-700 font-bold mb-2">{ "URL Imagen Beneficiario" }</label>
                            <input type="text" value={form_data.beneficiary_image_url.clone()} oninput={on_input_change("beneficiary_image_url", form_data.clone())} class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary" />
                        </div>
                         <div class="mt-4">
                            <label class="block text-gray-700 font-bold mb-2">{ "URL Imagen QR (Cobros)" }</label>
                            <input type="text" value={form_data.qr_code_url.clone()} oninput={on_input_change("qr_code_url", form_data.clone())} class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary" />
                        </div>
                    </div>

                    <div class="pt-6">
                        <button type="submit" class="w-full bg-primary text-white font-bold py-3 rounded-lg hover:bg-orange-600 transition shadow-lg">
                            { "Guardar Cambios" }
                        </button>
                    </div>
                 </form>
            </div>
        </div>
    }
}
