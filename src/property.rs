use std::any::Any;

/// Property callback function type.
type PropertyCallback<T> = Box<dyn FnMut(&T)>;

/// Represents a property that can hold a value of a generic type `T` and allows subscribing to
/// changes in its value.
///
/// A `Property` contains the following components:
///
/// - `value`: The current value of the property, which can be of any type that implements the
/// `Any` trait.
/// - `subscriptions`: A list of callbacks (closures) that are invoked whenever the property's
/// value is updated. Subscribers can register these callbacks to react to changes.
pub struct Property<T: Any>
{
    /// Value of the property.
    value: Option<T>,
    /// List of subscribers to the property.
    subscriptions: Vec<PropertyCallback<T>>,
}

impl<T: Any> Property<T>
{
    /// Creates a new property with the given value.
    pub fn new(value: T) -> Self
    {
        Self {
            value: Some(value),
            subscriptions: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            value: None,
            subscriptions: Vec::new(),
        }
    }

    /// Returns a reference to the value of the property.
    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn assign(&mut self, value: T)
    {
        self.value = Some(value);

        for callback in self.subscriptions.iter_mut() {
            // the value can be unwrapped safely because we just assigned it above
            callback(self.value.as_ref().unwrap());
        }
    }

    /// Subscribes to changes in the property by registering a callback function.
    ///
    /// When you subscribe to a property, you provide a callback function that will be invoked
    /// whenever the property's value changes. The callback function should accept a reference to
    /// the property's value (of type `&T`) as its parameter.
    ///
    /// # Parameters
    ///
    /// - `callback`: A boxed closure (callback function) that takes a reference to the property's
    ///   value as a parameter. This closure will be executed whenever the property's value changes.
    pub fn subscribe(&mut self, callback: Box<dyn FnMut(&T)>) {
        self.subscriptions.push(callback);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use super::*;

    #[test]
    fn test_empty_property_i32() {
        let prop = Property::<i32>::empty();
        assert_eq!(prop.get(), None);
    }

    #[test]
    fn test_new_property_i32() {
        let prop = Property::new(42);
        assert_eq!(prop.get(), Some(42).as_ref());
    }

    #[test]
    fn test_assign_property_i32() {
        let mut prop = Property::new(42);
        assert_eq!(prop.get(), Some(42).as_ref());

        prop.assign(99);
        assert_eq!(prop.get(), Some(99).as_ref());

        prop.assign(11);
        assert_eq!(prop.get(), Some(11).as_ref());
    }

    #[test]
    fn test_subscribe_property_i32() {
        let mut prop = Property::new(42);
        let count = Arc::new(Mutex::new(0));

        let count_clone = Arc::clone(&count);
        prop.subscribe(Box::new(move |val| {
            assert_eq!(*val, 11);
            let mut count = count_clone.lock().unwrap();
            *count += 1;
        }));

        prop.assign(11);
        prop.assign(11);

        // the subscription should have been called twice
        assert_eq!(*count.lock().unwrap(), 2);
    }

    // test cases for the Newtype pattern

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct MyInt(i32);
    #[test]
    fn test_empty_property_newtype() {
        let prop = Property::<MyInt>::empty();
        assert_eq!(prop.get(), None);
    }

    #[test]
    fn test_new_property_newtype() {
        let prop = Property::new(MyInt(42));
        assert_eq!(prop.get(), Some(MyInt(42)).as_ref());
    }

    #[test]
    fn test_assign_property_newtype() {
        let mut prop = Property::new(MyInt(42));
        assert_eq!(prop.get(), Some(MyInt(42)).as_ref());

        prop.assign(MyInt(99));
        assert_eq!(prop.get(), Some(MyInt(99)).as_ref());

        prop.assign(MyInt(11));
        assert_eq!(prop.get(), Some(MyInt(11)).as_ref());
    }

    #[test]
    fn test_subscribe_property_newtype() {
        let mut prop = Property::new(MyInt(42));
        let count = Arc::new(Mutex::new(0));

        let count_clone = Arc::clone(&count);
        prop.subscribe(Box::new(move |val| {
            assert_eq!(*val, MyInt(11));
            let mut count = count_clone.lock().unwrap();
            *count += 1;
        }));

        prop.assign(MyInt(11));
        prop.assign(MyInt(11));
        prop.assign(MyInt(11));

        // the subscription should have been called twice
        assert_eq!(*count.lock().unwrap(), 3);
    }
}
