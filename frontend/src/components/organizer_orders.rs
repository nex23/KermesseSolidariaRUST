use yew::prelude::*;
use reqwasm::http::Request;
use crate::context::UserContext;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct SaleResponse {
    pub id: i32,
    pub customer_name: String,
    pub total_amount: f64,
    pub status: String,
    // Add more fields if needed for the dashboard details
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub kermesse_id: i32,
}

#[derive(Clone, PartialEq)]
pub enum OrderFilter {
    All,
    Pending,
    Paid,
    Delivered,
}

#[function_component(OrganizerOrders)]
pub fn organizer_orders(props: &Props) -> Html {
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let orders = use_state(|| Vec::<SaleResponse>::new());
    let filter = use_state(|| OrderFilter::All);
    let kermesse_id = props.kermesse_id;
    let loading = use_state(|| true);
    
    // Refresh Trigger
    let refresh_trigger = use_state(|| 0);

    {
        let orders = orders.clone();
        let loading = loading.clone();
        let user_ctx = user_ctx.clone();
        let refresh_trigger = refresh_trigger.clone();
        use_effect_with(refresh_trigger, move |_| {
            if let Some(user) = &user_ctx.user {
                let token = user.token.clone();
                let orders = orders.clone();
                let loading = loading.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("http://127.0.0.1:8080/kermesses/{}/sales", kermesse_id);
                    let resp = Request::get(&url)
                        .header("Authorization", &format!("Bearer {}", token))
                        .send()
                        .await;

                    if let Ok(resp) = resp {
                         if let Ok(data) = resp.json::<Vec<SaleResponse>>().await {
                             orders.set(data);
                         }
                    }
                    loading.set(false);
                });
            }
            || ()
        });
    }

    let update_status = {
        let user_ctx = user_ctx.clone();
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |(sale_id, new_status): (i32, String)| {
             let token = user_ctx.user.as_ref().unwrap().token.clone();
             let refresh_trigger = refresh_trigger.clone();
             wasm_bindgen_futures::spawn_local(async move {
                 let url = format!("http://127.0.0.1:8080/sales/{}/status", sale_id);
                 let body = serde_json::json!({ "status": new_status });
                 let _ = Request::put(&url)
                     .header("Authorization", &format!("Bearer {}", token))
                     .header("Content-Type", "application/json")
                     .body(body.to_string())
                     .send()
                     .await;
                 
                 // Trigger refresh
                 refresh_trigger.set(*refresh_trigger + 1);
             });
        })
    };

    let filtered_orders = orders.iter().filter(|o| {
        match *filter {
            OrderFilter::All => true,
            OrderFilter::Pending => o.status == "PENDING" || o.status == "PENDING_PAYMENT",
            OrderFilter::Paid => o.status == "PAID" || o.status == "CONFIRMED",
            OrderFilter::Delivered => o.status == "DELIVERED",
        }
    }).collect::<Vec<_>>();

    let status_badge = |status: &str| -> Html {
        let (color, label) = match status {
            "PENDING" | "PENDING_PAYMENT" => ("bg-yellow-100 text-yellow-800", "Pendiente"),
            "PAID" | "CONFIRMED" => ("bg-green-100 text-green-800", "Pagado/Confirmado"),
            "DELIVERED" => ("bg-blue-100 text-blue-800", "Entregado"),
            "CANCELLED" => ("bg-red-100 text-red-800", "Cancelado"),
             _ => ("bg-gray-100 text-gray-800", status),
        };
        html! { <span class={format!("px-2 py-1 rounded-full text-xs font-bold {}", color)}>{ label }</span> }
    };

    html! {
        <div class="space-y-4">
            <div class="flex flex-col sm:flex-row justify-between items-center gap-4 bg-gray-50 p-4 rounded-lg">
                <div class="flex gap-2 text-sm overflow-x-auto w-full sm:w-auto">
                    <button onclick={let f = filter.clone(); Callback::from(move |_| f.set(OrderFilter::All))} class={format!("px-3 py-1 rounded-full transition {}", if *filter == OrderFilter::All { "bg-primary text-white" } else { "bg-white text-gray-600 hover:bg-gray-100" })}>{ "Todos" }</button>
                    <button onclick={let f = filter.clone(); Callback::from(move |_| f.set(OrderFilter::Pending))} class={format!("px-3 py-1 rounded-full transition {}", if *filter == OrderFilter::Pending { "bg-primary text-white" } else { "bg-white text-gray-600 hover:bg-gray-100" })}>{ "Pendientes" }</button>
                    <button onclick={let f = filter.clone(); Callback::from(move |_| f.set(OrderFilter::Paid))} class={format!("px-3 py-1 rounded-full transition {}", if *filter == OrderFilter::Paid { "bg-primary text-white" } else { "bg-white text-gray-600 hover:bg-gray-100" })}>{ "Pagados" }</button>
                    <button onclick={let f = filter.clone(); Callback::from(move |_| f.set(OrderFilter::Delivered))} class={format!("px-3 py-1 rounded-full transition {}", if *filter == OrderFilter::Delivered { "bg-primary text-white" } else { "bg-white text-gray-600 hover:bg-gray-100" })}>{ "Entregados" }</button>
                </div>
                <button onclick={let rt = refresh_trigger.clone(); Callback::from(move |_| rt.set(*rt + 1))} class="text-secondary hover:text-teal-700 font-bold text-sm flex items-center gap-1">
                    <span>{"🔄"}</span> { "Actualizar" }
                </button>
            </div>

            if *loading {
                 <div class="text-center py-8">{ "Cargando pedidos..." }</div>
            } else if filtered_orders.is_empty() {
                 <div class="text-center py-8 text-gray-500 bg-white rounded-lg border border-dashed border-gray-300">
                    { "No hay pedidos en esta categoría." }
                 </div>
            } else {
                <div class="bg-white rounded-lg shadow overflow-hidden">
                    <table class="w-full text-left text-sm">
                        <thead class="bg-gray-50 text-gray-600 border-b">
                            <tr>
                                <th class="p-3 font-semibold">{ "Orden" }</th>
                                <th class="p-3 font-semibold">{ "Cliente" }</th>
                                <th class="p-3 font-semibold text-right">{ "Total" }</th>
                                <th class="p-3 font-semibold text-center">{ "Estado" }</th>
                                <th class="p-3 font-semibold text-center">{ "Acciones" }</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-gray-100">
                            {
                                filtered_orders.iter().map(|o| {
                                    let id = o.id;
                                    let status = o.status.clone();
                                    let update_status = update_status.clone();
                                    html! {
                                        <tr class="hover:bg-gray-50 transition">
                                            <td class="p-3 font-mono text-gray-500">{ format!("#{}", o.id) }</td>
                                            <td class="p-3 font-medium">{ &o.customer_name }</td>
                                            <td class="p-3 text-right font-bold">{ format!("Bs. {:.2}", o.total_amount) }</td>
                                            <td class="p-3 text-center">{ status_badge(&o.status) }</td>
                                            <td class="p-3 flex justify-center gap-2">
                                                if status == "PENDING" || status == "PENDING_PAYMENT" {
                                                    <button onclick={let u = update_status.clone(); Callback::from(move |_| u.emit((id, "PAID".to_string())))} class="bg-green-100 text-green-700 hover:bg-green-200 px-2 py-1 rounded text-xs font-bold" title="Marcar como Pagado">{ "✔ Pagar" }</button>
                                                }
                                                if status == "PAID" || status == "CONFIRMED" {
                                                    <button onclick={let u = update_status.clone(); Callback::from(move |_| u.emit((id, "DELIVERED".to_string())))} class="bg-blue-100 text-blue-700 hover:bg-blue-200 px-2 py-1 rounded text-xs font-bold" title="Marcar como Entregado">{ "🚀 Enviar" }</button>
                                                }
                                                if status != "CANCELLED" && status != "DELIVERED" {
                                                     <button onclick={let u = update_status.clone(); Callback::from(move |_| u.emit((id, "CANCELLED".to_string())))} class="bg-red-50 text-red-600 hover:bg-red-100 px-2 py-1 rounded text-xs" title="Cancelar Pedido">{ "✕" }</button>
                                                }
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                </div>
            }
        </div>
    }
}
