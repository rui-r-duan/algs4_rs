use super::linkedbag::LinkedBag;
use super::resizingbag::ResizingBag;
use std::collections::HashSet;

#[test]
fn linked_bag_of_str() {
    let mut bag = LinkedBag::new();
    let list = [
        "to", "be", "or", "not", "to", "-", "be", "-", "-", "that", "-", "-", "-", "is",
    ];
    for s in list {
        bag.add(s);
    }

    assert_eq!(bag.len(), 14);
    assert_eq!(
        bag.iter().map(|s| *s).collect::<HashSet<&str>>(),
        HashSet::from(list)
    );

    // test clone
    let bag2 = bag.clone();
    assert_eq!(bag2.len(), bag.len());
    assert_eq!(
        bag2.iter().map(|s| *s).collect::<HashSet<&str>>(),
        HashSet::from(list)
    );
}

#[test]
fn resizing_bag_of_str() {
    let mut bag = ResizingBag::new();
    let list = [
        "to", "be", "or", "not", "to", "-", "be", "-", "-", "that", "-", "-", "-", "is",
    ];
    for s in list {
        bag.add(s);
    }

    assert_eq!(bag.len(), 14);
    assert_eq!(
        bag.iter().map(|s| *s).collect::<HashSet<&str>>(),
        HashSet::from(list)
    );

    // test clone
    let bag2 = bag.clone();
    assert_eq!(bag2.len(), bag.len());
    assert_eq!(
        bag2.iter().map(|s| *s).collect::<HashSet<&str>>(),
        HashSet::from(list)
    );
}
