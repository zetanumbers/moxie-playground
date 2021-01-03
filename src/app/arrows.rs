use moxie_dom::prelude::*;

use super::utils::apply_transform;

pub trait Arrows {
    fn arrow(&self, x1: f64, y1: f64, x2: f64, y2: f64);
}

impl Arrows for sys::CanvasRenderingContext2d {
    fn arrow(&self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.save();
        let transform = self.get_transform().unwrap();
        self.reset_transform().unwrap();

        let (x1, y1) = apply_transform(&transform, x1, y1);
        let (x2, y2) = apply_transform(&transform, x2, y2);
        let (dx, dy) = (x2 - x1, y2 - y1);
        let len = dx.hypot(dy);
        let (ndx, ndy) = (dx / len, dy / len);
        let (pdx, pdy) = (-ndy, ndx);

        const ARROW_HEAD_RADIUS: f64 = 5.0;
        const ARROW_HEAD_LENGTH: f64 = 10.0;

        self.move_to(x1, y1);
        self.line_to(x2, y2);
        self.line_to(
            x2 - ndx * ARROW_HEAD_LENGTH + pdx * ARROW_HEAD_RADIUS,
            y2 - ndy * ARROW_HEAD_LENGTH + pdy * ARROW_HEAD_RADIUS,
        );
        self.move_to(
            x2 - ndx * ARROW_HEAD_LENGTH - pdx * ARROW_HEAD_RADIUS,
            y2 - ndy * ARROW_HEAD_LENGTH - pdy * ARROW_HEAD_RADIUS,
        );
        self.line_to(x2, y2);

        self.restore();
    }
}
