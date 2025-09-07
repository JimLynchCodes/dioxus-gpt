use crate::components::Hero;
use dioxus::prelude::*;

const ROUND_TABLE_SVG: Asset = asset!("/assets/cards-table.jpg");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn AppPage() -> Element {
    rsx! {
        div {
            id: "AppPage",

            br {}
            
            // Content
            h1 { "This is the App Page!" }
            br {}
            br {}
            p { " -> Image [Round Table of Department Heads]" }
            img {
                src: ROUND_TABLE_SVG,
                alt: "Round Table of Department Heads",
                style: "max-width: 400px; border-radius: 8px;"
            }
            br {}
            br {}
            br {}
            br {}
            
            p { " -> Select Input: to selected a Department Head" }
            p { "(Idea is that you're only focused on one person at a time)" }
            br {}
            // Select input
            label { "Selected Department Head: " }
            select {
                option { value: "Head of Product", "Head of Product" }
                option { value: "Head of Marketing", "Head of Marketing" }
                option { value: "Head of Financial", "Head of Financial" }
                option { value: "Head of Operating", "Head of Operating" }
                option { value: "Head of Growth", "Head of Growth" }
                option { value: "Head of Technology", "Head of Technology" }
                option { value: "Head of Legal", "Head of Legal" }
                option { value: "Head of Strategy", "Head of Strategy" }
                option { value: "Head of Information Security", "Head of Information Security" }
                option { value: "Head of People", "Head of People" }
            }
            br {}
            br {}
            // Select input
            label { "Question To Ask Department Head:" }
            select {
                option { value: "What of these products are underpriced?", "What of these products are underpriced?" }
                option { value: "What's up?", "What's up?" }
                
            }
             p { "^ Question options change depending on which member is selected." }
             br {}
             p { "Input box to type your own custom question, along with save button to save the custom question for reuse later." }

            br {}
            br {}
            br {}
            br {}
            br {}
            br {}
            br {}
            br {}
            br {}
            
            p { " -> File upload button that takes a csv, Upload orders csv" }
            br {}
            // File upload for orders CSV
            label { "Upload orders CSV: " }
            input {
                r#type: "file",
                accept: ".csv"
            }
            br {}
            br {}
            
            p { " -> File upload button that takes a csv, Upload inventory csv" }
            br {}
            // File upload for inventory CSV
            label { "Upload inventory CSV: " }
            input {
                r#type: "file",
                accept: ".csv"
            }
            
            p { " -> File upload button that takes a csv, Upload customer rewards csv" }
            br {}
            // File upload for inventory CSV
            label { "Upload customer loyalty data CSV: " }
            input {
                r#type: "file",
                accept: ".csv"
            }
            
            br {}
            br {}
            br {}
            br {}
      
        }
    }
}
