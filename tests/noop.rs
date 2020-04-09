#[test]
fn noop() {
    const DATA: u64 = 0xDEADBEEF;

    let (mut sq, mut cq) = uring::IoUring {
        sq_entries: 2,
        ..Default::default()
    }
    .setup()
    .unwrap();

    let sqe = sq.next_sqe().unwrap();
    sqe.user_data = DATA;
    sq.submit_sqe();

    cq.wait_for_cqe().unwrap();

    let cqe = cq.next_cqe().unwrap();
    assert_eq!(cqe.user_data, DATA);

    assert!(cq.next_cqe().is_none())
}
