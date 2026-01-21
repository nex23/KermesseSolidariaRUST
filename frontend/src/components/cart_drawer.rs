use yew::prelude::*;
use yew_router::prelude::*;
use crate::context::CartContext;
use crate::router::Route;

#[function_component(CartDrawer)]
pub fn cart_drawer() -> Html {
    let cart_ctx = use_context::<CartContext>().expect("No CartContext found");
    let is_open = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let total = cart_ctx.state.total();
    let count = cart_ctx.state.count();

    let toggle_cart = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let on_checkout = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Checkout))
    };

    html! {
        <>
            // Floating Button
            if count > 0 {
                <button 
                    onclick={toggle_cart.clone()}
                    class="fixed bottom-8 right-8 bg-primary text-white p-4 rounded-full shadow-2xl z-50 hover:bg-red-600 transition transform hover:scale-110 flex items-center gap-2"
                >
                    <span class="text-2xl">{"🛒"}</span>
                    <span class="font-bold">{ count }</span>
                </button>
            }

            // Drawer Overlay & Content
            if *is_open {
                <div class="fixed inset-0 z-50 flex justify-end">
                    // Backdrop
                    <div onclick={toggle_cart.clone()} class="absolute inset-0 bg-black bg-opacity-50 transition-opacity"></div>
                    
                    // Sidebar
                    <div class="relative w-full max-w-md bg-white h-full shadow-xl flex flex-col animate-slide-in-right">
                        <div class="p-4 border-b flex justify-between items-center bg-gray-50">
                            <h2 class="text-xl font-bold text-gray-800">{ "Tu Pedido" }</h2>
                            <button onclick={toggle_cart.clone()} class="text-gray-500 hover:text-gray-700">
                                { "✕" }
                            </button>
                        </div>

                        <div class="flex-1 overflow-y-auto p-4 space-y-4">
                            if cart_ctx.state.items.is_empty() {
                                <p class="text-center text-gray-500 py-8">{ "Tu carrito está vacío." }</p>
                            } else {
                                { for cart_ctx.state.items.iter().map(|item| {
                                    let item_id = item.dish_id;
                                    let cart_ctx = cart_ctx.clone();
                                    let remove = Callback::from(move |_| {
                                        cart_ctx.dispatch.emit(crate::context::CartAction::RemoveItem(item_id));
                                    });
                                    
                                    html! {
                                        <div class="flex justify-between items-center bg-gray-50 p-3 rounded-lg">
                                            <div>
                                                <p class="font-bold text-gray-800">{ &item.dish_name }</p>
                                                <p class="text-sm text-gray-600">{ format!("{} x Bs. {:.2}", item.quantity, item.price) }</p>
                                            </div>
                                            <div class="flex items-center gap-3">
                                                <span class="font-bold text-primary">{ format!("Bs. {:.2}", item.price * item.quantity as f64) }</span>
                                                <button onclick={remove} class="text-red-500 hover:text-red-700 font-bold px-2">
                                                    { "🗑" }
                                                </button>
                                            </div>
                                        </div>
                                    }
                                }) }
                            }
                        </div>

                        <div class="p-6 border-t bg-gray-50">
                            <div class="flex justify-between items-center mb-4 text-xl font-bold">
                                <span>{ "Total" }</span>
                                <span>{ format!("Bs. {:.2}", total) }</span>
                            </div>
                            <button 
                                onclick={on_checkout}
                                disabled={cart_ctx.state.items.is_empty()}
                                class="w-full bg-green-600 text-white font-bold py-3 rounded-xl hover:bg-green-700 transition disabled:opacity-50 disabled:cursor-not-allowed"
                            >
                                { "Proceder al Pago" }
                            </button>
                        </div>
                    </div>
                </div>
            }
        </>
    }
}
