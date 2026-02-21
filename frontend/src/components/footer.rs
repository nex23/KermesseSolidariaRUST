use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    let year = "2026"; // Or dynamically get it

    html! {
        <footer class="bg-gray-900 text-white mt-auto py-12 border-t-4 border-orange-500">
            <div class="container mx-auto px-4 sm:px-6 lg:px-8 max-w-7xl">
                <div class="grid grid-cols-1 md:grid-cols-4 gap-8 mb-8">
                    // Logo and Description
                    <div class="col-span-1 md:col-span-1">
                        <Link<Route> to={Route::Home} classes="text-2xl font-display font-bold text-white mb-4 block">
                            <span class="text-orange-500">{ "Kermi" }</span>
                        </Link<Route>>
                        <p class="text-gray-400 text-sm leading-relaxed mb-4">
                            { "Conectando causas nobles con corazones solidarios a través de la mejor gastronomía boliviana. ¡Ayudemos juntos!" }
                        </p>
                    </div>

                    // Main Menu Links
                    <div>
                        <h4 class="text-lg font-bold mb-4 tracking-wider text-gray-200">{ "Menú Principal" }</h4>
                        <ul class="space-y-2 text-sm text-gray-400">
                            <li><Link<Route> to={Route::Home} classes="hover:text-orange-500 transition-colors">{ "Inicio" }</Link<Route>></li>
                            <li><Link<Route> to={Route::Dashboard} classes="hover:text-orange-500 transition-colors">{ "Mi Panel Organizador" }</Link<Route>></li>
                            <li><Link<Route> to={Route::CollaboratorDashboard} classes="hover:text-orange-500 transition-colors">{ "Panel Colaborador" }</Link<Route>></li>
                            <li><Link<Route> to={Route::Login} classes="hover:text-orange-500 transition-colors">{ "Iniciar Sesión" }</Link<Route>></li>
                        </ul>
                    </div>

                    // Help & Info
                    <div>
                        <h4 class="text-lg font-bold mb-4 tracking-wider text-gray-200">{ "Ayuda e Información" }</h4>
                        <ul class="space-y-2 text-sm text-gray-400">
                            <li><a href="#" class="hover:text-orange-500 transition-colors">{ "Acerca de nosotros" }</a></li>
                            <li><a href="#" class="hover:text-orange-500 transition-colors">{ "Ayuda (Cómo crear una kermesse)" }</a></li>
                            <li><a href="#" class="hover:text-orange-500 transition-colors">{ "Colabora (Apoya la App)" }</a></li>
                        </ul>
                    </div>

                    // Legal
                    <div>
                        <h4 class="text-lg font-bold mb-4 tracking-wider text-gray-200">{ "Legal" }</h4>
                        <ul class="space-y-2 text-sm text-gray-400">
                            <li><a href="#" class="hover:text-orange-500 transition-colors">{ "Políticas de Privacidad" }</a></li>
                            <li><a href="#" class="hover:text-orange-500 transition-colors">{ "Protección de Datos" }</a></li>
                            <li><a href="#" class="hover:text-orange-500 transition-colors">{ "Términos y Condiciones" }</a></li>
                        </ul>
                    </div>
                </div>

                <div class="border-t border-gray-800 pt-8 flex flex-col md:flex-row justify-between items-center text-sm text-gray-500">
                    <p>{ format!("© {} Kermi. Todos los derechos reservados.", year) }</p>
                    <p class="mt-4 md:mt-0 font-medium">
                        { "Desarrollado por: " } <span class="text-orange-500 font-bold">{ "HugoNex" }</span>
                    </p>
                </div>
            </div>
        </footer>
    }
}
