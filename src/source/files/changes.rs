use crate::prelude::*;

pub fn create_source_changes() -> SourceChanges {
    SourceChanges::default()
}

#[derive(Default)]
pub struct SourceChanges(Vec<SourceLocation>);

impl From<SourceLocation> for SourceChanges {
    fn from(from: SourceLocation) -> Self {
        Self(vec!(from))
    }
}

impl SourceChanges {
    pub fn push(&mut self, change: SourceLocation) {
        self.0.push(change);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, SourceLocation> {
        self.0.iter()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}