use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;
use crate::router::Route;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Kermesse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub event_date: String,
    pub status: String,
    pub organizer_id: i32,
    pub beneficiary_name: String,
    pub beneficiary_reason: String,
    pub beneficiary_image_url: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub financial_goal: Option<String>, // Backend sends Decimal as String
    pub qr_code_url: Option<String>,
    pub department: Option<String>,
    pub city: Option<String>,
}

use crate::context::UserContext;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let kermesses = use_state(|| vec![]);
    let selected_dept = use_state(|| "Todos".to_string());
    
    {
        let kermesses = kermesses.clone();
        let selected_dept = selected_dept.clone();
        use_effect_with(selected_dept.clone(), move |dept_handle| {
            let dept = dept_handle.clone();
            let kermesses = kermesses.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let url = if *dept == "Todos" {
                    "http://127.0.0.1:8080/kermesses".to_string()
                } else {
                    format!("http://127.0.0.1:8080/kermesses?department={}", *dept)
                };

                if let Ok(resp) = Request::get(&url).send().await {
                    if let Ok(fetched) = resp.json().await {
                        kermesses.set(fetched);
                    }
                }
            });
            || ()
        });
    }

    let on_change_dept = {
        let selected_dept = selected_dept.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            selected_dept.set(input.value());
        })
    };

    let on_detect_location = {
        let selected_dept = selected_dept.clone();
        Callback::from(move |_| {
            // Simplified Geolocator: Just asking browser, but since we don't have reverse geocoding API key, 
            // we will simulate or just focus on manual selection for now as per plan.
            // But let's add a placeholder alerting the user.
            gloo_dialogs::alert("Geolocalización: Selecciona tu departamento manualmente por ahora (API Key requerida para reverso).");
        })
    };

    let nav_for_detail = navigator.clone();
    let on_click_detail = move |id: i32| {
        let navigator = nav_for_detail.clone();
        Callback::from(move |_| navigator.push(&Route::KermesseDetail { id }))
    };
    
    let on_click_login = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Login))
    };

    let on_click_logout = {
        let user_ctx = user_ctx.clone();
        Callback::from(move |_| user_ctx.set_user.emit(None))
    };

    let nav_for_home = navigator.clone();
    let on_click_home = Callback::from(move |_| nav_for_home.push(&Route::Home));

    let nav_for_collab = navigator.clone();
    let on_click_collab = Callback::from(move |_| nav_for_collab.push(&Route::CollaboratorDashboard));

    html! {
        <div class="min-h-screen bg-gray-50 text-gray-800 font-sans">
            // Navbar
            <nav class="bg-white shadow-sm sticky top-0 z-50">
                <div class="container mx-auto px-6 py-4 flex justify-between items-center">
                    <div class="flex items-center space-x-2 cursor-pointer" onclick={on_click_home}>
                        <span class="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-primary to-secondary">
                            { "Kermesse" }
                        </span>
                        <span class="text-2xl font-light text-gray-500">{ "Solidaria" }</span>
                    </div>
                    <div class="space-x-4 flex items-center">
                        <button class="text-gray-600 hover:text-primary transition font-medium">{ "Inicio" }</button>
                        if let Some(user) = &user_ctx.user {
                             <button onclick={Callback::from(move |_| navigator.push(&Route::Dashboard))} class="text-gray-600 hover:text-primary transition font-medium mr-4">{ "Mi Panel" }</button>
                             <button onclick={on_click_collab} class="text-gray-600 hover:text-primary transition font-medium mr-4">{ "Panel Colaborador" }</button>
                             <span class="text-gray-600 font-medium mr-4">{ format!("Hola, {}", user.username) }</span>
                             <button onclick={on_click_logout} class="text-red-500 hover:text-red-700 font-medium transition">
                                { "Cerrar Sesión" }
                             </button>
                        } else {
                            <button onclick={on_click_login} class="bg-primary text-white px-5 py-2 rounded-full shadow-lg hover:bg-red-500 hover:shadow-xl transition transform hover:-translate-y-0.5">
                                { "Iniciar Sesión" }
                            </button>
                        }
                    </div>
                </div>
            </nav>

            // Hero Section
            <header class="relative bg-white overflow-hidden">
                <div class="absolute inset-0 z-0 opacity-10 bg-pattern"></div>
                <div class="container mx-auto px-6 py-12 relative z-10 text-center">
                    <h1 class="text-5xl md:text-6xl font-extrabold mb-6 leading-tight text-dark">
                        { "Ayudemos " } <span class="text-primary">{ "Juntos" }</span>
                    </h1>
                    <p class="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
                        { "Encuentra kermesses solidarias cerca de ti." }
                    </p>
                    
                    // Location Filter
                    <div class="max-w-md mx-auto bg-white p-2 rounded-full shadow-xl border border-gray-100 flex items-center gap-2">
                        <div class="pl-4 text-gray-400">
                            <span>{"📍"}</span>
                        </div>
                        <select 
                            onchange={on_change_dept} 
                            class="flex-grow bg-transparent border-none focus:ring-0 text-gray-700 font-medium cursor-pointer"
                        >
                            <option value="Todos">{ "Todo Bolivia" }</option>
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
                        <button onclick={on_detect_location} class="bg-gray-100 text-gray-600 p-2 rounded-full hover:bg-gray-200 transition" title="Usar mi ubicación actual">
                            {"🎯"}
                        </button>
                    </div>
                </div>
            </header>

            // Content
            <main class="container mx-auto px-6 py-12">
                <div class="flex items-center justify-between mb-8">
                    <h2 class="text-3xl font-bold text-gray-800 border-l-4 border-primary pl-4">
                        { format!("Eventos en {}", *selected_dept) }
                    </h2>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {
                        if kermesses.is_empty() {
                            html! {
                                <div class="col-span-full text-center py-20 text-gray-500">
                                    <p class="text-xl">{ "Cargando eventos o no hay kermesses activas..." }</p>
                                </div>
                            }
                        } else {
                            kermesses.iter().map(|k: &Kermesse| {
                                let id = k.id;
                                html! {
                                    <div class="bg-white rounded-2xl shadow-md hover:shadow-2xl transition-all duration-300 transform hover:-translate-y-2 overflow-hidden border border-gray-100 flex flex-col h-full">
                                        <div class="h-48 bg-gradient-to-br from-gray-200 to-gray-300 flex items-center justify-center">
                                            <span class="text-4xl">{"🍲"}</span>
                                        </div>
                                        <div class="p-6 flex-grow flex flex-col">
                                            <div class="flex justify-between items-start mb-2">
                                                 <span class="bg-green-100 text-green-800 text-xs font-semibold px-2.5 py-0.5 rounded dark:bg-green-200 dark:text-green-900">
                                                    { &k.status }
                                                </span>
                                                <span class="text-sm text-gray-500 font-medium">{ &k.event_date }</span>
                                            </div>
                                            <h3 class="text-2xl font-bold mb-1 text-gray-800">{ &k.name }</h3>
                                            
                                            if let (Some(dept), Some(city)) = (&k.department, &k.city) {
                                                <div class="flex items-center gap-1 mb-2 text-xs text-gray-500 font-semibold uppercase tracking-wide">
                                                    <span>{"📍"}</span>
                                                    <span>{ format!("{}, {}", city, dept) }</span>
                                                </div>
                                            }
                                            
                                            <p class="text-gray-600 mb-4 line-clamp-3">{ &k.description }</p>
                                            
                                            <div class="mt-auto pt-4 border-t border-gray-100">
                                                <button onclick={on_click_detail(id)} class="w-full bg-white text-primary border border-primary font-bold py-2 px-4 rounded-xl hover:bg-primary hover:text-white transition-colors duration-300">
                                                    { "Ver Menú y Colaborar" }
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    }
                </div>
            </main>

            // Footer
             <footer class="bg-dark text-white py-12">
                <div class="container mx-auto px-6 text-center">
                    <p class="mb-4 text-gray-400">{ "Hecho con ❤️ en Bolivia para ayudar." }</p>
                    <p class="text-sm text-gray-600">{ "© 2026 Kermesse Solidaria. Todos los derechos reservados." }</p>
                </div>
            </footer>
        </div>
    }
}
