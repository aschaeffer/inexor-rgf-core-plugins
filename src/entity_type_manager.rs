use crate::model::{EntityType, Extension, PropertyType};

#[derive(Debug)]
pub enum EntityTypeCreationError {
    Failed,
}

pub trait EntityTypeManager: Send + Sync {
    /// Returns all entity types.
    fn get_entity_types(&self) -> Vec<EntityType>;

    /// Returns true, if a entity type with the given name exists.
    fn has(&self, name: String) -> bool;

    /// Returns the entity type with the given name or empty.
    fn get(&self, name: String) -> Option<EntityType>;

    /// Returns all entity types whose names matches the given search string.
    fn find(&self, search: String) -> Vec<EntityType>;

    /// Creates a new entity type.
    fn create(&self, name: String, group: String, components: Vec<String>, behaviours: Vec<String>, properties: Vec<PropertyType>, extensions: Vec<Extension>);

    /// Deletes the entity type with the given name.
    fn delete(&self, name: String);

    /// Imports an entity type from a JSON file file located at the given path.
    fn import(&self, path: String);

    /// Exports the entity type with the given name to a JSON file located at the given path.
    fn export(&self, name: String, path: String);
}
