use std::time::{Duration, UNIX_EPOCH};
use libc::ENOENT;

const TTL: Duration = Duration::from_secs(1); // 1 second

use fuser::{
    FileAttr, FileType, Filesystem, MountOption,
};

struct DemoFS;

const ROOT_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: 512,
    padding: 0,
};

impl Filesystem for DemoFS {

    fn getattr(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        println!("getattr ino={ino}");
        match ino {
            1 => reply.attr(&TTL, &ROOT_DIR_ATTR),
            _ => reply.error(ENOENT),
        }
    }

    fn readdir(
            &mut self,
            _req: &fuser::Request<'_>,
            ino: u64,
            _fh: u64,
            offset: i64,
            mut reply: fuser::ReplyDirectory,
        ) {
        println!("readdir ino={ino}");

        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
        ];

        for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
            // i + 1 means the index of the next entry
            if reply.add(entry.0, (i + 1) as i64, entry.1, entry.2) {
                break;
            }
        }
        reply.ok();
    }
}

fn main() {
    let mountpoint = "foo";
    let mut options = vec![MountOption::RO, MountOption::FSName("foo".to_string())];
    options.push(MountOption::AutoUnmount);
    fuser::mount2(DemoFS, mountpoint, &options).unwrap();
}
