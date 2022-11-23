use gloo::render::request_animation_frame;
use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        let ferris_wh = 64;
        let ferris_r = ferris_wh / 2;
        let area_wh = 1000;

        let frame_handle = create_rc_signal(None);
        let tick = create_rc_signal(0u64);
        let ferris = create_signal(cx, (area_wh / 2, area_wh / 2, 7, -3));

        create_effect(cx, move || {
            let _ = tick.get();
            let (x, y, v_x, v_y) = *ferris.get();
            let (x, v_x) = move_1_axis(x, v_x, ferris_r, 0, area_wh);
            let (y, v_y) = move_1_axis(y, v_y, ferris_r, 0, area_wh);
            ferris.set((x, y, v_x, v_y));

            let tick = tick.clone();
            let h = request_animation_frame(move |t| tick.set(t as u64));
            frame_handle.set(Some(h));
        });

        let ferris_image_url_list = vec!["/images/ferris.svg", "/images/ferris2.svg"];
        let ferris_image_index = create_signal(cx, 0);

        view! { cx,
            svg(
                xmlns = "http://www.w3.org/2000/svg",
                viewBox = format!("0 0 {area_wh} {area_wh}"),
                style = "width:90vmin; height:90vmin; border:1px solid #000",
            ) {
                image (
                    href = ferris_image_url_list[*ferris_image_index.get()],
                    width = ferris_wh,
                    height = ferris_wh,
                    x = (ferris.get().0 - ferris_r),
                    y = (ferris.get().1 - ferris_r),
                    on:click = |_| { ferris_image_index.set( (*ferris_image_index.get() + 1) % 2); },
                ) {}
            }
        }
    });
}

fn move_1_axis(pos: i16, v: i16, r: i16, lower: i16, upper: i16) -> (i16, i16) {
    let new_pos = pos + v;
    let upper = upper - r;
    let lower = lower + r;
    if new_pos >= upper {
        (upper, v * -1)
    } else if new_pos <= lower {
        (lower, v * -1)
    } else {
        (new_pos, v)
    }
}
