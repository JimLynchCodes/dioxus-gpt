use crate::Route;
use dioxus::prelude::*;

const SETTINGS_CSS: Asset = asset!("/assets/styling/settings.css");

#[component]
pub fn Settings(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SETTINGS_CSS }

        div {
            id: "settings",

            // Content
            h1 { "This is the settings page!" }
            p { "settings route id: #{id}"}

        }
    }
}
