use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Home)]
pub fn home() -> html {
    html! (
        <div>   
            <h1>{"Home"}</h1>
        <div>

        </div>
            <Link<Route> to = { Route::Hello}> {"hello???"}</Link<Route>>
        </div>
    )
}