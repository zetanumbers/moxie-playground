use std::str::FromStr;

use moxie_dom::raw::JsCast;
use moxie_dom::{elements::scripting::Canvas, prelude::*};

pub fn get_rendering_context(canvas: &Canvas) -> sys::CanvasRenderingContext2d {
    canvas
        .to_bind()
        .expect_concrete()
        .dyn_ref::<sys::HtmlCanvasElement>()
        .unwrap()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap()
}

pub fn input_value(ev: impl AsRef<sys::Event>) -> String {
    let event: &sys::Event = ev.as_ref();
    let target = event.target().unwrap();
    let input: sys::HtmlInputElement = target.dyn_into().unwrap();
    input.value()
}

pub fn make_setter<T, E>(setter: moxie::Key<T>) -> impl Fn(E)
where
    T: PartialEq + FromStr,
    E: AsRef<sys::Event>,
{
    move |e| drop(input_value(e).parse().map(|v| setter.set(v)))
}

pub fn apply_transform(transform: &sys::DomMatrix, x: f64, y: f64) -> (f64, f64) {
    (
        transform.a() * x + transform.c() * y + transform.e(),
        transform.b() * x + transform.d() * y + transform.f(),
    )
}
