use super::linkedbag::LinkedBag;
use super::vecbag::VecBag;
use std::collections::HashSet;

#[test]
fn linked_bag_of_str() {
    let mut bag = LinkedBag::new();
    let list = ["to", "be", "or", "not", "to", "-", "be", "-", "-", "that", "-", "-", "-", "is"];
    for s in list {
        bag.add(s);
    }

    assert_eq!(bag.len(), 14);
    assert_eq!(bag.iter().map(|s| *s).collect::<HashSet<&str>>(), HashSet::from(list));
}

#[test]
fn vec_bag_of_str() {
    let mut bag = VecBag::new();
    let list = ["to", "be", "or", "not", "to", "-", "be", "-", "-", "that", "-", "-", "-", "is"];
    for s in list {
        bag.add(s);
    }

    assert_eq!(bag.len(), 14);
    assert_eq!(bag.iter().map(|s| *s).collect::<HashSet<&str>>(), HashSet::from(list));
}
