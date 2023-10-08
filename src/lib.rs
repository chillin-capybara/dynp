//! Dynamic property system that emphasizes the use of the Newtype pattern.
//!
//! Each property must be represented by a unique type. This allows the compiler to
//! check the type of each property at compile time and provide a type-safe way of using dynamic
//! properties.
//!
//! # Example
//!
//! ```
//! use dynp::PropertyCollection;
//!
//! // define a custom property using the Newtype pattern
//! #[derive(Copy, Clone, Debug)]
//! struct CustomProperty(i32);
//!
//! fn main() {
//!     // create a new property collection
//!     let mut collection = PropertyCollection::new();
//!
//!     // assign a new property
//!     collection.assign(CustomProperty(42));
//!
//!     // get the property
//!     match collection.get::<CustomProperty>() {
//!        Some(prop) => {
//!           println!("Property: {:?}", prop);
//!         },
//!         None => {
//!             println!("Property does not exist");
//!         }
//!     };
//! }
//! ```
//!

mod property_collection;
mod property;

pub use property_collection::PropertyCollection;