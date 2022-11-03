use testing::{splish, sploosh};

#[test]
fn test_splish_sploosh() {
    let result = sploosh(splish(-1, 0), splish(1, 1), splish(3, 2));
    assert_eq!(result, 4);
}
