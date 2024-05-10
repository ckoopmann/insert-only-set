# Insert Only Set

`insert_only_set` is a procedural macro to generates thread-safe insert-only sets from enums in Rust.
Under the hood this set is a struct with a `OnceLock` field for each enum variant. 

## Features

- Automatically generates an insert-only set for any enum
- Thread-safe insertions with `OnceLock`
- Iterates over set variants that have been inserted

## Example

```rust 
use insert_only_set::InsertOnlySet;

#[derive(InsertOnlySet, Debug, PartialEq)]
pub enum Type {
    Customer,
    Employee,
}

fn main() {
    let set = Type::InsertOnlySet();

    assert!(!set.contains(Type::Customer));
    assert!(!set.contains(Type::Employee));

    assert!(set.insert(Type::Customer));
    assert!(set.contains(Type::Customer));
    assert!(!set.contains(Type::Employee));

    assert!(set.insert(Type::Employee));
    assert!(set.contains(Type::Customer));
    assert!(set.contains(Type::Employee));

    // Try to insert again, should return false
    assert!(!set.insert(Type::Customer));

    for variant in set.iter() {
        println!("{:?}", variant);
    }
}
```


