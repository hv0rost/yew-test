use std::{rc::Rc, fmt::format};

use::yew::prelude::*;
use::yewdux::prelude::*;

use crate::store::auth_store::{AuthStore, self};

pub enum Msg {
    Store(Rc<AuthStore>),
}
pub struct DisplayAuth{
    dispatch : Dispatch<AuthStore>
}

impl Component for DisplayAuth {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<AuthStore>::subscribe(ctx.link().callback(Msg::Store));
        Self { dispatch }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Store(_) => true,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let auth_store = self.dispatch.get();
        let username = format!("Username: {}", auth_store.username.as_deref().unwrap_or_default());
        let password = format!("Password: {}", auth_store.password.as_deref().unwrap_or_default());
        let is_auth = format!("Is authentificated: {:?}", auth_store.is_authentificated);
        html!(
            <div>
                <h2>{"Auth Store"} </h2>
                <div>{username}</div>
                <div>{password}</div>
                <div>{is_auth}</div>
            </div>
        )
    }
}