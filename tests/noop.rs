use uring::Params;

#[test]
fn noop() {
    const DATA: u64 = 0xDEADBEEF;

    let (mut sq, cq) = uring::setup(Params {
        sq_entries: 2,
        ..Default::default()
    })
    .unwrap();

    let sqe = sq.next_sqe().unwrap();
    sqe.user_data = DATA;
    sq.submit_sqe();

    cq.wait_for_cqes(1, 1).unwrap();

    let cqe = cq.next_cqe().unwrap();
    assert_eq!(cqe.user_data, DATA)
}
