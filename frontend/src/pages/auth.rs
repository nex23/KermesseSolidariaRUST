use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use crate::router::Route;
use crate::context::{UserContext, User};

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    token: String,
    username: String,
    id: i32,
}

#[function_component(Login)]
pub fn login() -> Html {
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let navigator = use_navigator().unwrap();
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");

    let onsubmit = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let request = LoginRequest { username, password };
                let body = serde_json::to_string(&request).unwrap();
                let resp = Request::post("http://127.0.0.1:8080/auth/login")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await;

                if let Ok(resp) = resp {
                    if resp.ok() {
                        if let Ok(auth_resp) = resp.json::<AuthResponse>().await {
                            user_ctx.set_user.emit(Some(User {
                                username: auth_resp.username,
                                token: auth_resp.token,
                                id: auth_resp.id,
                            }));
                            navigator.push(&Route::Home);
                        }
                    } else {
                        // Handle error (e.g. show alert)
                        gloo_dialogs::alert("Credenciales inválidas");
                    }
                } else {
                     gloo_dialogs::alert("Error de conexión");
                }
            });
        })
    };

    html! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8 bg-white p-10 rounded-xl shadow-lg">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">{ "Iniciar Sesión" }</h2>
                </div>
                <form class="mt-8 space-y-6" onsubmit={onsubmit}>
                    <div class="rounded-md shadow-sm -space-y-px">
                        <div>
                            <label for="username" class="sr-only">{ "Usuario" }</label>
                            <input ref={username_ref} id="username" name="username" type="text" required=true class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm" placeholder="Usuario" />
                        </div>
                        <div>
                            <label for="password" class="sr-only">{ "Contraseña" }</label>
                            <input ref={password_ref} id="password" name="password" type="password" required=true class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm" placeholder="Contraseña" />
                        </div>
                    </div>

                    <div>
                        <button type="submit" class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary hover:bg-red-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary">
                            { "Ingresar" }
                        </button>
                    </div>
                </form>
                <div class="text-center">
                     <button onclick={Callback::from(move |_| navigator.push(&Route::Register))} class="text-sm text-primary hover:underline">{ "¿No tienes cuenta? Regístrate" }</button>
                </div>
            </div>
        </div>
    }
}

#[derive(Serialize)]
struct RegisterRequest {
    username: String,
    password: String,
    email: String,
    full_name: String,
    phone: String,
}

#[function_component(Register)]
pub fn register() -> Html {
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let email_ref = use_node_ref();
    let fullname_ref = use_node_ref();
    let phone_ref = use_node_ref();
    let navigator = use_navigator().unwrap();
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");

    let onsubmit = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        let email_ref = email_ref.clone();
        let fullname_ref = fullname_ref.clone();
        let phone_ref = phone_ref.clone();
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let full_name = fullname_ref.cast::<HtmlInputElement>().unwrap().value();
            let phone = phone_ref.cast::<HtmlInputElement>().unwrap().value();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let request = RegisterRequest { username, password, email, full_name, phone };
                let body = serde_json::to_string(&request).unwrap();
                let resp = Request::post("http://127.0.0.1:8080/auth/register")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await;

                if let Ok(resp) = resp {
                    if resp.ok() {
                        if let Ok(auth_resp) = resp.json::<AuthResponse>().await {
                             user_ctx.set_user.emit(Some(User {
                                username: auth_resp.username,
                                token: auth_resp.token,
                                id: auth_resp.id,
                            }));
                            navigator.push(&Route::Home);
                        }
                    } else {
                        gloo_dialogs::alert("Error en registro (usuario/email duplicado?)");
                    }
                } else {
                     gloo_dialogs::alert("Error de conexión");
                }
            });
        })
    };

    html! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
             <div class="max-w-md w-full space-y-8 bg-white p-10 rounded-xl shadow-lg">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">{ "Registrarse" }</h2>
                </div>
                 <form class="mt-8 space-y-6" onsubmit={onsubmit}>
                    <div class="rounded-md shadow-sm -space-y-px">
                        <div><input ref={fullname_ref} type="text" required=true class="appearance-none rounded-t-md relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm" placeholder="Nombre Completo" /></div>
                        <div><input ref={username_ref} type="text" required=true class="appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm" placeholder="Usuario" /></div>
                        <div><input ref={email_ref} type="email" required=true class="appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm" placeholder="Email" /></div>
                        <div><input ref={phone_ref} type="text" required=true class="appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm" placeholder="Teléfono" /></div>
                        <div><input ref={password_ref} type="password" required=true class="appearance-none rounded-b-md relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm" placeholder="Contraseña" /></div>
                    </div>

                    <div>
                        <button type="submit" class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary hover:bg-red-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary">
                            { "Crear Cuenta" }
                        </button>
                    </div>
                </form>
                 <div class="text-center">
                     <button onclick={Callback::from(move |_| navigator.push(&Route::Login))} class="text-sm text-primary hover:underline">{ "¿Ya tienes cuenta? Inicia Sesión" }</button>
                </div>
             </div>
        </div>
    }
}
