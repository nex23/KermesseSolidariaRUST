use yew::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_storage::{LocalStorage, Storage};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub username: String,
    pub token: String,
    pub id: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserContext {
    pub user: Option<User>,
    pub set_user: Callback<Option<User>>,
}

#[derive(Properties, PartialEq)]
pub struct UserContextProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &UserContextProviderProps) -> Html {
    let user_state = use_state(|| LocalStorage::get("user").ok());

    let set_user = {
        let user_state = user_state.clone();
        Callback::from(move |user: Option<User>| {
            if let Some(u) = &user {
                let _ = LocalStorage::set("user", u);
            } else {
                let _ = LocalStorage::delete("user");
            }
            user_state.set(user);
        })
    };

    let context = UserContext {
        user: (*user_state).clone(),
        set_user,
    };

    html! {
        <ContextProvider<UserContext> context={context}>
            { props.children.clone() }
        </ContextProvider<UserContext>>
    }
}
