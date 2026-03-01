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
    pub financial_goal: Option<String>,
    pub qr_code_url: Option<String>,
    pub department: Option<String>,
    pub city: Option<String>,
}

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let kermesses = use_state(|| Vec::<Kermesse>::new());
    let selected_dept = use_state(|| "Todos".to_string());
    
    {
        let kermesses = kermesses.clone();
        let selected_dept = selected_dept.clone();
        
        let dept_val = (*selected_dept).clone();
        use_effect_with(dept_val, move |dept_ref| {
            let dept = dept_ref.clone();
            let kermesses = kermesses.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let url = if dept == "Todos" {
                    "http://127.0.0.1:8080/kermesses".to_string()
                } else {
                    format!("http://127.0.0.1:8080/kermesses?department={}", dept)
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

    let nav_for_detail = navigator.clone();
    let on_click_detail = move |id: i32| {
        let navigator = nav_for_detail.clone();
        Callback::from(move |_| navigator.push(&Route::KermesseDetail { id }))
    };

    // Limit to 6 on home
    let kermesses_to_show: Vec<_> = kermesses.iter().take(6).collect();
    let has_more = kermesses.len() > 6;

    html! {
        <div class="bg-gray-50 text-gray-800 font-sans">
            // Hero Section
            <header class="relative bg-gradient-to-br from-orange-50 to-orange-100 overflow-hidden">
                <div class="absolute inset-0 z-0 opacity-10 bg-[url('/pattern.svg')]"></div>
                <div class="container mx-auto px-6 py-24 relative z-10 text-center">
                    <span class="inline-block py-1 px-3 rounded-full bg-orange-200 text-orange-800 text-sm font-bold mb-4 animate-pulse">
                        { "🎉 ¡La app solidaria de Bolivia!" }
                    </span>
                    <h1 class="text-6xl md:text-7xl font-display font-extrabold mb-6 leading-tight text-gray-900 tracking-tight">
                        { "Ayudemos " } 
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-orange-600 to-red-600">
                            { "Juntos" }
                        </span>
                    </h1>
                    <p class="text-xl md:text-2xl text-gray-600 mb-12 max-w-3xl mx-auto font-light leading-relaxed">
                        { "Disfruta de la mejor gastronomía mientras colaboras con causas nobles en Kermi." }
                    </p>
                    
                    // Location Filter - Floating Pill
                    <div class="max-w-2xl mx-auto bg-white p-2 rounded-full shadow-2xl shadow-orange-200/50 border border-gray-100 flex items-center gap-2 transform transition hover:scale-[1.01]">
                        <div class="pl-6 text-orange-500 text-xl">
                            <span>{"📍"}</span>
                        </div>
                        <div class="flex-grow flex flex-col items-start pl-2">
                             <label class="text-xs text-gray-400 font-bold uppercase tracking-wider">{ "Departamento" }</label>
                             <select 
                                onchange={on_change_dept} 
                                class="w-full bg-transparent border-none focus:ring-0 text-gray-800 font-bold text-lg cursor-pointer p-0"
                            >
                                <option value="Todos" selected={*selected_dept == "Todos"}>{ "Todo Bolivia" }</option>
                                <option value="La Paz" selected={*selected_dept == "La Paz"}>{ "La Paz" }</option>
                                <option value="Santa Cruz" selected={*selected_dept == "Santa Cruz"}>{ "Santa Cruz" }</option>
                                <option value="Cochabamba" selected={*selected_dept == "Cochabamba"}>{ "Cochabamba" }</option>
                                <option value="Oruro" selected={*selected_dept == "Oruro"}>{ "Oruro" }</option>
                                <option value="Potosí" selected={*selected_dept == "Potosí"}>{ "Potosí" }</option>
                                <option value="Chuquisaca" selected={*selected_dept == "Chuquisaca"}>{ "Chuquisaca" }</option>
                                <option value="Tarija" selected={*selected_dept == "Tarija"}>{ "Tarija" }</option>
                                <option value="Beni" selected={*selected_dept == "Beni"}>{ "Beni" }</option>
                                <option value="Pando" selected={*selected_dept == "Pando"}>{ "Pando" }</option>
                            </select>
                        </div>
                        <Link<Route> to={Route::AllKermesses} classes="bg-gradient-to-r from-orange-500 to-red-600 text-white font-bold text-lg px-8 py-4 rounded-full shadow-lg hover:shadow-orange-500/30 hover:to-red-700 transition duration-300 shrink-0">
                            { "Buscar" }
                        </Link<Route>>
                    </div>
                </div>
            </header>

            // Content
            <main class="container mx-auto px-6 py-12 max-w-7xl">
                <div class="flex items-center justify-between mb-8">
                    <h2 class="text-3xl font-bold text-gray-800 border-l-4 border-orange-500 pl-4">
                        { "Eventos Activos" }
                    </h2>
                    if has_more {
                        <Link<Route> to={Route::AllKermesses} classes="text-orange-500 font-bold hover:text-orange-700 transition">
                            { "Ver todos →" }
                        </Link<Route>>
                    }
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {
                        if kermesses.is_empty() {
                            html! {
                                <div class="col-span-full text-center py-20 text-gray-500">
                                    <p class="text-4xl mb-4">{"🎪"}</p>
                                    <p class="text-xl font-medium">{ "No hay eventos activos por ahora." }</p>
                                    <p class="text-gray-400">{ "¡Vuelve pronto o crea el tuyo!" }</p>
                                </div>
                            }
                        } else {
                            kermesses_to_show.iter().map(|k| {
                                let id = k.id;
                                html! {
                                    <div class="bg-white rounded-3xl shadow-lg hover:shadow-2xl transition-all duration-300 transform hover:-translate-y-2 overflow-hidden border border-gray-100 flex flex-col h-full group">
                                        // Image Area
                                        <div class="h-52 bg-gradient-to-br from-orange-100 to-red-50 flex items-center justify-center relative overflow-hidden">
                                            if let Some(img_url) = &k.beneficiary_image_url {
                                                <img src={img_url.clone()} alt={k.name.clone()} class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" />
                                            } else {
                                                <span class="text-6xl group-hover:scale-110 transition-transform duration-500">{"🍲"}</span>
                                            }
                                            <div class="absolute top-4 right-4 bg-white/90 backdrop-blur px-3 py-1 rounded-full shadow-sm text-sm font-bold text-gray-700 z-10">
                                                { "📅 " }{ &k.event_date }
                                            </div>
                                            <div class="absolute inset-0 bg-black/30 opacity-0 group-hover:opacity-100 transition-opacity duration-300 z-10 flex items-center justify-center">
                                                <button onclick={on_click_detail(id)} class="bg-white text-orange-600 font-bold py-2 px-6 rounded-full shadow-lg transform scale-90 group-hover:scale-100 transition-transform">
                                                    { "Ver Detalles" }
                                                </button>
                                            </div>
                                        </div>
                                        
                                        <div class="p-6 flex-grow flex flex-col">
                                            <div class="flex justify-between items-start mb-2">
                                                 <span class={format!("text-xs font-bold px-3 py-1 rounded-full uppercase tracking-wide {}", 
                                                    if k.status == "ACTIVE" { "bg-green-100 text-green-700" } else { "bg-gray-100 text-gray-600" }
                                                 )}>
                                                    { if k.status == "ACTIVE" { "Activo" } else { &k.status } }
                                                </span>
                                            </div>
                                            
                                            <h3 class="text-xl font-display font-bold mb-2 text-gray-900 leading-tight group-hover:text-orange-600 transition-colors">
                                                { &k.name }
                                            </h3>
                                            
                                            if let (Some(dept), Some(city)) = (&k.department, &k.city) {
                                                <div class="flex items-center gap-1.5 mb-3 text-sm text-gray-500 font-medium">
                                                    <span class="text-orange-500">{"📍"}</span>
                                                    <span>{ format!("{}, {}", city, dept) }</span>
                                                </div>
                                            }
                                            
                                            <p class="text-gray-600 mb-4 line-clamp-2 leading-relaxed text-sm flex-grow">
                                                { &k.description }
                                            </p>
                                            
                                            <div class="pt-4 border-t border-gray-100 mt-auto">
                                                <button onclick={on_click_detail(id)} class="w-full bg-gray-50 text-gray-800 font-bold py-3 px-4 rounded-xl hover:bg-orange-50 hover:text-orange-600 transition-colors duration-300 flex items-center justify-center gap-2">
                                                    { "Colaborar →" }
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    }
                </div>

                // "Ver más" button
                if has_more {
                    <div class="text-center mt-12">
                        <Link<Route> to={Route::AllKermesses} classes="inline-block bg-gradient-to-r from-orange-500 to-red-600 text-white font-bold text-lg px-10 py-4 rounded-full shadow-lg hover:shadow-orange-500/30 hover:to-red-700 transition transform hover:-translate-y-1">
                            { "Ver más eventos →" }
                        </Link<Route>>
                    </div>
                }
            </main>
        </div>
    }
}
