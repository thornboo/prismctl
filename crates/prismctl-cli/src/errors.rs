#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Usage,
    Runtime,
}

const PREFIX_USAGE: &str = "PRISMCTL_USAGE:";
const PREFIX_RUNTIME: &str = "PRISMCTL_RUNTIME:";

pub fn usage(message: impl Into<String>) -> String {
    format!("{}{}", PREFIX_USAGE, message.into())
}

#[allow(dead_code)]
pub fn runtime(message: impl Into<String>) -> String {
    format!("{}{}", PREFIX_RUNTIME, message.into())
}

pub fn strip_tag(message: &str) -> (Option<ErrorKind>, &str) {
    if let Some(rest) = message.strip_prefix(PREFIX_USAGE) {
        return (Some(ErrorKind::Usage), rest);
    }
    if let Some(rest) = message.strip_prefix(PREFIX_RUNTIME) {
        return (Some(ErrorKind::Runtime), rest);
    }
    (None, message)
}
