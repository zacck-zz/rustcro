//pub for macro
#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    // outer curly braces is for macro rules
    // inner curly braces is the block the macro expands to
    ($element:expr) => {{
        let mut vs = Vec::new();
        vs.push($element);
        vs
    }};
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
