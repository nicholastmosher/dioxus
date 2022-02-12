use dioxus::prelude::*;

fn main() {
    rink::launch(app);
}

fn app(cx: Scope) -> Element {
    let (radius, set_radius) = use_state(&cx, || 0);

    cx.render(rsx! {
        div {
            width: "100%",
            height: "100%",
            justify_content: "center",
            align_items: "center",
            background_color: "hsl(248, 53%, 58%)",
            onwheel: move |w| set_radius((radius + w.delta_y as i8).abs()),

            // the border can either be solid, double, thick, OR rounded
            // if multable are set only the last style is appiled
            // to skip a side set the style to none
            border_style: "solid none solid double",
            border_width: "thick",
            border_radius: "{radius}px",
            border_color: "#0000FF #FF00FF #FF0000 #00FF00",

            "{radius}"
        }
    })
}
