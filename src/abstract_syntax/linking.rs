
use core::panic;

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
    control: AbstractSyntaxControlType,
    path: Option<String>
}

impl<'a> AbstractSyntaxStreamLinker<'a> {
    fn new(root_location: SourceLocation, stream_lookup: &'a AbstractSyntaxTokenStreamLookup) -> Self {
        Self {
            stream_lookup,
            linked_stream: AbstractSyntaxTokenStream::default(),
            positions: vec!(),
            root_location,
            control: AbstractSyntaxControlType::Unknown,
            path: None
        }
    }

    fn linked_stream(self) -> AbstractSyntaxTokenStream {
        self.linked_stream
    }

    fn start_scope_node(&mut self) {
        self.linked_stream.start_node(AbstractSyntaxControlType::Scope)
    }

    fn start_other_node(&mut self, other: AbstractSyntaxControlType) {
        self.linked_stream.start_node(other)
    }
    
    fn end_scope_node(&mut self) {
        self.linked_stream.end_node(AbstractSyntaxControlType::Scope)
    }

    fn end_other_node(&mut self, other: AbstractSyntaxControlType) {
        self.linked_stream.end_node(other)
    }

    fn store_path(&mut self, property: &AbstractSyntaxProperty) {
        self.path = Some(property.value().get_string_value().unwrap());
    }

    fn copy_property(&mut self, property: &AbstractSyntaxProperty) {
        self.linked_stream.property(property.clone())
    }

    fn append_stream_at_path(&mut self) {
        if let Some(path) = &self.path.clone() {
            self.append_linked_stream(&path);
        } else {
            panic!()
        }
    }

    fn append_linked_stream(&mut self, relative_location: &str) {
        let control_location = self.root_location.to_relative_location(&relative_location).unwrap();
        let mut control_stream = link_streams(self.root_location.clone(), control_location, &self.stream_lookup);
        self.linked_stream.append_stream(&mut control_stream);
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
            AbstractSyntaxControlType::ControlReference => self.start_scope_node(),
            other => self.start_other_node(*other),
            
        }
        self.control = *node_type;
    }

    fn property(&mut self, property: &AbstractSyntaxProperty, _context: &mut DataContext) {
        match self.control {
            AbstractSyntaxControlType::Empty => {},
            AbstractSyntaxControlType::ControlReference => {
                match property.property_type() {
                    AbstractSyntaxPropertyType::Path => self.store_path(property),
                    AbstractSyntaxPropertyType::ControlArguments => self.copy_property(property),
                    _ => {}
                }
            },
            _=> self.copy_property(property)
        }
    }

    fn end_node(&mut self, node_type: &AbstractSyntaxControlType, _context: &mut DataContext) -> EndNodeAction {
        match node_type {
            AbstractSyntaxControlType::Empty => {},
            AbstractSyntaxControlType::ControlReference => {
                self.append_stream_at_path();
                self.end_scope_node();
            }
            other => self.end_other_node(*other),
        }
        self.control = AbstractSyntaxControlType::Unknown;
        EndNodeAction::Continue
    }   

    fn token_error(&mut self, _error: &AbstractSyntaxTokenError) {
    }
}