use std::io::{self, IoSlice, IoSliceMut};
use std::os::unix::io::AsRawFd;

#[test]
fn read_write_file() -> io::Result<()> {
    const TEXT: &[u8] = b"Hello World!";

    let (mut sq, mut cq) = uring::IoUring::with_entries(2).setup()?;

    let f = tempfile::tempfile()?;

    {
        let write_bufs = [IoSlice::new(&TEXT[..])];

        let sqe = sq.next_sqe().unwrap();
        sqe.prep_write_vectored(f.as_raw_fd(), &write_bufs);
        sq.submit_sqe();

        cq.wait_for_cqe()?;
        let cqe = cq.next_cqe().unwrap();
        assert_eq!(cqe.res, TEXT.len() as i32);
    }

    {
        let mut read_buf = [0; 128];
        let mut read_bufs = [IoSliceMut::new(&mut read_buf)];

        let sqe = sq.next_sqe().unwrap();
        sqe.prep_read_vectored(f.as_raw_fd(), &mut read_bufs);
        sq.submit_sqe();

        cq.wait_for_cqe()?;
        let cqe = cq.next_cqe().unwrap();
        assert_eq!(&read_buf[..cqe.res as usize], TEXT);
    }

    Ok(())
}
