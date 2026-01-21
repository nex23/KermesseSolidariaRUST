use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct SaleItemReceipt {
    pub dish_name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub subtotal: f64,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct SaleReceipt {
    pub id: i32,
    pub kermesse_name: String,
    pub event_date: String,
    pub customer_name: String,
    pub total_amount: f64,
    pub status: String,
    pub payment_method: String,
    pub delivery_method: String,
    pub created_at: String,
    pub items: Vec<SaleItemReceipt>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub receipt: SaleReceipt,
    pub on_close: Callback<()>,
}

#[function_component(Receipt)]
pub fn receipt(props: &Props) -> Html {
    let on_print = Callback::from(|_| {
        let window = web_sys::window().unwrap();
        window.print().unwrap();
    });

    html! {
        <div class="bg-white p-8 rounded-xl shadow-lg max-w-2xl mx-auto border border-gray-200" id="printable-receipt">
            <div class="text-center border-b pb-6 mb-6">
                <h2 class="text-3xl font-bold text-gray-800 mb-2">{ "Comprobante de Pedido" }</h2>
                <p class="text-gray-500">{ format!("Orden #{}", props.receipt.id) }</p>
            </div>

            <div class="grid grid-cols-2 gap-4 mb-6 text-sm">
                <div>
                    <h4 class="font-bold text-gray-600 mb-1">{ "Evento:" }</h4>
                    <p class="text-gray-900">{ &props.receipt.kermesse_name }</p>
                    <p class="text-gray-500">{ &props.receipt.event_date }</p>
                </div>
                <div class="text-right">
                    <h4 class="font-bold text-gray-600 mb-1">{ "Cliente:" }</h4>
                    <p class="text-gray-900">{ &props.receipt.customer_name }</p>
                    <p class="text-gray-500">{ &props.receipt.created_at }</p>
                </div>
            </div>

            <div class="mb-6">
                <table class="w-full text-left border-collapse">
                    <thead>
                        <tr class="bg-gray-50 text-gray-600 text-sm">
                            <th class="py-2 px-3 font-semibold">{ "Plato / Item" }</th>
                            <th class="py-2 px-3 font-semibold text-center">{ "Cant." }</th>
                            <th class="py-2 px-3 font-semibold text-right">{ "Precio" }</th>
                            <th class="py-2 px-3 font-semibold text-right">{ "Subtotal" }</th>
                        </tr>
                    </thead>
                    <tbody class="text-gray-800">
                        {
                            props.receipt.items.iter().map(|item| {
                                html! {
                                    <tr class="border-b last:border-0 hover:bg-gray-50">
                                        <td class="py-3 px-3">{ &item.dish_name }</td>
                                        <td class="py-3 px-3 text-center">{ item.quantity }</td>
                                        <td class="py-3 px-3 text-right">{ format!("Bs. {:.2}", item.unit_price) }</td>
                                        <td class="py-3 px-3 text-right font-bold">{ format!("Bs. {:.2}", item.subtotal) }</td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </tbody>
                </table>
            </div>

            <div class="flex justify-end border-t pt-4 mb-8">
                <div class="text-right">
                    <p class="text-sm text-gray-500 mb-1">{ "Total a Pagar:" }</p>
                    <p class="text-3xl font-bold text-primary">{ format!("Bs. {:.2}", props.receipt.total_amount) }</p>
                </div>
            </div>
            
            <div class="bg-yellow-50 p-4 rounded-lg mb-8 text-sm text-yellow-800 border-l-4 border-yellow-400">
                <p class="font-bold">{ match props.receipt.payment_method.as_str() {
                    "QR" => "Pago QR - PENDIENTE",
                    "CASH" => "Pago en Efectivo - Contra entrega",
                    _ => "Pendiente"
                } }</p>
                <p>{ match props.receipt.delivery_method.as_str() {
                    "DELIVERY" => "Método de entrega: Delivery (Se contactarán contigo)",
                    "PICKUP" => "Método de entrega: Recojo en el lugar",
                    "EAT_HERE" => "Método de entrega: Comer en el evento",
                    _ => ""
                } }</p>
            </div>

            <div class="flex gap-4 print:hidden">
                <button onclick={let on_close = props.on_close.clone(); move |_| on_close.emit(())} class="flex-1 bg-gray-100 text-gray-800 font-bold py-3 rounded-lg hover:bg-gray-200 transition">
                    { "Cerrar / Ir al Inicio" }
                </button>
                <button onclick={on_print} class="flex-1 bg-blue-600 text-white font-bold py-3 rounded-lg hover:bg-blue-700 transition shadow-md flex items-center justify-center gap-2">
                    <span>{"🖨️"}</span> { "Imprimir Comprobante" }
                </button>
            </div>
        </div>
    }
}
