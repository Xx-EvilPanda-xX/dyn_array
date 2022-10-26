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

    for (i, c) in &mut arr {
        if i == [1, 1, 1] {
            assert_eq!(c, "bye");
            *c = String::from("hello");
        }
    }
    assert_eq!(arr[[1, 1, 1]], String::from("hello"));

    arr[[2, 2, 2]] = String::from("bye");
    for (i, c) in arr {
        if i == [2, 2, 2] {
            assert_eq!(c, "bye");
        }
    }
}

#[test]
fn index() {
    let mut index = [4, 3, 4];
    let dims = [5, 5, 5];
    next_index(&mut index, &dims);
    assert_eq!(index, [0, 4, 4]);
    next_index(&mut index, &dims);
    assert_eq!(index, [1, 4, 4]);
    next_index(&mut index, &dims);
    assert_eq!(index, [2, 4, 4]);
    next_index(&mut index, &dims);
    assert_eq!(index, [3, 4, 4]);
    next_index(&mut index, &dims);
    assert_eq!(index, [4, 4, 4]);
    next_index(&mut index, &dims);
    assert_eq!(index, [0, 0, 5]);
}