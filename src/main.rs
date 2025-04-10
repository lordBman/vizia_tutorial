use vizia::{Application, ApplicationError};
use vizia::prelude::*;

//Lession 3 - HStack and Buttons
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx|{
        HStack::new(cx, |cx|{
            Button::new(cx, |cx|{
                Label::new(cx, "-").color("#337Aff").font_size(20.0)
            }).background_color("#00000000").border_width(Pixels(2.0)).border_color("#337Aff").size(Pixels(50.0)).corner_radius("50%");
            Label::new(cx, "0")
                .font_size(32.0)
                .color("#337Aff")
                .font_weight(300).padding(Pixels(20.0));
            Button::new(cx, |cx|{
                Label::new(cx, "+").color("#ffffff").font_size(20.0)
            }).background_color("#337Aff").border_width(Pixels(2.0)).border_color("#337Aff").size(Pixels(50.0)).corner_radius("50%");
        }).space(Pixels(10.0)).alignment(Alignment::Center);
    }).title("First Vizia").min_inner_size(Some((800, 480))).run()    
}
