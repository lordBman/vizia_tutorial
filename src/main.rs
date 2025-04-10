use vizia::{Application, ApplicationError};
use vizia::prelude::*;



pub enum AppEvent { Plus, Minus }
#[derive(Lens)]
pub struct AppData {
    pub count: i32,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _meta| match app_event {
            AppEvent::Minus => self.count -= 1,
            AppEvent::Plus => self.count += 1,
        });
    }
}

//Lession 5 - Model and State handling
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx|{
        cx.add_stylesheet(include_style!("css/styles.css")).expect("unable to find stylesheet file");

        AppData { count: 0 }.build(cx); // Build the data into the app

        HStack::new(cx, |cx|{
            Button::new(cx, |cx|{
                Label::new(cx, "-").class("minusBtnLabel")
            }).class("minusBtn").on_press(|event| event.emit(AppEvent::Minus));
            Label::new(cx, AppData::count).class("value");
            Button::new(cx, |cx|{
                Label::new(cx, "+").class("plusBtnLabel")
            }).class("plusBtn").on_press(|event| event.emit(AppEvent::Plus));
        }).class("container");
    }).title("First Vizia").min_inner_size(Some((800, 480))).run()    
}
