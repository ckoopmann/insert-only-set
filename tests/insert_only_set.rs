use insert_only_set::GenerateInsertOnlySet;

#[derive(GenerateInsertOnlySet, Debug, PartialEq)]
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
        let set = TypeInsertOnlySet::new();

        assert!(!set.contains(Type::Customer));
        assert!(!set.contains(Type::Employee));

        assert!(set.insert(Type::Customer));
        assert!(set.contains(Type::Customer));
        assert!(!set.contains(Type::Employee));

        assert!(set.insert(Type::Employee));
        assert!(set.contains(Type::Customer));
        assert!(set.contains(Type::Employee));
    }

    #[test]
    fn test_insert_once() {
        let set = TypeInsertOnlySet::new();

        assert!(set.insert(Type::Customer));
        assert!(!set.insert(Type::Customer));
    }

    #[test]
    fn test_iter() {
        let set = TypeInsertOnlySet::new();

        assert!(set.iter().collect::<Vec<_>>().is_empty());

        set.insert(Type::Customer);
        let mut expected = vec![Type::Customer];
        assert_eq!(set.iter().collect::<Vec<_>>(), expected);

        set.insert(Type::Employee);
        expected.push(Type::Employee);
        assert_eq!(set.iter().collect::<Vec<_>>(), expected);
    }
}
