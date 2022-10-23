use yew::prelude::*;

pub enum StructCounterMessage {
    ButtonClicked(u32)
}

pub struct StructCounter {
    pub count : u32,
}

impl Component for StructCounter {
    type Properties = ();
    type Message = StructCounterMessage;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            count: 0 
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StructCounterMessage::ButtonClicked(amount) => {
                self.count += amount;
            },
        }
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div>
                <button onclick = {ctx.link().callback(|_| StructCounterMessage::ButtonClicked(1))}> {"Increment by one"}</button>
                <p>{"\"Increment by one\" was clicked: "}{&self.count}{" times"}</p>
            </div>
        )
    }
}