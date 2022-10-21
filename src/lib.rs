use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

mod components;

use components::{atoms::title::{Title, Color}, molecules::form::Data};
use crate::components::molecules::form::CustomForm;
#[derive(Serialize, Deserialize)]
struct Model{
    value : i32,
	//password : String
}

#[function_component(App)]
pub fn app() -> Html {
	let custom_form_submit = Callback::from(|data : Data|{
		log!("username is", data.username);
		log!("pos is", data.pos);
	});

    html! {
		<>
		<Title title = "This is a TITLE" color = {Color::Normal}/>
		<CustomForm onsubmit = {custom_form_submit}/>
		</>
    }
}