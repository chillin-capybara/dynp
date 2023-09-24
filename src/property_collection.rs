use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use thiserror::Error;
use crate::property::Property;

// TODO: complete the documentation

#[derive(Error, Debug)]
pub enum PropertyCollectionError {
    #[error("Property not found in the collection.")]
    PropertyNotFound,
    #[error("Property type mismatch.")]
    PropertyTypeMismatch,
}

#[derive(Default, Debug)]
pub struct PropertyCollection {
    properties: HashMap<TypeId, Box<dyn Any>>,
}

impl PropertyCollection {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// Retrieves a reference to a property of a specific type from the PropertyCollection.
    ///
    /// This function attempts to retrieve a property of the specified type `T` from the
    /// `PropertyCollection`. If the property exists and its type matches `T`, a reference to
    /// that property is returned. If the property does not exist or its type does not match `T`,
    /// an error is returned.
    ///
    /// # Returns
    ///
    /// - `Ok(&Property<T>)` if the property of type `T` exists and its type matches `T`.
    /// - `Err(PropertyCollectionError::PropertyNotFound)` if the property is not found.
    /// - `Err(PropertyCollectionError::PropertyTypeMismatch)` if the property's type does not match `T`.
    #[inline]
    fn get_property<T: Any>(&self) -> Result<&Property<T>, PropertyCollectionError> {
        // Get the TypeId of the specified type T
        let type_id = TypeId::of::<T>();

        // Attempt to find the property by its TypeId
        if let Some(property) = self.properties.get(&type_id) {
            // Attempt to downcast the property to the specified type T
            property
                .downcast_ref::<Property<T>>()
                .ok_or(PropertyCollectionError::PropertyTypeMismatch)
        } else {
            // Property not found
            Err(PropertyCollectionError::PropertyNotFound)
        }
    }

    /// Retrieves a mutable reference to a property of a specific type from the PropertyCollection.
    ///
    /// This function attempts to retrieve a property of the specified type `T` from the
    /// `PropertyCollection`. If the property exists and its type matches `T`, a reference to
    /// that property is returned. If the property does not exist or its type does not match `T`,
    /// an error is returned.
    ///
    /// # Returns
    ///
    /// - `Ok(&mut Property<T>)` if the property of type `T` exists and its type matches `T`.
    /// - `Err(PropertyCollectionError::PropertyNotFound)` if the property is not found.
    /// - `Err(PropertyCollectionError::PropertyTypeMismatch)` if the property's type does not match `T`.
    #[inline]
    fn get_property_mut<T: Any>(&mut self) -> Result<&mut Property<T>,
        PropertyCollectionError> {
        let type_id = TypeId::of::<T>();
        if let Some(property) = self.properties.get_mut(&type_id) {
            property
                .downcast_mut::<Property<T>>()
                .ok_or(PropertyCollectionError::PropertyTypeMismatch)
        } else {
            Err(PropertyCollectionError::PropertyNotFound)
        }
    }

    /// Retrieves the value of a property of a specific type from the PropertyCollection.
    ///
    /// This function attempts to retrieve the value of a property of the specified type `T` from the
    /// `PropertyCollection`. If the property exists and its type matches `T`, a reference to its value
    /// is returned.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the `PropertyCollection` from which to retrieve the property value.
    ///
    /// # Returns
    ///
    /// - `Ok(&T)` if the property of type `T` exists and its type matches `T`.
    /// - `Err(PropertyCollectionError::PropertyNotFound)` if the property is not found.
    /// - `Err(PropertyCollectionError::PropertyTypeMismatch)` if the property's type does not match `T`.
    pub fn get<T: Any>(&self) -> Result<&T, PropertyCollectionError> {
        // Attempt to get the property of type T
        let property: &Property<T> = self.get_property::<T>()?;
        if let Some(value) = property.get() {
            // Return a reference to the property's value
            Ok(value)
        } else {
            // Property has no value (it's empty)
            Err(PropertyCollectionError::PropertyNotFound)
        }
    }

    /// Assigns the value to a property or adds a new property with the specified value.
    ///
    /// This function assigns the provided `value` to an existing property of the specified type `T`
    /// if it already exists in the collection. If no property of that type exists, a new property of
    /// that type is added to the collection with the specified `value`.
    ///
    /// # Parameters
    ///
    /// - `value`: A reference to the value to assign or add to the property collection. The value
    ///   should be of a type that implements the `Any` trait and can be cloned using `ToOwned`.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type of property to assign or add. It should be a type that implements the `Any`
    ///   trait and can be cloned using `ToOwned`.
    pub fn assign<U, T: 'static>(&mut self, value: U)
        where
            U: ToOwned<Owned = T>,
    {
        let property = self.get_property_mut::<T>();
        match property {
            Ok(property) => {
                // if the property exists, set its value
                property.assign(value);
            }
            Err(err) => {
                match err {
                    // if the property doesn't exist, add it
                    PropertyCollectionError::PropertyNotFound => {
                        self.properties.insert(
                            TypeId::of::<T>(),
                            Box::new(Property::new(value))
                        );
                    }
                    // if the property type doesn't match, return the error
                    _ => panic!("Unexpected error, when assigning a property!")
                }
            }
        }
    }

