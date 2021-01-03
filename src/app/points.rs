use std::f64::consts::TAU;

use moxie_dom::prelude::*;

use super::utils::apply_transform;

pub trait Points {
    fn fill_point(&self, x: f64, y: f64);
}

impl Points for sys::CanvasRenderingContext2d {
    fn fill_point(&self, x: f64, y: f64) {
        self.save();
        self.begin_path();
        let transform = self.get_transform().unwrap();
        self.reset_transform().unwrap();

        let (x, y) = apply_transform(&transform, x, y);
        self.arc(x, y, 2.0, 0.0, TAU).unwrap();

        self.fill();
        self.restore();
    }
}
