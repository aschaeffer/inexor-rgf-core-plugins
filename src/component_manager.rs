use crate::model::{Component, PropertyType};

#[derive(Debug)]
pub enum ComponentCreationError {
    Failed,
}

pub trait ComponentManager: Send + Sync {
    /// Returns all components
    fn get_components(&self) -> Vec<Component>;

    /// Returns true, if a component with the given name exists.
    fn has(&self, name: String) -> bool;

    /// Returns the component with the given name or empty.
    fn get(&self, name: String) -> Option<Component>;

    /// Returns all components whose names matches the given search string.
    fn find(&self, search: String) -> Vec<Component>;

    /// Creates a new component with the given name and the given properties.
    fn create(&self, name: String, properties: Vec<PropertyType>);

    /// Deletes the component with the given name.
    fn delete(&self, name: String);

    /// Imports a component from a JSON file located at the given path.
    fn import(&self, path: String);

    /// Exports the component with the given name to a JSON file located at the given path.
    fn export(&self, name: String, path: String);
}
