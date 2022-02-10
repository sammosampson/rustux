
use crate::prelude::*;


pub fn link_streams(
    root_location: SourceLocation,
    location: SourceLocation,
    stream_lookup: &AbstractSyntaxTokenStreamLookup
) -> AbstractSyntaxTokenStream {
    let stream = stream_lookup.get(&location).unwrap(); 
    let mut linker = AbstractSyntaxStreamLinker::new(root_location, stream_lookup);
    stream.accept(&mut linker, &mut DataContext::default());
    linker.linked_stream()
}


pub struct AbstractSyntaxStreamLinker<'a> {
    stream_lookup: &'a AbstractSyntaxTokenStreamLookup,
    linked_stream: AbstractSyntaxTokenStream,
    positions: Vec<usize>,
    root_location: SourceLocation,
    control: AbstractSyntaxControlType
}

impl<'a> AbstractSyntaxStreamLinker<'a> {
    fn new(root_location: SourceLocation, stream_lookup: &'a AbstractSyntaxTokenStreamLookup) -> Self {
        Self {
            stream_lookup,
            linked_stream: AbstractSyntaxTokenStream::default(),
            positions: vec!(),
            root_location,
            control: AbstractSyntaxControlType::Unknown
        }
    }
    fn linked_stream(self) -> AbstractSyntaxTokenStream {
        self.linked_stream
    }
}

impl<'a> AbstractSyntaxTokenStreamVisitor for AbstractSyntaxStreamLinker<'a> {
    fn push_last_node_position(&mut self, position: usize) {
        self.positions.push(position);
    }

    fn pop_last_node_position(&mut self) -> Option<usize> {
        self.positions.pop()
    }

    fn start_node(&mut self, node_type: &AbstractSyntaxControlType, _context: &mut DataContext) {
        match node_type {
            AbstractSyntaxControlType::Empty => {},
            AbstractSyntaxControlType::Control => {},
            other => self.linked_stream.start_node(*other),
        }
        self.control = *node_type;
    }

    fn property(&mut self, property: &AbstractSyntaxProperty, _context: &mut DataContext) {
        match self.control {
            AbstractSyntaxControlType::Empty => {},
            AbstractSyntaxControlType::Control => {
                if property.property_type() == &AbstractSyntaxPropertyType::Path {
                    let relative_location = property.value().get_string_value().unwrap();
                    let control_location = self.root_location.to_relative_location(&relative_location).unwrap();
                    let mut control_stream = link_streams(self.root_location.clone(), control_location, &self.stream_lookup);
                    self.linked_stream.append_stream(&mut control_stream)
                }
            },
            _=> self.linked_stream.property(property.clone())
        }
    }

    fn end_node(&mut self, node_type: &AbstractSyntaxControlType, _context: &mut DataContext) -> EndNodeAction {
        match node_type {
            AbstractSyntaxControlType::Empty => {},
            AbstractSyntaxControlType::Control => {},
            other => self.linked_stream.end_node(*other),
        }
        self.control = AbstractSyntaxControlType::Unknown;
        EndNodeAction::Continue
    }

    fn token_error(&mut self, _error: &AbstractSyntaxTokenError) {
    }
}