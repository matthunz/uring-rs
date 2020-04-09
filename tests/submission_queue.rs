use uring::Params;

#[test]
fn next_sqe() {
    const ENTRIES: u32 = 2;
    let (mut sq, _) = uring::setup(Params {
        sq_entries: ENTRIES,
        ..Default::default()
    })
    .unwrap();

    for _ in 0..ENTRIES {
        assert!(sq.next_sqe().is_some())
    }
    assert!(sq.next_sqe().is_none())
}
