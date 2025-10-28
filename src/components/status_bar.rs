use dioxus::prelude::*;
use crate::message::{MessageSeverity, MESSAGE};

#[component]
pub(crate) fn StatusBar() -> Element {
    let (class, text) = if let Some(msg) = &*MESSAGE.read() {
        let t = msg.message.clone();
        match msg.severity {
            MessageSeverity::Error => ("status-bar-error", t),
            MessageSeverity::Warning => ("status-bar-warning", t),
            MessageSeverity::Info => ("status-bar-info", t),
        }
    } else {
        ("status-bar-none", String::new())
    };
    rsx! {
        div {
            class,
            "{text}"
        }
    }
}