use compost::run::run_file;

#[test]
fn test_automatic_definitions() {
    let result = run_file("examples/automatic_definitions.compost");
    assert_eq!(result, "BottomRight of A: 30, 15. Width and Height of B: 5, 10")
}

#[test]
fn test_class_inheritance() {
    let result = run_file("examples/class_inheritance.compost");
    assert_eq!(result, "100")
}

#[test]
fn test_classes() {
    let result = run_file("examples/classes.compost");
    assert_eq!(result, "There is no way to output this point")
}

#[test]
fn test_functions_and_constants() {
    let result = run_file("examples/functions_and_constants.compost");
    assert_eq!(result, "52")
}

#[test]
fn test_traits_and_definitions() {
    let result = run_file("examples/traits_and_definitions.compost");
    assert_eq!(result, "-1, -2")
}