use dioxus::prelude::*;

pub(crate) enum MessageSeverity {
    Error,
    Warning,
    Info,
}

pub(crate) struct Message {
    pub(crate) severity: MessageSeverity,
    pub(crate) message: String
}

impl Message {
    pub(crate) fn error(s: &str) -> Self {
        Self {
            severity: MessageSeverity::Error,
            message: s.to_string()
        }
    }

    pub(crate) fn warning(s: &str) -> Self {
        Self {
            severity: MessageSeverity::Warning,
            message: s.to_string()
        }
    }

    pub(crate) fn info(s: &str) -> Self {
        Self {
            severity: MessageSeverity::Info,
            message: s.to_string()
        }
    }
}

pub(crate) static MESSAGE: GlobalSignal<Option<Message>> = Signal::global(|| None);

pub(crate) fn error_msg(s: &str) {
    *MESSAGE.write() = Some(Message::error(s))
}

pub(crate) fn warning_msg(s: &str) {
    *MESSAGE.write() = Some(Message::warning(s))
}

pub(crate) fn info_msg(s: &str) {
    *MESSAGE.write() = Some(Message::info(s))
}

pub(crate) fn clear_msg() {
    *MESSAGE.write() = None
}