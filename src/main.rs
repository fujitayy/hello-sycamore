use futures::TryFutureExt;
use gloo::render::request_animation_frame;
use std::{cell::Cell, rc::Rc};
use sycamore::{futures::create_resource, prelude::*};

fn main() {
    sycamore::render(|cx| {
        let ferris_wh = 64;
        let ferris_r = ferris_wh / 2;
        let area_wh = 1000;

        let ferrises = create_signal(cx, Vec::new());
        create_resource(cx, async move {
            let url = "https://raw.githubusercontent.com/fujitayy/hello-sycamore/main/ferris_initial_params.json";
            ferrises.set(
                gloo::net::http::Request::get(url)
                    .send()
                    .and_then(|r| async move { r.json().await })
                    .unwrap_or_else(|err| {
                        gloo::console::log!(format!(
                            "Failed to load ferris initial params json: {err}"
                        ));
                        vec![(0, area_wh / 2, area_wh / 2, 7, -3)]
                    })
                    .await
                    .into_iter()
                    .map(|x| create_signal(cx, x))
                    .collect(),
            );
        });

        let frame_handle = Rc::new(Cell::new(None));
        let tick = create_rc_signal(0u64);

        create_effect(cx, move || {
            let _ = tick.get();

            for ferris in ferrises.get().iter() {
                let (image_index, x, y, v_x, v_y) = *ferris.get();
                let (x, v_x) = move_1_axis(x, v_x, ferris_r, 0, area_wh);
                let (y, v_y) = move_1_axis(y, v_y, ferris_r, 0, area_wh);
                ferris.set((image_index, x, y, v_x, v_y));
            }

            let tick = tick.clone();
            let h = request_animation_frame(move |t| tick.set(t as u64));
            frame_handle.set(Some(h));
        });

        let ferris_image_url_list = Rc::new(vec!["/images/ferris.svg", "/images/ferris2.svg"]);

        view! { cx,
            svg(
                xmlns = "http://www.w3.org/2000/svg",
                viewBox = format!("0 0 {area_wh} {area_wh}"),
                style = "width:90vmin; height:90vmin; border:1px solid #000",
            ) {
                Indexed (
                    iterable = ferrises,
                    view = move |cx, ferris| {
                        let ferris_image_url_list = ferris_image_url_list.clone();
                        view! { cx,
                            image (
                                href = ferris_image_url_list[ferris.get().0],
                                width = ferris_wh,
                                height = ferris_wh,
                                x = (ferris.get().1 - ferris_r),
                                y = (ferris.get().2 - ferris_r),
                                on:click = move |_| {
                                    gloo::console::log!("aaa");
                                    let (index, x,y, v_x, v_y) = *ferris.get();
                                    ferris.set(((index + 1) % 2, x, y, v_x, v_y));
                                },
                            )
                        }
                    },
                )
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
