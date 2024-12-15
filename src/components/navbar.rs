use crate::Route;
use dioxus::prelude::*;

// const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        // document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            class: "rounded-b-md border-b-2 backdrop-blur-sm bg-white/30  sticky top-0 p-2 flex space-x-2",
            id: "navbar",
            Link {
                class: "rounded-md border-2 m-1 p-0.5",
                to: Route::Home {},
                "Home"
            }
            Link {
                class: "rounded-md border-2 m-1 p-0.5",
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
