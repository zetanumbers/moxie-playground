use mox::mox;
use moxie_dom::{
    elements::{html::*, text_content::Div},
    prelude::*,
};
use sys::window;

mod lab06;

pub fn main() -> Div {
    match sys::UrlSearchParams::new_with_str(&window().unwrap().location().search().unwrap())
        .unwrap()
        .get("lab")
        .as_ref()
        .map(String::as_str)
    {
        Some("06") => lab06::main(),
        Some(lab) => mox! { <div>{% "lab-{} is not found", lab}</div> },
        None => mox! { <div /> },
    }
}
