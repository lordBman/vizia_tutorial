use vizia::{Application, ApplicationError};
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx|{
       Label::new(cx, "First Label")
            .font_size(32.0)
            .color("#3367ff")
            .font_weight(300).padding(Pixels(20.0));
    }).title("First Vizia").min_inner_size(Some((800, 480))).run()    
}
