use super::linkedqueue::LinkedQueue;
use super::resizingqueue::ResizingQueue;
use super::svecque::SVecQue;

#[test]
fn linked_queue_of_str() {
    let mut qu: LinkedQueue<&str> = LinkedQueue::new();
    assert_eq!(qu.iter().collect::<Vec<&&str>>().len(), 0);
    let list = [
        "to", "be", "or", "not", "to", "-", "be", "-", "-", "that", "-", "-", "-", "is",
    ];
    let mut popped = Vec::new();
    for item in list {
        if item != "-" {
            qu.enqueue(item);
        } else if !qu.is_empty() {
            popped.push(qu.dequeue().unwrap());
        }
    }
    let popped_str = popped.join(" ");
    let queue_len = format!("({} left on queue)", qu.len());
    let output = format!("{} {}", popped_str, queue_len);
    assert_eq!(output, "to be or not to be (2 left on queue)");
    assert_eq!(qu.to_string(), "that is ");

    // test clone
    let qu2 = qu.clone();
    assert_eq!(qu2.to_string(), "that is ");

    // Drop should be good, no memory issue.
}

#[test]
fn linked_queue_drop() {
    let mut qu: LinkedQueue<String> = LinkedQueue::new();
    let list: Vec<String> = [
        "to", "be", "or", "not", "to", "-", "be", "-", "-", "that", "-", "-", "-", "is",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();
    for item in list {
        qu.enqueue(item);
    }
    // Drop should be good, no memory issue.
}

// The following function should compile.
//
// LinkedQueue<T> should be covariant over T.  It means:
//
//   U is a subtype of T
//     ==>
//   LinkedQueue<U> is a subtype of LinkedQueue<T>
//
//   &'long is a subtype of &'short
//     ==>
//   LinkedQueue<&'long> is a subtype of LinkedQueue<&'short>
//
// For more information, see `example/subtyping_variance.rs`.
#[test]
fn linked_queue_variance() {
    fn _two_refs<'short, 'long: 'short>(a: LinkedQueue<&'short str>, b: LinkedQueue<&'long str>) {
        _take_two(a, b);
    }
    fn _take_two<T>(_val1: T, _val2: T) {}

    fn _bar<'a>() {
        let s: LinkedQueue<&'static str> = LinkedQueue::new();
        let _t: LinkedQueue<&'a str> = s;
    }
}

#[test]
fn resizing_queue_of_str() {
    let mut qu = ResizingQueue::new();
    assert_eq!(qu.iter().collect::<Vec<&&str>>().len(), 0);
    let list = [
        "to", "be", "or", "not", "to", "-", "be", "-", "-", "that", "-", "-", "-", "is",
    ];
    let mut popped = Vec::new();
    for item in list {
        if item != "-" {
            qu.enqueue(item);
        } else if !qu.is_empty() {
            popped.push(qu.dequeue().unwrap());
        }
    }
    let popped_str = popped.join(" ");
    let queue_len = format!("({} left on queue)", qu.len());
    let output = format!("{} {}", popped_str, queue_len);
    assert_eq!(output, "to be or not to be (2 left on queue)");
    assert_eq!(qu.to_string(), "that is ");

    // test clone
    let qu2 = qu.clone();
    assert_eq!(qu2.to_string(), "that is ");
}

#[test]
fn svecque_of_str() {
    let mut qu = SVecQue::new();
    assert_eq!(qu.iter().collect::<Vec<&&str>>().len(), 0);
    let list = [
        "to", "be", "or", "not", "to", "-", "be", "-", "-", "that", "-", "-", "-", "is",
    ];
    let mut popped = Vec::new();
    for item in list {
        if item != "-" {
            qu.enqueue(item);
        } else if !qu.is_empty() {
            popped.push(qu.dequeue().unwrap());
        }
    }
    let popped_str = popped.join(" ");
    let queue_len = format!("({} left on queue)", qu.len());
    let output = format!("{} {}", popped_str, queue_len);
    assert_eq!(output, "to be or not to be (2 left on queue)");
    assert_eq!(qu.to_string(), "that is ");

    // test clone
    let qu2 = qu.clone();
    assert_eq!(qu2.to_string(), "that is ");
}
