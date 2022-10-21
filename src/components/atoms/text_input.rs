use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub label : String,
    pub name: String,
    pub handle_onchange : Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props : &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |e : Event|{
        let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(value);
    });

    html!(
        <input type="text" placeholder = {props.label.clone()} name={props.name.clone()} onchange = {onchange}/>
    )
}