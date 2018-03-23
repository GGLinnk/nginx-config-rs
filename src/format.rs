use std::fmt;
use std::default::Default;


#[derive(Debug, PartialEq)]
pub(crate) struct Formatter<'a> {
    buf: String,
    style: &'a Style,
    indent: u32,
}

/// A configuration of formatting style
///
/// Currently we only have indentation configured, other things might be
/// added later.
#[derive(Debug, PartialEq, Clone)]
pub struct Style {
    indent: u32,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            indent: 4,
        }
    }
}

impl Style {
    /// Change the number of spaces used for indentation
    pub fn indent(&mut self, indent: u32) -> &mut Self {
        self.indent = indent;
        self
    }
}

pub(crate) trait Displayable {
    fn display(&self, f: &mut Formatter);
}

impl<'a> Formatter<'a> {
    pub fn new(style: &Style) -> Formatter {
        Formatter {
            buf: String::with_capacity(1024),
            style,
            indent: 0,
        }
    }

    pub fn indent(&mut self) {
        for _ in 0..self.indent {
            self.buf.push(' ');
        }
    }

    pub fn end(&mut self) {
        self.buf.push(';');
        self.buf.push('\n');
    }
    pub fn endline(&mut self) {
        self.buf.push('\n');
    }

    pub fn start_block(&mut self) {
        self.buf.push('{');
        self.endline();
        self.indent += self.style.indent;
    }

    pub fn end_block(&mut self) {
        self.indent = self.indent.checked_sub(self.style.indent)
            .expect("negative indent");
        self.indent();
        self.buf.push('}');
        self.endline();
    }

    pub fn margin(&mut self) {
        if !self.buf.is_empty() && !self.buf.ends_with("{\n") {
            self.buf.push('\n');
        }
    }

    pub fn write(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    pub fn fmt<D: fmt::Display>(&mut self, s: &D) {
        use std::fmt::Write;
        write!(&mut self.buf, "{}", s).expect("write never fails");
    }

    pub fn into_string(self) -> String {
        self.buf
    }
}
