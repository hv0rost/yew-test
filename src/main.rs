mod components;
mod store;
mod router;

use std::ops::Deref;
use yew::prelude::*;
use yew::ContextProvider;
use components::atoms::struct_hello::StructHello;
use components::{atoms::title::{Title, Color}, molecules::form::{Data, CustomForm}};
use::yew_router::prelude::*;
use router::{Route, switch};
use components::molecules::yewdux_counter::DisplayCount;
use components::molecules::increment_btn::IncrementCount;


#[derive(Clone, PartialEq, Default)]
pub struct User {
    pub username : String,
    pub pos : String,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state( User::default);
    let first_load = use_state(|| true);


    use_effect(move || {
        //this code will run on first render and every rerender of tis component
        if *first_load {
            //this code will run on first render
            first_load.set(false);
        }

        || {}
    });

    let custom_form_submit = {
        let state = state.clone();
        Callback::from(move |data : Data|{
            let mut user = state.deref().clone();
            user.username = data.username;
            user.pos = data.pos;
            state.set(user)
        })
    };

    html! {
		<ContextProvider<User> context = {state.deref().clone()}>
            <Title title = "This is a TITLE" color = {Color::Normal}/>
            <CustomForm onsubmit = {custom_form_submit}/>
            <StructHello message = {"?!?!!?!?!?"}/>
            <BrowserRouter>
                <Switch<Route> render = {Switch::render(switch)} />
            </BrowserRouter>
            <IncrementCount />
            <DisplayCount />
		</ContextProvider<User>>
    }
}

fn main() {
    yew::start_app::<App>();
}

