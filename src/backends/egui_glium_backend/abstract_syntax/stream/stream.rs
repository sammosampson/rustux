use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct AbstractSyntaxTokenStream(Vec<AbstractSyntaxTokenResult>, bool);

impl AbstractSyntaxTokenStream { 
    pub fn append_stream(&mut self, control_stream: &mut AbstractSyntaxTokenStream) {
        self.0.append(&mut control_stream.0);
    }

    pub fn add_error(&mut self, error: AbstractSyntaxTokenError) {
        self.0.push(Err(error));
    }

    pub fn start_node(&mut self, node_type: AbstractSyntaxControlType) {
        println!("start {:?}", node_type);
        if node_type == AbstractSyntaxControlType::Root {
            self.1 = true;
        }
        self.0.push(Ok(AbstractSyntaxToken::StartControl(node_type)));

    }

    pub fn property(&mut self, property: AbstractSyntaxProperty) {
        println!("{:?}", property);
        self.0.push(Ok(AbstractSyntaxToken::Property(property)));
    }

    pub fn property_error(&mut self, error: AbstractSyntaxTokenError) {
        self.0.push(Err(error));
    }

    pub fn end_node(&mut self, node_type: AbstractSyntaxControlType) {
        println!("end {:?}", node_type);
        self.0.push(Ok(AbstractSyntaxToken::EndControl(node_type)));
    }

    pub fn contains_root(&self) -> bool {
        self.1
    }

    pub fn accept(&self, visitor: &mut impl AbstractSyntaxTokenStreamVisitor, context: &mut DataContext) {
        for position in 0..self.0.len() {
            self.accept_node(position, visitor, context);
        }
    }

    fn accept_node(&self, position: usize, visitor: &mut impl AbstractSyntaxTokenStreamVisitor, context: &mut DataContext) {
        let node_result = &self.0[position];

        match node_result {
            Ok(node) => match node {
                AbstractSyntaxToken::StartControl(node_type) => visitor.start_node_with_repeat_possibility(position, node_type, context),
                AbstractSyntaxToken::Property(property) => visitor.property(property, context),
                AbstractSyntaxToken::EndControl(node_type) =>
                    if let Some(range) = visitor.end_node_with_repeat_check(position, node_type, context) {
                        for child_position in RangeInclusive::<usize>::from(&range) {
                            self.accept_node(child_position, visitor, context);
                        }
                    },
            },
            Err(error) => visitor.token_error(error),
        }
    }
}