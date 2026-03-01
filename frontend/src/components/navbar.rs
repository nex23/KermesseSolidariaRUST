use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::context::UserContext;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let user_ctx = use_context::<UserContext>().expect("No UserContext found");
    let navigator = use_navigator().unwrap();

    let nav_login = navigator.clone();
    let nav_dashboard = navigator.clone();
    let nav_collab = navigator.clone();
    let nav_home = navigator.clone();

    let user_ctx_logout = user_ctx.clone();
    let on_logout = Callback::from(move |_| user_ctx_logout.set_user.emit(None));

    html! {
        <nav class="bg-white shadow-sm sticky top-0 z-50 border-b border-gray-100">
            <div class="container mx-auto px-6 py-4 flex justify-between items-center max-w-7xl">
                // Logo
                <div class="flex items-center space-x-2 cursor-pointer" onclick={Callback::from(move |_| nav_home.push(&Route::Home))}>
                    <span class="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-orange-500 to-red-600">
                        { "Kermi" }
                    </span>
                </div>

                // Navigation links
                <div class="hidden md:flex items-center space-x-6">
                    <Link<Route> to={Route::Home} classes="text-gray-600 hover:text-orange-500 transition font-medium">
                        { "Inicio" }
                    </Link<Route>>
                    <Link<Route> to={Route::AllKermesses} classes="text-gray-600 hover:text-orange-500 transition font-medium">
                        { "Ver Eventos" }
                    </Link<Route>>
                    if user_ctx.user.is_some() {
                        <Link<Route> to={Route::Dashboard} classes="text-gray-600 hover:text-orange-500 transition font-medium">
                            { "Mi Panel" }
                        </Link<Route>>
                        <Link<Route> to={Route::CollaboratorDashboard} classes="text-gray-600 hover:text-orange-500 transition font-medium">
                            { "Colaborador" }
                        </Link<Route>>
                    }
                </div>

                // Right side: login/user
                <div class="flex items-center gap-3">
                    if let Some(user) = &user_ctx.user {
                        <span class="text-gray-600 font-medium text-sm hidden sm:block">{ format!("Hola, {}", user.username) }</span>
                        <button
                            onclick={on_logout}
                            class="text-sm text-red-500 hover:text-red-700 font-bold transition px-3 py-1 rounded-lg hover:bg-red-50"
                        >
                            { "Salir" }
                        </button>
                    } else {
                        <button
                            onclick={Callback::from(move |_| nav_login.push(&Route::Login))}
                            class="bg-gradient-to-r from-orange-500 to-red-600 text-white font-bold text-sm px-5 py-2 rounded-full shadow hover:shadow-orange-500/30 transition transform hover:-translate-y-0.5"
                        >
                            { "Iniciar Sesión" }
                        </button>
                    }
                </div>
            </div>
        </nav>
    }
}
