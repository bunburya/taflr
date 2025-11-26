use dioxus::prelude::*;
use crate::route::Route;

const HOME_ICON: &str = include_str!("../../assets/icons/house_simple_32x32.svg");
const QUIT_ICON: &str = include_str!("../../assets/icons/longship_half_simple_32x32.svg");

#[component]
fn HeaderIcon(html: &'static str, alt: &'static str, to: Route) -> Element {
    rsx! {
        Link {
            class: "header-icon",
            alt: alt,
            to: to,
            dangerous_inner_html: html
        }
    }
}

#[component]
pub(crate) fn HeaderBar(title: String) -> Element {
    rsx! {
        div {
            class: "header-bar",
            div {
                class: "header-text",
                { title }
            }

            div {
                class: "header-button-container",
                HeaderIcon {
                    html: HOME_ICON,
                    alt: "Home",
                    to: Route::MainMenu
                }
                HeaderIcon {
                    html: QUIT_ICON,
                    alt: "Quit",
                    to: Route::Quit
                }
            }
        }
    }
}