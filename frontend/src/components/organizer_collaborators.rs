use yew::prelude::*;
use reqwasm::http::Request;
use crate::context::UserContext;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct CollaboratorRequestResponse {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub full_name: String,
    pub proposed_role: Option<String>,
    pub status: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub kermesse_id: i32,
}

#[function_component(OrganizerCollaborators)]
pub fn organizer_collaborators(props: &Props) -> Html {
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let requests = use_state(|| Vec::<CollaboratorRequestResponse>::new());
    let loading = use_state(|| true);
    let kermesse_id = props.kermesse_id;
    let refresh_trigger = use_state(|| 0);

    {
        let requests = requests.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let refresh_trigger = refresh_trigger.clone();
        use_effect_with(refresh_trigger, move |_| {
            if let Some(user) = &user_ctx.user {
                let token = user.token.clone();
                let requests = requests.clone();
                let loading = loading.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("http://127.0.0.1:8080/kermesses/{}/collaborators/requests", kermesse_id);
                    let resp = Request::get(&url)
                        .header("Authorization", &format!("Bearer {}", token))
                        .send()
                        .await;

                    if let Ok(resp) = resp {
                         if let Ok(data) = resp.json::<Vec<CollaboratorRequestResponse>>().await {
                             requests.set(data);
                         }
                    }
                    loading.set(false);
                });
            }
            || ()
        });
    }

    let manage_request = {
        let user_ctx = user_ctx.clone();
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |(req_id, approve): (i32, bool)| {
             let token = user_ctx.user.as_ref().unwrap().token.clone();
             let refresh_trigger = refresh_trigger.clone();
             wasm_bindgen_futures::spawn_local(async move {
                 let url = format!("http://127.0.0.1:8080/kermesses/{}/collaborators/{}/manage", kermesse_id, req_id);
                 let body = serde_json::json!({ 
                     "approve": approve,
                     "assigned_role": if approve { Some("COLLABORATOR") } else { None } 
                 });
                 let _ = Request::post(&url)
                     .header("Authorization", &format!("Bearer {}", token))
                     .header("Content-Type", "application/json")
                     .body(body.to_string())
                     .send()
                     .await;
                 
                 refresh_trigger.set(*refresh_trigger + 1);
             });
        })
    };

    html! {
        <div class="space-y-6">
            <h3 class="text-xl font-bold text-gray-700">{ "Solicitudes de Colaboración" }</h3>
            
            if *loading {
                 <div class="text-center py-4">{ "Cargando solicitudes..." }</div>
            } else if requests.is_empty() {
                 <div class="text-center py-8 bg-gray-50 rounded-lg border border-dashed border-gray-300 text-gray-500">
                     { "No hay solicitudes pendientes." }
                 </div>
            } else {
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {
                        requests.iter().map(|req| {
                            let id = req.id;
                            let manage = manage_request.clone();
                            html! {
                                <div class="bg-white p-4 rounded-xl shadow-md border border-gray-100 flex flex-col">
                                    <div class="flex items-center gap-3 mb-3">
                                        <div class="w-10 h-10 rounded-full bg-secondary text-white flex items-center justify-center font-bold text-lg">
                                            { req.full_name.chars().next().unwrap_or('?') }
                                        </div>
                                        <div>
                                            <h4 class="font-bold text-gray-800">{ &req.full_name }</h4>
                                            <p class="text-xs text-gray-500">{ &req.username }</p>
                                        </div>
                                    </div>
                                    <div class="mb-4 bg-gray-50 p-2 rounded text-sm text-gray-600">
                                        <span class="font-semibold block mb-1">{ "Rol propuesto:" }</span>
                                        { req.proposed_role.as_deref().unwrap_or("Sin preferencia") }
                                    </div>
                                    <div class="flex gap-2 mt-auto">
                                        <button onclick={let m = manage.clone(); Callback::from(move |_| m.emit((id, false)))} class="flex-1 bg-gray-100 text-gray-600 font-bold py-2 rounded-lg hover:bg-gray-200 transition">
                                            { "Rechazar" }
                                        </button>
                                        <button onclick={let m = manage.clone(); Callback::from(move |_| m.emit((id, true)))} class="flex-1 bg-green-500 text-white font-bold py-2 rounded-lg hover:bg-green-600 transition shadow">
                                            { "Aceptar" }
                                        </button>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        </div>
    }
}
