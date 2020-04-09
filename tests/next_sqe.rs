#[test]
fn next_sqe() {
    const ENTRIES: u32 = 2;
    let (mut sq, _) = uring::IoUring::with_entries(2).setup().unwrap();

    for _ in 0..ENTRIES {
        assert!(sq.next_sqe().is_some())
    }

    assert!(sq.next_sqe().is_none())
}
