use pest::Position;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SpanPosition {
    pub line: usize,
    pub column: usize,
}

impl SpanPosition {
    pub fn from_tuple(line_col: (usize, usize)) -> SpanPosition {
        let (line, column) = line_col;
        SpanPosition { line, column }
    }

    pub fn from_pest(position: Position) -> SpanPosition {
        let (line, column) = position.line_col();
        SpanPosition { line, column }
    }

    pub fn to_tuple(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}
