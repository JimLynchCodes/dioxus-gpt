use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            // Attributes should be defined in the element before any children
            id: "hero",
            // After all attributes are defined, we can define child elements and components
            // img { src: HEADER_SVG, id: "header" }
            h1 { "Department Heads App" }
            div { id: "links",
                // The RSX macro also supports text nodes surrounded by quotes
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Get Starter" }
                // a { href: "https://github.com/dioxus-community/", "📡 Community Forums" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 Chrome Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
