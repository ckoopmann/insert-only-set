use insert_only_set::InsertOnlySet;

#[derive(InsertOnlySet, Debug, PartialEq)]
pub enum Person {
    Customer,
    Employee,
    // Add new variants here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
        let set = Person::InsertOnlySet();

        assert!(!set.contains(Person::Customer));
        assert!(!set.contains(Person::Employee));

        assert!(set.insert(Person::Customer));
        assert!(set.contains(Person::Customer));
        assert!(!set.contains(Person::Employee));

        assert!(set.insert(Person::Employee));
        assert!(set.contains(Person::Customer));
        assert!(set.contains(Person::Employee));
    }

    #[test]
    fn test_insert_once() {
        let set = Person::InsertOnlySet();

        assert!(set.insert(Person::Customer));
        assert!(!set.insert(Person::Customer));
    }

    #[test]
    fn test_iter() {
        let set = Person::InsertOnlySet();

        assert!(set.iter().collect::<Vec<_>>().is_empty());

        set.insert(Person::Customer);
        let mut expected = vec![Person::Customer];
        assert_eq!(set.iter().collect::<Vec<_>>(), expected);

        set.insert(Person::Employee);
        expected.push(Person::Employee);
        assert_eq!(set.iter().collect::<Vec<_>>(), expected);
    }
}
