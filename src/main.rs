use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut high_fives = use_signal(|| 0);

    rsx! {
        link { rel: "stylesheet", href: "/assets/style.css" }
        img { src: "/assets/header.svg" }
        div {
            h1 { "Dioxus iOS apps!" }
            h3 { class: "sparkles", "zero-xcode!!!!" }
            button { onclick: move |_| high_fives += 1, "High five!" }
            p { "High fives: {high_fives}" }
        }
    }
}
