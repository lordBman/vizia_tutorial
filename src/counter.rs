use vizia::{
    binding::Lens, context::{Context, EmitContext, EventContext}, events::Event, localization::Localized, modifiers::{ActionModifiers, StyleModifiers}, view::{Handle, View}, views::{Button, HStack, Label}
};

pub enum CounterEvent { Minus, Plus }

pub struct Counter{
    on_plus: Option<Box<dyn Fn(&mut EventContext)>>,
    on_minus: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl View for Counter {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|counter_event, _meta| match counter_event{
            CounterEvent::Plus => {
                if let Some(callback) = &self.on_plus {
                    (callback)(cx);
                }
            }
            CounterEvent::Minus => {
                if let Some(callback) = &self.on_minus {
                    (callback)(cx);
                }
            }
        });
    }
}

pub trait CounterModifiers {
    fn plus<F: Fn(&mut EventContext) + 'static>(self, callback: F) -> Self;
    fn minus<F: Fn(&mut EventContext) + 'static>(self, callback: F) -> Self;
}

impl<'a> CounterModifiers for Handle<'a, Counter> {
    fn plus<F: Fn(&mut EventContext) + 'static>(self, callback: F) -> Self {
        self.modify(|counter| counter.on_plus = Some(Box::new(callback)))
    }

    fn minus<F: Fn(&mut EventContext) + 'static>(self, callback: F) -> Self {
        self.modify(|counter| counter.on_minus = Some(Box::new(callback)))
    }
}

impl Counter {
    pub fn new<L: Lens<Target = i32>>(cx: &mut Context, lens: L) -> Handle<Self> {
        Self { on_minus: None, on_plus: None }.build(cx, |cx|{
            HStack::new(cx, |cx|{
                Button::new(cx, |cx|{
                    Label::new(cx, Localized::new("dec")).class("minusBtnLabel")
                }).class("minusBtn").on_press(|event| event.emit(CounterEvent::Minus));
                Label::new(cx, lens).class("value");
                Button::new(cx, |cx|{
                    Label::new(cx, Localized::new("inc")).class("plusBtnLabel")
                }).class("plusBtn").on_press(|event| event.emit(CounterEvent::Plus));
            }).class("container");
        })
    }
}