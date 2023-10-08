# dynp

[![codecov](https://codecov.io/gh/chillin-capybara/dynp/graph/badge.svg?token=P49P373ZGE)](https://codecov.io/gh/chillin-capybara/dynp)

A dynamic, type-safe property system that emphasizes the use of the Newtype pattern.

## 💡 Inspiration

This library was ispired by the [`iZotope/glassproperties`](https://github.com/iZotope/glassproperties) C++ library; big credits to the amazing authors of that library.

## 📦 Features

- [X] Property collection
- [X] Property assignment
- [X] Property retrieval
- [X] Property subscriptions (with callbacks / closures)

## 👨‍💻 Usage

The following snipped should give you a basic idea what this library is about.

```rust
use dynp::PropertyCollection;

// define a custom property using the Newtype pattern
#[derive(Copy, Clone, Debug)]
struct CustomProperty(i32);

fn main() {
    // create a new property collection
    let mut collection = PropertyCollection::new();

    // assign a new property
    collection.assign(CustomProperty(42));

    // get the property
    match collection.get::<CustomProperty>() {
       Some(prop) => {
          println!("Property: {:?}", prop);
        },
        None => {
            println!("Property does not exist");
        }
    };
}
```

## 🚧 Roadmap

- [ ] Property deserialization
- [ ] Performance tests and improvements
- [ ] Improved documentation

