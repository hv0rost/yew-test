use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::molecules::struct_counter::StructCounter;
use crate::router::Route;

#[function_component(Hello)]
pub fn hello() -> html {
    let history = use_history().unwrap();
    let onclick = Callback::from(move|_| {
        history.push(Route::Home)
    });

    html! (
        <div>
            <h1>{"Hello here"}</h1>
            <StructCounter />
            <button onclick = {onclick}>{"No hello anymore"}</button>
        </div>
        
    )
}