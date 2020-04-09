use std::fs::File;
use std::io::{self, IoSliceMut};
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use uring::Params;

#[test]
fn read_file() -> io::Result<()> {
    const TEXT: &[u8] = b"Hello World!";

    let (mut sq, mut cq) = uring::setup(Params {
        sq_entries: 2,
        ..Default::default()
    })
    .unwrap();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("file.txt");

    let f = File::open(&path)?;
    let mut buf = [0; 4096];
    let mut bufs = [IoSliceMut::new(&mut buf)];

    let sqe = sq.next_sqe().unwrap();
    sqe.prep_read_vectored(f.as_raw_fd(), &mut bufs);
    sq.submit_sqe();

    cq.wait_for_cqe().unwrap();
    let cqe = cq.next_cqe().unwrap();
    assert_eq!(TEXT, &buf[..cqe.res as usize]);

    Ok(())
}
