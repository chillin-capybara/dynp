use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use crate::property::Property;

/// Dynamic collection of properties.
#[derive(Debug)]
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
    /// # Returns
    /// - `Some(&Property<T>)` if the property of type `T` exists and its type matches `T`.
    /// - `None` if the property is not found or the property assigned to the TypeId of `T` does not
    ///  match `T`.
    #[inline]
    fn get_property<T: Any>(&self) -> Option<&Property<T>> {
        // Get the TypeId of the specified type T
        let type_id = TypeId::of::<T>();

        // Attempt to find the property by its TypeId
        if let Some(property) = self.properties.get(&type_id) {
            // Attempt to downcast the property to the specified type T
            property
                .downcast_ref::<Property<T>>()
        } else {
            // Property not found
            None
        }
    }

    /// Retrieves a mutable reference to a property of a specific type from the PropertyCollection.
    ///
    /// # Returns
    /// - `Some(&mut Property<T>)` if the property of type `T` exists and its type matches `T`.
    /// - `None` if the property is not found or the property assigned to the TypeId of `T` does not
    ///  match `T`.
    #[inline]
    fn get_property_mut<T: Any>(&mut self) -> Option<&mut Property<T>> {
        let type_id = TypeId::of::<T>();
        if let Some(property) = self.properties.get_mut(&type_id) {
            property
                .downcast_mut::<Property<T>>()
        } else {
            None
        }
    }

    /// Retrieves the value of a property of a specific type from the PropertyCollection.
    ///
    /// # Type Parameters
    /// - `T`: The type of the property to retrieve. It should be a type that implements the `Any`
    /// 
    /// # Returns
    /// - `Some(&T)` if the property of type `T` exists.
    /// - `None` if the property is not found.
    pub fn get<T: Any>(&self) -> Option<&T> {
        if let Some(property) = self.get_property::<T>() {
            property.get()
        } else {
            None
        }
    }

    /// Assigns the value to a property or adds a new property with the specified value.
    ///
    /// This function assigns the provided `value` to an existing property of the specified type `T`
    /// if it already exists in the collection. If no property of that type exists, a new property of
    /// that type is added to the collection with the specified `value`.
    ///
    /// # Type Parameters
    /// - `T`: The type of the property to assign. It should be a type that implements the `Any`
    /// 
    /// # Parameters
    /// - `value`: The value of type `T` to assign to the property.
    pub fn assign<T: Any>(&mut self, value: T)
    {
        if let Some(property) = self.get_property_mut::<T>() {
            // if the property was found, just assign the new value
            property.assign(value);
        } else {
            // if the property was not found, add it to the collection
            self.properties.insert(
                TypeId::of::<T>(),
                Box::new(Property::new(value))
            );
        }
    }

    /// Checks if a property of a specified type exists in the collection.
    ///
    /// # Returns
    /// * `true` - If a property of the specified type `T` exists in the collection.
    /// * `false` - If no property of the specified type `T` is found in the collection.
    pub fn contains<T: Any>(&self) -> bool {
        self.get_property::<T>().is_some()
    }

    /// Subscribes to changes in a property of a specific type in the PropertyCollection.
    ///
    /// This function attempts to subscribe to changes in a property of the specified type `T` in the
    /// `PropertyCollection`. If the property exists and its type matches `T`, it registers the provided
    /// callback function to be called when the property's value changes. If the property does not
    /// exist in the collection yet, an *early subscription* is performed by adding the callback to
    /// ab empty property of type `T`. This allows the callback to be called when the property is
    /// assigned a value later.
    ///
    ///
    /// # Arguments
    /// * `callback` - The callback function to be called when the property's value changes.
    ///
    /// # Example
    ///
    /// ```
    /// use dynp::PropertyCollection;
    ///
    /// // define a custom property using the Newtype pattern
    /// #[derive(Copy, Clone, Debug)]
    /// struct CustomProperty(i32);
    ///
    /// fn main() {
    ///     // create a new property collection
    ///     let mut collection = PropertyCollection::new();
    ///     collection.subscribe::<CustomProperty>(|value: &CustomProperty| {
    ///         println!("Property changed: {:?}", value);
    ///     });
    ///
    ///     // assign a new property
    ///     collection.assign(CustomProperty(42));
    /// }
    /// ```
    pub fn subscribe<T: Any>(
        &mut self,
        callback: impl FnMut(&T) + 'static
    ) {
        if let Some(property) = self.get_property_mut::<T>() {
            // if the property exists, add the callback to its subscriptions
            property.subscribe(Box::new(callback));
        } else {
            // if the property does not exist, add an empty property and add the callback to its
            // subscriptions
            let mut property: Property<T> = Property::empty();
            property.subscribe(Box::new(callback));
            self.properties.insert(
                TypeId::of::<T>(),
                Box::new(property)
            );
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

        assert_eq!(collection.get::<i32>(), Some(&42));
        assert_eq!(collection.get::<u32>(), None);
        assert_eq!(collection.get::<String>(), None);


        collection.assign(11);
        assert_eq!(collection.get::<i32>(), Some(&11));
        assert_eq!(collection.get::<u32>(), None);
        assert_eq!(collection.get::<String>(), None);

        // make sure the reference does not interfere with the original property
        collection.assign(&99);
        assert_eq!(collection.get::<i32>(), Some(&11));
        assert_eq!(collection.get::<&i32>(), Some(&&99));
        assert_eq!(collection.get::<u32>(), None);
        assert_eq!(collection.get::<String>(), None);
    }

    #[test]
    fn test_assign_get_property_newtype() {
        let mut collection = PropertyCollection::new();
        
        collection.assign(MyInt(42));
        assert_eq!(collection.get::<MyInt>(), Some(&MyInt(42)));
        assert_eq!(collection.get::<i32>(), None);
        assert_eq!(collection.get::<u32>(), None);
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

    #[test]
    fn test_early_subscribe() {
        let mut collection = PropertyCollection::new();

        let count = std::sync::Arc::new(std::sync::Mutex::new(0));
        let count_clone = std::sync::Arc::clone(&count);

        // create an early subscription
        collection.subscribe::<i32>(move |value: &i32| {
            assert_eq!(*value, 11);
            let mut count = count_clone.lock().unwrap();
            *count += 1;
        });

        // perform an assignment
        collection.assign::<i32>(11);
        assert_eq!(*count.lock().unwrap(), 1);

        // perform a new assignment
        collection.assign::<i32>(11);
        assert_eq!(*count.lock().unwrap(), 2);
    }

    #[test]
    fn test_subscribe() {
        let mut collection = PropertyCollection::new();

        let count = std::sync::Arc::new(std::sync::Mutex::new(0));
        let count_clone = std::sync::Arc::clone(&count);

        // perform an assignment
        collection.assign::<i32>(11);
        assert_eq!(*count.lock().unwrap(), 0);

        // create an early subscription
        collection.subscribe::<i32>(move |value: &i32| {
            assert_eq!(*value, 11);
            let mut count = count_clone.lock().unwrap();
            *count += 1;
        });

        assert_eq!(*count.lock().unwrap(), 0);

        // perform an assignment
        collection.assign::<i32>(11);
        assert_eq!(*count.lock().unwrap(), 1);

        // perform an assignment
        collection.assign::<i32>(11);
        assert_eq!(*count.lock().unwrap(), 2);
    }

}
