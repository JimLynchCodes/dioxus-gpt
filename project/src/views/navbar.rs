use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const HEADER_LOGO: Asset = asset!("assets/department-heads-logo-v1.png");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            div {
                id: "navbar-left",
                img {
                    src: HEADER_LOGO
                }
            }
            div {
                id: "navbar-right",
                Link {
                    to: Route::Home {},
                    "Home"
                }
                Link {
                    to: Route::AppPage {},
                    "App" 
                }
                Link {
                    to: Route::Settings { id: 1 },
                    "Settings"
                }
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
