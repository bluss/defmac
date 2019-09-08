
#[test]
fn full_path() {
    defmac::defmac! { len x => x.len() }

    assert_eq!(len!(&[1, 2]), 2);
}
