use crate::{
    arrows::Arrows,
    checkbox::{self, checkbox},
    points::Points,
    utils::*,
};
use instant::Instant;
use mox::mox;
use moxie_dom::{
    elements::{html::*, text_content::Div},
    interfaces::node::Child,
    prelude::*,
};
use std::{cell::Cell, f64::consts::TAU, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

type Vec2 = cgmath::Vector2<f64>;
const FRAMERATE: f64 = 60.0;

fn calc_points(oa: f64, ab: f64, am_per_ab: f64, progress: f64) -> (Vec2, Vec2, Vec2) {
    use std::cmp::Ordering::*;
    let t = progress * TAU;
    let ob = match oa.partial_cmp(&ab).unwrap() {
        Less => oa * t.cos() + ab,
        Equal => (oa + ab) * t.cos(),
        Greater => oa + ab * t.cos(),
    };
    let b: Vec2 = Vec2::new(ob, 0.0);
    let a_x = (ob * ob + oa * oa - ab * ab) / (2.0 * ob);
    let a_y = (oa * oa - a_x * a_x).sqrt().copysign(0.5 - progress);
    let a: Vec2 = Vec2::new(a_x, a_y);
    let m: Vec2 = a + am_per_ab * (b - a);

    (a, b, m)
}

pub fn main() -> Div {
    let (progress, set_progress) = moxie::state(|| 0.0);
    let (oa, set_oa) = moxie::state(|| 1.0);
    let (ab, set_ab) = moxie::state(|| 1.0);
    let (am_per_ab, set_am_per_ab) = moxie::state(|| 0.5);
    let (period, set_period) = moxie::state(|| 5.0);
    let (autorun, set_autorun) = moxie::state(|| false);
    let (trace_path, set_trace_path) = moxie::state(|| true);

    {
        let set_progress = set_progress.clone();
        let interval_id = moxie::once(|| Rc::new(Cell::new(None)));
        let last_frame = moxie::once(|| Rc::new(Cell::new(Instant::now())));

        moxie::cache_with(
            &(*autorun, *period),
            |(autorun, period)| {
                let period = *period;

                let cb = Closure::wrap(Box::new({
                    let last_frame = last_frame.clone();
                    move || {
                        let now = Instant::now();
                        let dt = (now - last_frame.replace(now)).as_secs_f64();
                        set_progress.update(|p| Some((p + dt / period) % 1.0));
                    }
                }) as Box<dyn Fn()>);

                if let Some(id) = interval_id.get() {
                    sys::window().unwrap().clear_interval_with_handle(id);
                }

                interval_id.set(if *autorun {
                    last_frame.set(Instant::now());
                    Some(
                        sys::window()
                            .unwrap()
                            .set_interval_with_callback_and_timeout_and_arguments_0(
                                cb.as_ref().dyn_ref().unwrap(),
                                (1000.0 / FRAMERATE) as i32,
                            )
                            .unwrap(),
                    )
                } else {
                    None
                });

                cb
            },
            |_| (),
        );
    }

    let canvas = mox! {
        <canvas width=480 height=480 />
    };

    {
        let ctx = get_rendering_context(&canvas);
        ctx.clear_rect(0.0, 0.0, 480.0, 480.0);
        ctx.set_font("normal 20px times");
        ctx.set_text_baseline("top");

        fn tx(x: f64) -> f64 {
            (x + 1.0) * 240.0
        }
        fn ty(y: f64) -> f64 {
            (-y + 1.0) * 240.0
        }

        // axes
        ctx.begin_path();
        ctx.arrow(tx(-1.0), ty(0.0), tx(1.0), ty(0.0));
        ctx.arrow(tx(0.0), ty(-1.0), tx(0.0), ty(1.0));
        ctx.stroke();
        ctx.set_text_align("right");
        ctx.fill_text("x", tx(1.0), ty(0.0) + 10.0).unwrap();
        ctx.set_text_align("left");
        ctx.fill_text("y", tx(0.0) + 10.0, ty(1.0)).unwrap();

        let tx = |x: f64| tx(x * 0.9 / (*oa + *ab));
        let ty = |y: f64| ty(y * 0.9 / (*oa + *ab));
        let named_path = |points: &[(Vec2, &str)]| {
            ctx.begin_path();
            ctx.move_to(tx(points[0].0.x), ty(points[0].0.y));
            for point in points.iter().skip(1) {
                ctx.line_to(tx(point.0.x), ty(point.0.y));
            }
            ctx.stroke();
            for point in points {
                ctx.fill_point(tx(point.0.x), ty(point.0.y));
                ctx.fill_text(point.1, tx(point.0.x), ty(point.0.y))
                    .unwrap();
            }
        };

        // handle
        let o = Vec2::new(0.0, 0.0);
        let (a, b, m) = calc_points(*oa, *ab, *am_per_ab, *progress);
        named_path(&[(o, "O"), (a, "A"), (b, "B"), (m, "M")]);

        // path
        if *trace_path {
            moxie::cache_with(
                &(*oa, *ab, *am_per_ab, *progress),
                |(oa, ab, am_per_ab, progress)| {
                    let path = sys::Path2d::new().unwrap();
                    path.move_to(tx(0.0), ty(0.0));
                    for i in 0..500 {
                        let (_a, _b, m) =
                            calc_points(*oa, *ab, *am_per_ab, i as f64 / 500.0 * progress);
                        path.line_to(tx(m.x), ty(m.y));
                    }
                    path
                },
                |path| ctx.stroke_with_path(path),
            );
        }
    }

    mox! {
        <div>
            <table>
                <tr>
                    <td rowspan=6>
                        {canvas}
                    </td>
                    <td>
                        "OA = "
                        <input type="number"
                            value=*oa
                            step="any"
                            onchange=make_setter(set_oa) />
                    </td>
                </tr>
                <tr>
                    <td>
                        "AB = "
                        <input type="number"
                            value=*ab
                            step="any"
                            onchange=make_setter(set_ab) />
                    </td>
                </tr>
                <tr>
                    <td>
                        "AM / AB = "
                        <input type="range"
                            min=0.0 max=1.0 step="any"
                            value=*am_per_ab
                            oninput=make_setter(set_am_per_ab) />
                    </td>
                </tr>
                <tr>
                    <td>
                        "Period = "
                        <input type="number"
                            value=*period
                            step="any"
                            style="width: 40pt"
                            onchange=make_setter(set_period) />
                        " seconds per cycle"
                    </td>
                </tr>
                <tr>
                    <td>
                        "Autorun "
                        <checkbox
                            value=*autorun
                            onchange=checkbox::make_setter(set_autorun) />
                    </td>
                </tr>
                <tr>
                    <td>
                        "Trace M point's path "
                        <checkbox
                            value=*trace_path
                            onchange=checkbox::make_setter(set_trace_path) />
                    </td>
                </tr>
                <tr>
                    <td colspan=2>
                        <input type="range"
                            style="width: 100%"
                            min=0.0 max=1.0 step="any"
                            value=progress
                            oninput=make_setter(set_progress) />
                    </td>
                </tr>
            </table>
        </div>
    }
}
