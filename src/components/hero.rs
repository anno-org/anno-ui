use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1",
            img { src: HEADER_SVG, id: "header" }
            div { class: "flex flex-col justify-self-center",
                a {
                    class: "p-4 m-1",
                    href: "https://dioxuslabs.com/learn/0.6/",
                    "📚 Learn Dioxus"
                }
                a {
                    class: "p-4 m-1",
                    href: "https://dioxuslabs.com/awesome",
                    "🚀 Awesome Dioxus"
                }
                a {
                    class: "p-4 m-1",
                    href: "https://github.com/dioxus-community/",
                    "📡 Community Libraries"
                }
                a {
                    class: "p-4 m-1",
                    href: "https://github.com/DioxusLabs/sdk",
                    "⚙️ Dioxus Development Kit"
                }
                a {
                    class: "p-4 m-1",
                    href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a {
                    class: "p-4 m-1",
                    href: "https://discord.gg/XgGxMSkvUM",
                    "👋 Community Discord"
                }
            }
        }
    }
}
