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

    let mut it = arr.iter_mut();
    let i = it.next().unwrap();
    i.1.push('#');
    // big no no. not possible under normal circumstances
    it.arr[[0, 0, 0]].push('$');
    println!("{}", i.1);

    arr[[2, 2, 2]] = String::from("bye");
    for (i, c) in arr {
        if i == [2, 2, 2] {
            assert_eq!(c, "bye");
        }
    }
}