use std::ops::Deref;
use yew::prelude::*;
use crate::User;
use crate::{components::atoms::{text_input::TextInput, button::CustomButton}};

#[derive(Default, Clone)]
pub struct Data {
    pub username : String,
    pub pos : String,
}

#[derive(Properties, PartialEq)]
pub struct Props{
    pub onsubmit : Callback<Data>
}

#[function_component(CustomForm)]
pub fn custom_form(props : &Props) -> Html {
    let state = use_state(|| Data::default());
    let user_context = use_context::<User>();

    let clonned_state = state.clone();
    let username_changed  = Callback::from(move |username|{
        clonned_state.set(
            Data{
                username,
                ..clonned_state.deref().clone()
            }
        );
    }); 

    let clonned_state = state.clone();
       let pos_changed  = Callback::from(move |pos|{
        clonned_state.set(
            Data{
                pos,
                ..clonned_state.deref().clone()
            }
        );
    });

    let clonned_state = state.clone();
    let form_onsubmit = props.onsubmit.clone();
    let onsubmit = Callback::from(move |e : FocusEvent|{
        e.prevent_default();
        let data = clonned_state.deref().clone();
        form_onsubmit.emit(data);
    });

/*     let button_clicked = Callback::from(move|_| {
        let mut data = clonned_state.deref().clone();
        data.counter += 1;
        clonned_state.set(data)

/*         clonned_state.set(
            Data{
                counter : clonned_state.deref().clone().counter + 1,
                ..clonned_state.deref().clone()
            }
        ); */
    }); */



    html!(
        <form onsubmit={onsubmit}>
            <TextInput label = "Name" name="username" handle_onchange={username_changed}/>
            <TextInput label = "Pos in DOTA TWO" name="favorite pos in dota" handle_onchange={pos_changed}/>
            <CustomButton label="push"/>
            <p>{"Name: "}{user_context.clone().unwrap_or_default().username}</p>
            <p>{"PoS in DOTA DVA: "}{user_context.unwrap_or_default().pos}</p>
        </form>
    )
}