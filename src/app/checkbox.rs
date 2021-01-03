use moxie_dom::raw::JsCast;
use moxie_dom::{elements::forms::Input, prelude::*, raw};

pub struct Checkbox {
    pub checked: bool,
    callbacks: Vec<Box<dyn FnMut(raw::event::Change) + 'static>>,
}

pub fn checkbox() -> Checkbox {
    Checkbox {
        checked: false,
        callbacks: vec![],
    }
}

fn checked(ev: impl AsRef<sys::Event>) -> bool {
    let event: &sys::Event = ev.as_ref();
    let target = event.target().unwrap();
    let input: sys::HtmlInputElement = target.dyn_into().unwrap();
    input.checked()
}

pub fn make_setter<E>(setter: moxie::Key<bool>) -> impl Fn(E)
where
    E: AsRef<sys::Event>,
{
    move |e| drop(setter.set(checked(e)))
}

impl Checkbox {
    pub fn value(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn onchange(mut self, callback: impl FnMut(raw::event::Change) + 'static) -> Self {
        self.callbacks.push(Box::from(callback));
        self
    }

    pub fn build(mut self) -> Input {
        let mut element = html::input().type_("checkbox");
        for callback in self.callbacks.drain(..) {
            element = element.onchange(callback);
        }

        let element = if self.checked {
            element.checked("")
        } else {
            element
        };
        element.build()
    }
}
