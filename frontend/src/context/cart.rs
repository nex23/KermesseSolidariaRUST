use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct CartItem {
    pub dish_id: i32,
    pub dish_name: String,
    pub price: f64,
    pub quantity: i32,
    pub kermesse_id: i32, // Enforce single kermesse per order
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct CartState {
    pub items: Vec<CartItem>,
}

impl CartState {
    pub fn total(&self) -> f64 {
        self.items.iter().map(|i| i.price * i.quantity as f64).sum()
    }

    pub fn count(&self) -> i32 {
        self.items.iter().map(|i| i.quantity).sum()
    }
}

pub enum CartAction {
    AddItem(CartItem),
    RemoveItem(i32), // dish_id
    Clear,
}

#[derive(Clone, PartialEq)]
pub struct CartContext {
    pub state: UseStateHandle<CartState>,
    pub dispatch: Callback<CartAction>,
}

#[function_component(CartProvider)]
pub fn cart_provider(props: &html::ChildrenProps) -> Html {
    let state = use_state(|| CartState::default());

    let dispatch = {
        let state = state.clone();
        Callback::from(move |action: CartAction| {
            let mut current = (*state).clone();
            match action {
                CartAction::AddItem(item) => {
                    // Check if exists
                    if let Some(existing) = current.items.iter_mut().find(|i| i.dish_id == item.dish_id) {
                        existing.quantity += item.quantity;
                    } else {
                        // Check if same kermesse (simple rule: clear if different kermesse or forbid)
                        // For simplicity: if adding from different kermesse, clear cart first? Or just allow mixed? 
                        // Backend create_sale takes one kermesse_id. So user must clear or we auto-clear.
                        // Let's Auto-Clear if mismatch for now.
                        if !current.items.is_empty() && current.items[0].kermesse_id != item.kermesse_id {
                             current.items.clear();
                        }
                        current.items.push(item);
                    }
                },
                CartAction::RemoveItem(id) => {
                    current.items.retain(|i| i.dish_id != id);
                },
                CartAction::Clear => {
                    current.items.clear();
                }
            }
            state.set(current);
        })
    };

    let context = CartContext {
        state,
        dispatch,
    };

    html! {
        <ContextProvider<CartContext> context={context}>
            { props.children.clone() }
        </ContextProvider<CartContext>>
    }
}
