use yew::prelude::*;
use::stylist::yew::styled_component;
use stylist::style;

#[derive(Properties, PartialEq)]
pub struct TitleProp{
    pub title : String,
    pub color: Color
}

#[derive(PartialEq)]
pub enum Color{
    Normal,
    Ok,
    Err
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Normal => "normal".to_owned(),
            Color::Ok => "ok".to_owned(),
            Color::Err => "err".to_owned(),
        }
    }
}

#[styled_component(Title)]
pub fn title(prop : &TitleProp) -> Html{
    let style = style!(r#"
        .normal {
            color : purple;
        }
        .ok {
            color : green;
        }
        .err {
            color : red;
        }
    "#).unwrap();
    html!(
        <div class = {style}>
            <h1 class = {prop.color.to_string()}> {&prop.title} </h1>
        </div>
    )
}