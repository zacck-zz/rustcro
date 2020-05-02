//pub for macro
#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    // outer curly braces is for macro rules
    // inner curly braces is the block the macro expands to
    ($($element:expr),+ /* + means one or more exprs separated by a comma*/) => {{
        let mut vs = Vec::new();
        // repeat this operation as many times
        // the same number of times as the pattern that had element in it
        $(vs.push($element);)*
        vs
    }};
}

#[test]
fn double() {
    let x: Vec<u32> = avec![421, 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    //assert eq is a declarative macro and can use any delimiter
    assert_eq![x[0], 421];
    assert_eq![x[1], 43];
}

#[test]
fn single() {
    let x: Vec<u32> = avec![421];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    //assert eq is a declarative macro and can use any delimiter
    assert_eq![x[0], 421];
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty())
}
