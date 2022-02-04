use crate::prelude::*;

#[derive(Debug)]
pub enum ContainerRunError {
    IncorrectAmountOfArgumentsPassed,
    FirstArgumentNotStateVariable,
    PropertyValueError(AbstractSyntaxPropertyValueError)
}

impl From<AbstractSyntaxPropertyValueError> for ContainerRunError {
    fn from(from: AbstractSyntaxPropertyValueError) -> Self {
        Self::PropertyValueError(from)
    }
}