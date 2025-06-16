use std::borrow::Cow;



#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Source<'a> {
    pub source: Cow<'a, str>,
    pub filename: Option<Cow<'a, str>>,
}
impl<'a> Source<'a> {
    pub fn new(source: &'a str, filename: Option<&'a str>) -> Source<'a> {
        Source {
            source: Cow::from(source),
            filename: filename.map(|filename| Cow::from(filename))
        }
    }

    pub fn without_filename<T: std::fmt::Display>(source: T) -> Source<'a> {
        Source {
            source: Cow::from(source.to_string()),
            filename: None,
        }
    }

    pub fn filename(&self) -> Option<String> {
        self.filename.clone().map(String::from)
    }
}

impl<'a> From<&'a str> for Source<'a> {
    fn from(source: &'a str) -> Source<'a> {
        Source::without_filename(source)
    }
}

impl<'a> From<String> for Source<'a> {
    fn from(source: String) -> Source<'a> {
        Source::without_filename(source)
    }
}
