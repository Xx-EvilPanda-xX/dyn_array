use super::*;

#[test]
fn iterate() {
    let mut arr = DynArray::new([5, 5, 5], String::from("hi"));
    arr[[1, 1, 1]] = String::from("bye");
    for (i, c) in &arr {
        if i == [1, 1, 1] {
            assert_eq!(c, "bye");
        }
    }

    arr[[2, 2, 2]] = String::from("bye");
    for (i, c) in arr {
        if i == [2, 2, 2] {
            assert_eq!(c, "bye");
        }
    }
}