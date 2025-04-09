use vizia::views::{Button, HStack, Label };
use vizia::{Application, ApplicationError};
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx|{
        cx.add_stylesheet(include_style!("src/style.css")).expect("Stylesheet was not found");

        HStack::new(cx, |cx|{
            Button::new(cx, |cx| {
                Label::new(cx, "+").class("btnLabel")
            }).corner_radius(Pixels(20.0)).class("button");
            Label::new(cx, "Hello Vizia").font_size(60).class("value");
            Button::new(cx, |cx| {
                Label::new(cx, "-").font_size(60).class("btnLabel")
            }).corner_radius(Pixels(20.0)).class("button");
        }).alignment(Alignment::Center).class("container");
    })
    .title("First Vizia")
    .min_inner_size(Some((800, 480)))
    .run()    
}
