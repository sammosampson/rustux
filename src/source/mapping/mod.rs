use crate::prelude::*;

pub struct Mapped;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Relationship {
    pub parent: Option<Entity>,
    pub next_sibling: Option<Entity>,
    pub first_child: Option<Entity>,
    pub last_child: Option<Entity>
}

impl Relationship {
    pub fn for_parent_only(parent: Entity) -> Self {
        Self {
            parent: Some(parent),
            next_sibling: None,
            first_child: None,
            last_child: None 
        }
    }
    
    pub fn without_children(&self) -> Self {
        Self {
            parent: self.parent,
            next_sibling: self.next_sibling,
            first_child: None,
            last_child: None 
        }
    }
}

pub type RelationshipMap = HashMap<Entity, Relationship>;

pub fn create_relationship_map() -> RelationshipMap {
    RelationshipMap::new()
}

pub trait ParentRetrieval {
    fn get_parent(&self, entity: &Entity) -> Option<Entity>;
}

pub trait ChildrenRetrieval {
    fn get_children(&self, entity: &Entity) -> ChildrenRelationshipIterator;
}

impl ParentRetrieval for RelationshipMap {
    fn get_parent(&self, entity: &Entity) -> Option<Entity> {
        if let Some(relationship) = self.get(entity) {
            return relationship.parent;
        }
        None
    }
}

impl ChildrenRetrieval for RelationshipMap {
    fn get_children(&self, parent: &Entity) -> ChildrenRelationshipIterator {
        ChildrenRelationshipIterator::new(self, *parent)
    }
}

pub struct ChildrenRelationshipIterator<'a> {
    map: &'a RelationshipMap,
    parent: Entity,
    current_child: Option<Entity>
}

impl<'a> ChildrenRelationshipIterator<'a> {
    pub fn new(map: &'a RelationshipMap, parent: Entity) -> Self {
        Self { map, parent, current_child: None }
    }
}

impl <'a> Iterator for ChildrenRelationshipIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        if let Some(child) = self.current_child {
            if let Some(current_child_relationship) = self.map.get(&child) {
                self.current_child = current_child_relationship.next_sibling;
                return current_child_relationship.next_sibling
            } 
        } else {
            if let Some(parent_relationship) = self.map.get(&self.parent) {
                self.current_child = parent_relationship.first_child;
                return parent_relationship.first_child
            } 
        }
        None
    }
}