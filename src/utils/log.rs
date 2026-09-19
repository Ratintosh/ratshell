use color_print::{cprintln};

pub fn err(text: &str) {
    cprintln!("<red>[ERROR]</> {}", text);
}

pub fn usage(text: &str) {
    cprintln!("<yellow>[USAGE]</> {}", text);
}

pub fn warn(text: &str) {
   cprintln!("<yellow>[WARN]</> {}", text);
}