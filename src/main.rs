use yew::{function_component, html, prelude::*};
use yew_router::prelude::*;

use bws::Bws;
use subw::Subw;

mod bws;
mod subw;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/bws")]
    Bws,
    #[at("/subw")]
    Subw,
}

#[function_component(App)]
fn app() -> Html {
    html!(
        <BrowserRouter>
            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
        </BrowserRouter>
    )
}

//noinspection RsTypeCheck
fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Bws => {
            html! { <Bws /> }
        }
        Route::Subw => {
            html! { <Subw /> }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}