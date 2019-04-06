use trespass::increment;

#[increment]
fn inner(arg: i64) -> i64 {
    arg + 1
}

#[test]
fn works() {
    assert_eq!(inner(1), 3);
}
