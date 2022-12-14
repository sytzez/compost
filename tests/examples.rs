use compost::run::run_file;

#[test]
fn test_automatic_definitions() {
    assert_eq!(
        run_file("examples/automatic_definitions.compost"),
        "BottomRight of A: 30, 15. Width and Height of B: 5, 10",
    )
}

#[test]
fn test_class_inheritance() {
    assert_eq!(run_file("examples/class_inheritance.compost"), "100")
}

#[test]
fn test_classes() {
    assert_eq!(
        run_file("examples/classes.compost"),
        "There is no way to output this point",
    )
}

#[test]
fn test_functions_and_constants() {
    assert_eq!(run_file("examples/functions_and_constants.compost"), "52")
}

#[test]
fn test_traits_and_definitions() {
    assert_eq!(
        run_file("examples/traits_and_definitions.compost"),
        "-1, -2",
    )
}

#[test]
fn test_types() {
    assert_eq!(
        run_file("examples/types.compost"),
        "Hello, Bob. Hello, Fifi. Bob (20). Fifi (3)",
    )
}

#[test]
fn test_linked_list() {
    assert_eq!(
        run_file("examples/linked_list.compost"),
        "1, 2, 3, 4 (total: 10). Reversed: 4, 3, 2, 1 (total: 10). Together: 1, 2, 3, 4, 4, 3, 2, 1 (total: 20)",
    )
}

#[test]
fn test_multiple_inheritance() {
    assert_eq!(
        run_file("examples/multiple_inheritance.compost"),
        "Child of Perry (species: Platypus)",
    )
}

#[test]
fn test_if() {
    assert_eq!(run_file("examples/if.compost"), "Yes")
}

#[test]
fn test_binary_tree() {
    assert_eq!(run_file("examples/binary_tree.compost"), "3 -1 2")
}