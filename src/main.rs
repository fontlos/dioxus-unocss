#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    let mut count = use_signal(|| 0);
    rsx!(
        link {
            rel: "stylesheet",
            href: "./assets/uno.css"
        }
        div{
            class: "mt-20 w-full h-full flex flex-col items-center justify-center gap-40",
            div{
                class: "flex justify-center gap-20",
                img {
                    class: "w-70em",
                    src: "./assets/header.svg",
                }
                img {
                    src: "./assets/uno.svg"
                }
            }
            div {
                class: "text-blue-900 text-size-8em",
                "HELLO WORLD!"
            }
            button {
                class: "w-10em p-10 bg-blue-200 border-none rounded-50 text-blue-900 text-size-2em flex items-center justify-center gap-10 hover:(bg-gray-400 font-medium)",
                onclick: move |_| count+=1,
                div {
                    class: "w-10 h-10 text-blue-900 i-icon:click"
                }
                div {
                    "Clicks: {count}"
                }
            }
        }
    )
}
