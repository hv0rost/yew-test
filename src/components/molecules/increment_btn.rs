use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::counter_store::CounterStore;

pub enum Msg {
    Store(Rc<CounterStore>),
    ButtonClicked
}
pub struct IncrementCount {
    pub dispatch : Dispatch<CounterStore>
}

impl Component for IncrementCount {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<CounterStore>::subscribe(ctx.link().callback(Msg::Store));
        Self { dispatch }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Store(_) => false,
            Msg::ButtonClicked => {
                self.dispatch.reduce(|store|{
                    CounterStore{
                        count : store.count + 1,
                    }
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::ButtonClicked);
        html!(
            <div>
                <button onclick = {onclick}>{"click"}</button>
            </div>
        )
    }
}