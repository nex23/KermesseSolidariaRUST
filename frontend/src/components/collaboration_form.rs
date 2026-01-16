use yew::prelude::*;
use reqwasm::http::Request;
use web_sys::HtmlSelectElement;
use serde::Serialize;
use crate::context::UserContext;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub kermesse_id: i32,
}

#[derive(Serialize)]
struct CollaborateRequest {
    proposed_role: String,
}

#[function_component(CollaborationRequestForm)]
pub fn collaboration_request_form(props: &Props) -> Html {
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let role_ref = use_node_ref();
    let show_form = use_state(|| false);
    let kermesse_id = props.kermesse_id;

    let on_request = {
        let role_ref = role_ref.clone();
        let token = user_ctx.user.as_ref().map(|u| u.token.clone());
        let show_form = show_form.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let proposed_role = role_ref.cast::<HtmlSelectElement>().unwrap().value();
            let token = token.clone();
            let show_form = show_form.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(token) = token {
                    let body = serde_json::to_string(&CollaborateRequest { proposed_role }).unwrap();
                    let resp = Request::post(&format!("http://127.0.0.1:8080/kermesses/{}/join", kermesse_id))
                        .header("Authorization", &format!("Bearer {}", token))
                        .header("Content-Type", "application/json")
                        .body(body)
                        .send()
                        .await;

                    if let Ok(resp) = resp {
                        if resp.ok() {
                            gloo_dialogs::alert("¡Solicitud enviada! El organizador la revisará.");
                            show_form.set(false);
                        } else {
                            gloo_dialogs::alert("Error al enviar solicitud. Quizás ya solicitaste antes.");
                        }
                    }
                } else {
                    gloo_dialogs::alert("Debes iniciar sesión para colaborar.");
                }
            });
        })
    };

    html! {
        <div class="bg-gradient-to-r from-blue-50 to-indigo-50 rounded-xl shadow-md p-6">
            <h3 class="text-xl font-bold mb-4 text-gray-800">{ "¿Quieres Colaborar?" }</h3>
            
            if !*show_form {
                <div>
                    <p class="text-gray-600 mb-4">{ "Únete al equipo y ayuda a que este evento sea un éxito. Elige cómo quieres colaborar y envía tu solicitud." }</p>
                    <button 
                        onclick={Callback::from(move |_| show_form.set(true))}
                        class="w-full bg-primary text-white font-bold py-3 px-4 rounded-xl shadow hover:bg-red-600 transition"
                    >
                        { "Solicitar Ser Colaborador" }
                    </button>
                </div>
            } else {
                <form onsubmit={on_request} class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">{ "¿Cómo quieres ayudar?" }</label>
                        <select ref={role_ref} required=true class="w-full border-gray-300 rounded-lg shadow-sm p-2 border">
                            <option value="">{ "Selecciona una opción..." }</option>
                            <option value="KITCHEN">{ "🍳 Ayudar en la Cocina" }</option>
                            <option value="SELLER">{ "💰 Vender Tickets (Platos)" }</option>
                            <option value="DELIVERY">{ "🚚 Repartir Pedidos" }</option>
                            <option value="INGREDIENT_GETTER">{ "🛒 Conseguir Ingredientes" }</option>
                        </select>
                    </div>
                    <div class="flex gap-2">
                        <button type="submit" class="flex-1 bg-green-600 text-white font-bold py-2 px-4 rounded-lg shadow hover:bg-green-700 transition">
                            { "Enviar Solicitud" }
                        </button>
                        <button 
                            type="button"
                            onclick={Callback::from(move |_| show_form.set(false))}
                            class="flex-1 bg-gray-300 text-gray-700 font-bold py-2 px-4 rounded-lg shadow hover:bg-gray-400 transition"
                        >
                            { "Cancelar" }
                        </button>
                    </div>
                </form>
            }
        </div>
    }
}
