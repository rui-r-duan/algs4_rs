use super::linkedstack::LinkedStack;

#[test]
fn linked_stack_of_str() {
    let mut st = LinkedStack::new();
    let list = [
        "to", "be", "or", "not", "to", "-", "be", "-", "-", "that", "-", "-", "-", "is",
    ];
    let mut popped = Vec::new();
    for item in list {
        if item != "-" {
            st.push(item);
        } else if !st.is_empty() {
            popped.push(st.pop().unwrap());
        }
    }
    let popped_str = popped.join(" ");
    let stack_len = format!("({} left on stack)", st.len());
    let output = format!("{} {}", popped_str, stack_len);
    assert_eq!(output, "to be not that or be (2 left on stack)");
}
