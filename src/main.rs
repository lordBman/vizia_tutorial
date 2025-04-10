use vizia::{Application, ApplicationError};
use vizia::prelude::*;

//Lession 4 - Shared Styles(css)
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx|{
        cx.add_stylesheet(include_style!("css/styles.css")).expect("unable to find stylesheet file");

        HStack::new(cx, |cx|{
            Button::new(cx, |cx|{
                Label::new(cx, "-").class("minusBtnLabel")
            }).class("minusBtn");
            Label::new(cx, "0").class("value");
            Button::new(cx, |cx|{
                Label::new(cx, "+").class("plusBtnLabel")
            }).class("plusBtn");
        }).class("container");
    }).title("First Vizia").min_inner_size(Some((800, 480))).run()    
}