    /// Checks if a property of a specified type exists in the collection.
    ///
    /// This function determines whether a property of the specified type `T` exists in the property
    /// collection. It returns `true` if a property of the given type is found in the collection, and
    /// `false` otherwise.
    ///
    /// # Returns
    ///
    /// - `true`: If a property of the specified type `T` exists in the collection.
    /// - `false`: If no property of the specified type `T` is found in the collection.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type of property to check for. It should be a type that implements the `Any` trait
    ///   and can be cloned using `ToOwned`.
    pub fn contains<T: Any + ToOwned<Owned = T>>(&self) -> bool {
        self.properties.contains_key(&TypeId::of::<T>())
    }

    /// Subscribes to changes in a property of a specific type in the PropertyCollection.
    ///
    /// This function attempts to subscribe to changes in a property of the specified type `T` in the
    /// `PropertyCollection`. If the property exists and its type matches `T`, it registers the provided
    /// callback function to be called when the property's value changes.
    ///
    /// # Arguments
    /// * `callback` - A boxed closure (function) that takes a reference to the property's value (`&T`) as an argument.
    ///
    /// # Returns
    /// - `Ok(())` if the property of type `T` exists and its type matches `T`, and the subscription is set successfully.
    /// - `Err(PropertyCollectionError::PropertyNotFound)` if the property is not found.
    /// - `Err(PropertyCollectionError::PropertyTypeMismatch)` if the property's type does not match `T`.
    pub fn subscribe<T: Any>(
        &mut self,
        callback: impl FnMut(&T) + 'static
    ) {
        match self.get_property_mut::<T>() {
            Ok(property) => {
                // if the property exists, add the callback to its subscriptions
                property.subscribe(Box::new(callback));
            },
            Err(err) => {
                match err {
                    PropertyCollectionError::PropertyNotFound => {
                        // perform an early subscription by adding the callback to an empty
                        // property's subscriptions
                        let mut property: Property<T> = Property::empty();
                        property.subscribe(Box::new(callback));
                        self.properties.insert(
                            TypeId::of::<T>(),
                            Box::new(property)
                        );
                    },
                    _ => panic!("Unexpected error at property subscription: {:?}", err)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A newtype wrapper for `i32` that implements the `ToOwned` trait.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    struct MyInt(i32);

    #[test]
    fn test_new_property_collection() {
        let collection = PropertyCollection::new();
        assert!(collection.properties.is_empty());
    }

    #[test]
    fn test_assign_get_property_native() {
        let mut collection = PropertyCollection::new();
        collection.assign(42);

        // existing property
        match collection.get::<i32>() {
            Ok(val) => assert_eq!(val, &42),
            Err(_) => panic!("Expected property of type i32"),
        }

        // non-existing property
        match collection.get::<String>() {
            Err(PropertyCollectionError::PropertyNotFound) => (),
            _ => panic!("Expected PropertyNotFound error"),
        }

        // non-existing property
        match collection.get::<u32>() {
            Err(PropertyCollectionError::PropertyNotFound) => (),
            _ => panic!("Expected PropertyNotFound error"),
        }

        collection.assign(11);

        // existing property
        match collection.get::<i32>() {
            Ok(val) => assert_eq!(val, &11),
            Err(_) => panic!("Expected property of type i32"),
        }

        // non-existing property
        match collection.get::<String>() {
            Err(PropertyCollectionError::PropertyNotFound) => (),
            _ => panic!("Expected PropertyNotFound error"),
        }

        // non-existing property
        match collection.get::<u32>() {
            Err(PropertyCollectionError::PropertyNotFound) => (),
            _ => panic!("Expected PropertyNotFound error"),
        }
    }

    #[test]
    fn test_assign_get_property_newtype() {
        let mut collection = PropertyCollection::new();
        collection.assign(MyInt(42));

        // existing property
        match collection.get::<MyInt>() {
            Ok(val) => assert_eq!(val.0, 42),
            Err(_) => panic!("Expected property of type MyInt"),
        }

        // non-existing property
        match collection.get::<i32>() {
            Err(PropertyCollectionError::PropertyNotFound) => (),
            _ => panic!("Expected PropertyNotFound error"),
        }

        match collection.get::<String>() {
            Err(PropertyCollectionError::PropertyNotFound) => (),
            _ => panic!("Expected PropertyNotFound error"),
        }
    }

    #[test]
    fn test_contains_property_native() {
        let mut collection = PropertyCollection::new();
        assert!(!collection.contains::<i32>());

        collection.assign(42);
        collection.assign(&42);
        assert!(collection.contains::<i32>());
        assert!(!collection.contains::<String>());
    }

    #[test]
    fn test_contains_property_newtype() {
        let mut collection = PropertyCollection::new();
        assert!(!collection.contains::<MyInt>());

        collection.assign(MyInt(42));
        assert!(collection.contains::<MyInt>());
        assert!(!collection.contains::<i32>());
        assert!(!collection.contains::<String>());
    }

}
