use std::{borrow::Cow, fmt::Write};

/// String replacement pattern on file paths
pub struct PathPattern<'a> {
    /// The inner string data for replacement
    data: Vec<PathPatternItem<'a>>,

    /// The maximum length of the path after replacement
    length: usize,
}

/// A single string or replacement item
enum PathPatternItem<'a> {
    /// A static string
    String(&'a str),

    /// The day number
    Number,

    /// The zero-padded day number
    ZeroPadNumber,
}

impl<'a> PathPattern<'a> {
    /// Parse a path into a substitution pattern
    pub fn new(path: &'a str) -> Self {
        let mut data = vec![];

        for comp in path.split('{') {
            if comp.starts_with('}') {
                data.push(PathPatternItem::Number);
                data.push(PathPatternItem::String(&comp[1..]));
            } else if comp.starts_with("0}") {
                data.push(PathPatternItem::ZeroPadNumber);
                data.push(PathPatternItem::String(&comp[2..]));
            } else {
                data.push(PathPatternItem::String(comp));
            }
        }

        Self { data, length: 0 }
    }

    /// substitute the given day number into the path
    pub fn replace(&self, day: usize) -> Cow<'a, str> {
        if self.data.len() == 1 {
            if let PathPatternItem::String(s) = self.data[0] {
                return s.into();
            }
        }

        let mut output = String::with_capacity(self.length);
        for item in &self.data {
            match item {
                PathPatternItem::String(s) => output.push_str(s),

                // The impl of `Write` for `String` currently will never panic
                // (unless OOM) so this unwrap should never happen.
                PathPatternItem::Number => write!(output, "{day}").unwrap(),
                PathPatternItem::ZeroPadNumber => write!(output, "{day:02}").unwrap(),
            }
        }

        output.into()
    }
}
