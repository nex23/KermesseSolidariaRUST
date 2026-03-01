use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use web_sys::HtmlInputElement;
use serde::Deserialize;
use crate::router::Route;
use crate::pages::home::Kermesse;

#[function_component(AllKermesses)]
pub fn all_kermesses() -> Html {
    let navigator = use_navigator().unwrap();
    let kermesses = use_state(|| vec![]);
    let selected_dept = use_state(|| "Todos".to_string());
    let search_query = use_state(|| String::new());
    let loading = use_state(|| true);

    {
        let kermesses = kermesses.clone();
        let selected_dept = selected_dept.clone();
        let search_query = search_query.clone();
        let loading = loading.clone();
        
        let dept_val = (*selected_dept).clone();
        let query_val = (*search_query).clone();

        use_effect_with((dept_val, query_val), move |(dept, query)| {
            let dept = dept.clone();
            let query = query.clone();
            let kermesses = kermesses.clone();
            let loading = loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                let mut url = "http://127.0.0.1:8080/kermesses".to_string();
                let mut params = vec![];
                if dept != "Todos" {
                    params.push(format!("department={}", dept));
                }
                if !query.is_empty() {
                    params.push(format!("search={}", query));
                }
                if !params.is_empty() {
                    url = format!("{}?{}", url, params.join("&"));
                }
                if let Ok(resp) = Request::get(&url).send().await {
                    if let Ok(fetched) = resp.json::<Vec<Kermesse>>().await {
                        kermesses.set(fetched);
                    }
                }
                loading.set(false);
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

    let on_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            search_query.set(input.value());
        })
    };

    let nav_for_detail = navigator.clone();
    let on_click_detail = move |id: i32| {
        let navigator = nav_for_detail.clone();
        Callback::from(move |_| navigator.push(&Route::KermesseDetail { id }))
    };

    html! {
        <div class="min-h-screen bg-gray-50 font-sans">
            // Search Header
            <div class="bg-white border-b border-gray-100 shadow-sm py-8">
                <div class="container mx-auto px-6 max-w-7xl">
                    <h1 class="text-3xl font-display font-bold text-gray-900 mb-6">
                        { "🎪 Todos los Eventos Activos" }
                    </h1>
                    <div class="flex flex-col sm:flex-row gap-4">
                        // Text Search
                        <div class="flex-grow relative">
                            <span class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400 text-lg">{"🔍"}</span>
                            <input
                                type="text"
                                placeholder="Buscar por nombre o descripción..."
                                oninput={on_search_input}
                                class="w-full pl-12 pr-4 py-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-800 font-medium focus:ring-2 focus:ring-orange-500 focus:border-orange-500 outline-none transition"
                            />
                        </div>
                        // Department Select
                        <div class="sm:w-56">
                            <select
                                onchange={on_change_dept}
                                class="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-800 font-medium focus:ring-2 focus:ring-orange-500 focus:border-orange-500 outline-none transition cursor-pointer"
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
                    </div>
                    <p class="text-gray-500 text-sm mt-3">
                        { format!("{} evento(s) encontrado(s)", kermesses.len()) }
                    </p>
                </div>
            </div>

            // Grid
            <main class="container mx-auto px-6 py-10 max-w-7xl">
                if *loading {
                    <div class="flex flex-col items-center justify-center py-20">
                        <div class="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-orange-500 mb-4"></div>
                        <p class="text-gray-500 font-medium">{ "Cargando eventos..." }</p>
                    </div>
                } else if kermesses.is_empty() {
                    <div class="text-center py-20 text-gray-500">
                        <p class="text-4xl mb-4">{"😕"}</p>
                        <p class="text-xl font-medium">{ "No se encontraron eventos." }</p>
                        <p class="text-gray-400 mt-2">{ "Intenta con otra búsqueda o departamento." }</p>
                    </div>
                } else {
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {
                        kermesses.iter().map(|k: &Kermesse| {
                            let id = k.id;
                            html! {
                                <div class="bg-white rounded-3xl shadow-lg hover:shadow-2xl transition-all duration-300 transform hover:-translate-y-2 overflow-hidden border border-gray-100 flex flex-col h-full group">
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
                                        <span class="text-xs font-bold px-3 py-1 rounded-full uppercase tracking-wide bg-green-100 text-green-700 w-fit mb-2">
                                            { "Activo" }
                                        </span>
                                        <h3 class="text-xl font-display font-bold mb-2 text-gray-900 group-hover:text-orange-600 transition-colors">{ &k.name }</h3>
                                        if let (Some(dept), Some(city)) = (&k.department, &k.city) {
                                            <div class="flex items-center gap-1.5 mb-3 text-sm text-gray-500">
                                                <span class="text-orange-500">{"📍"}</span>
                                                <span>{ format!("{}, {}", city, dept) }</span>
                                            </div>
                                        }
                                        <p class="text-gray-600 mb-4 line-clamp-2 text-sm flex-grow">{ &k.description }</p>
                                        <div class="pt-4 border-t border-gray-100 mt-auto">
                                            <button onclick={on_click_detail(id)} class="w-full bg-gray-50 text-gray-800 font-bold py-3 rounded-xl hover:bg-orange-50 hover:text-orange-600 transition-colors flex items-center justify-center gap-2">
                                                { "Colaborar →" }
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    </div>
                }
            </main>
        </div>
    }
}
