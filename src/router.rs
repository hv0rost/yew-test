use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::home::Home;
use crate::components::pages::hello::Hello;
use crate::components::molecules::auth_form::AuthForm;
use crate::components::molecules::display_auth::DisplayAuth;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route  {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello,   
    #[at("/auth")]
    Auth,
}

pub fn switch(route : &Route) -> Html {
    match route {
        Route::Home => html!(<Home />),
        Route::Hello => html!(<Hello />),
        Route::Auth => html!(
            <div>
            <AuthForm />
            <DisplayAuth />
            </div>
        ),
    }
}