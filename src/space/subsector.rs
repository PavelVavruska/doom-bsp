use super::line::Line;
pub struct Subsector {
    pub line_segments: Vec<Line>,
}

impl Subsector {
    pub fn new(line_segments: &[Line]) -> Subsector {
        Subsector {
            line_segments: line_segments.to_vec(),
        }
    }

    pub fn get_lines(&self) -> &Vec<Line> {
        &self.line_segments
    }
}
