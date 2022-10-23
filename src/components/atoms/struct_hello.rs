use stylist::{Style, style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: String,
}

pub struct StructHello{
    pub style_sheet: Style,
}

impl StructHello {
    fn style() -> Style{
        style!(
            r#"
                color : purple;
            "#
        ).unwrap()
    }
}

impl Component for StructHello{
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            style_sheet : Self::style()
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        html!(
            <div>
                <h1 class = {self.style_sheet.clone()}> {&context.props().message}</h1>
            </div>
        )
    }
}