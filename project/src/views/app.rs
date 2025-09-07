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
            p { " -> Image [Round Table of C Suite Members]" }
            img {
                src: ROUND_TABLE_SVG,
                alt: "Round Table of C Suite Members",
                style: "max-width: 400px; border-radius: 8px;"
            }
            br {}
            br {}
            br {}
            br {}
            
            p { " -> Select Input: to selected a C Suite Member" }
            p { "(Idea is that you're only focused on one person at a time)" }
            br {}
            // Select input
            label { "Selected C Suite Member: " }
            select {
                option { value: "Chief Product Officer", "Chief Product Officer" }
                option { value: "Chief Marketing Officer", "Chief Marketing Officer" }
                option { value: "Chief Financial Officer", "Chief Financial Officer" }
                option { value: "Chief Operating Officer", "Chief Operating Officer" }
                option { value: "Chief Growth Officer", "Chief Growth Officer" }
                option { value: "Chief Technology Officer", "Chief Technology Officer" }
                option { value: "Chief Legal Officer", "Chief Legal Officer" }
                option { value: "Chief Strategy Officer", "Chief Strategy Officer" }
                option { value: "Chief Information Secutiy Officer", "Chief Information Secutiy Officer" }
                option { value: "Chief People Officer", "Chief People Officer" }
            }
            br {}
            br {}
            // Select input
            label { "Question To Ask C Suite Member:" }
            select {
                option { value: "What's up?", "What's up?" }
                option { value: "What of these products are underpriced?", "What of these products are underpriced?" }
                
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
