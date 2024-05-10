use std::sync::OnceLock;
use insert_only_set::GenerateAddOnlySet;

#[derive(GenerateAddOnlySet, Debug, PartialEq)]
pub enum Type {
    Customer,
    Employee,
    // Add new variants here
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_insert_and_contains() {
        let set = TypeAddOnlySet::new();

        assert!(!set.contains(Type::Customer));
        assert!(!set.contains(Type::Employee));

        set.insert(Type::Customer).expect("Failed to insert customer");
        assert!(set.contains(Type::Customer));
        assert!(!set.contains(Type::Employee));

        set.insert(Type::Employee).expect("Failed to insert employee");
        assert!(set.contains(Type::Customer));
        assert!(set.contains(Type::Employee));
    }

    #[test]
    fn test_insert_once() {
        let set = TypeAddOnlySet::new();

        set.insert(Type::Customer).expect("Failed to insert customer");
        let result = set.insert(Type::Customer);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Already set");
    }

    #[test]
    fn test_iter() {
        let set = TypeAddOnlySet::new();

        assert!(set.iter().collect::<Vec<_>>().is_empty());

        set.insert(Type::Customer).expect("Failed to insert customer");
        let mut expected = vec![Type::Customer];
        assert_eq!(set.iter().collect::<Vec<_>>(), expected);

        set.insert(Type::Employee).expect("Failed to insert employee");
        expected.push(Type::Employee);
        let entries = set.iter().collect::<Vec<Type>>();
        assert_eq!(entries, expected);
    }
}
