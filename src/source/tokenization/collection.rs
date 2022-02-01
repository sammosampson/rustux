use crate::prelude::*;

#[derive(Debug)]
pub enum SpecificCollectionError {
    NotEnoughItems(usize),
    WrongType
}

pub fn collect_array_unsigned_shorts(from: &Vec<ArrayTokenResult>, amount: usize) -> Result<Vec::<u16>, SpecificCollectionError> {
    let mut collected = vec!();
    if from.len() != amount {
        return Err(SpecificCollectionError::NotEnoughItems(from.len()));
    }
    for token in from {
        if let Ok(SourceTokenPropertyValue::UnsignedInt(value)) = token {
            collected.push(*value as u16)
        } else {
            return Err(SpecificCollectionError::WrongType)
        }
    }
    Ok(collected)
}

pub fn collect_array_floats(from: &Vec<ArrayTokenResult>, amount: usize) -> Result<Vec::<f32>, SpecificCollectionError> {
    let mut collected = vec!();
    if from.len() != amount {
        return Err(SpecificCollectionError::NotEnoughItems(from.len()));
    }
    for token in from {
        if let Ok(SourceTokenPropertyValue::Float(value)) = token {
            collected.push(*value as f32)
        } else {
            return Err(SpecificCollectionError::WrongType)
        }
    }
    Ok(collected)
}

pub fn collect_array_usizes(from: &Vec<ArrayTokenResult>, amount: usize) -> Result<Vec::<usize>, SpecificCollectionError> {
    let mut collected = vec!();
    if from.len() != amount {
        return Err(SpecificCollectionError::NotEnoughItems(from.len()));
    }
    for token in from {
        if let Ok(SourceTokenPropertyValue::UnsignedInt(value)) = token {
            collected.push(*value as usize)
        } else {
            return Err(SpecificCollectionError::WrongType)
        }
    }
    Ok(collected)
}

pub fn collect_properties_usize(from: &Vec<SourceTokenPropertyValue>, position: usize) -> Result<usize, SpecificCollectionError> {
    if from.len() < position + 1 {
        return Err(SpecificCollectionError::NotEnoughItems(from.len()));
    }

    if let SourceTokenPropertyValue::UnsignedInt(value) = from[position] {
        return Ok(value as usize);
    }
    
    Err(SpecificCollectionError::WrongType)
}