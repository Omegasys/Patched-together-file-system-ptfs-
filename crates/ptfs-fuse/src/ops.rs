use std::ffi::OsStr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use fuser::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyEntry, ReplyWrite, Request,
};

use libc::{ENOENT};

use ptfs_api::Ptfs;

const TTL: Duration = Duration::from_secs(1);

/// Simple inode layout
const ROOT_INODE: u64 = 1;
const FILE_INODE: u64 = 2;

/// FUSE wrapper
pub struct PtfsFuse {
    fs: Arc<Mutex<Ptfs>>,
}

impl PtfsFuse {
    pub fn new(fs: Arc<Mutex<Ptfs>>) -> Self {
        Self { fs }
    }

    fn root_attr() -> FileAttr {
        FileAttr {
            ino: ROOT_INODE,
            size: 0,
            blocks: 0,
            atime: SystemTime::now(),
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            crtime: SystemTime::now(),
            kind: FileType::Directory,
            perm: 0o755,
            nlink: 2,
            uid: 1000,
            gid: 1000,
            rdev: 0,
            flags: 0,
            blksize: 512,
        }
    }

    fn file_attr(size: u64) -> FileAttr {
        FileAttr {
            ino: FILE_INODE,
            size,
            blocks: 1,
            atime: SystemTime::now(),
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            crtime: SystemTime::now(),
            kind: FileType::RegularFile,
            perm: 0o644,
            nlink: 1,
            uid: 1000,
            gid: 1000,
            rdev: 0,
            flags: 0,
            blksize: 512,
        }
    }
}

impl Filesystem for PtfsFuse {
    fn lookup(
        &mut self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        reply: ReplyEntry,
    ) {
        if parent == ROOT_INODE && name.to_str() == Some("file") {
            let fs = self.fs.lock().unwrap();
            let size = fs.read_object(&1).map(|d| d.len()).unwrap_or(0);

            reply.entry(&TTL, &Self::file_attr(size as u64), 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request<'_>, ino: u64, reply: ReplyAttr) {
        match ino {
            ROOT_INODE => {
                reply.attr(&TTL, &Self::root_attr());
            }
            FILE_INODE => {
                let fs = self.fs.lock().unwrap();
                let size = fs.read_object(&1).map(|d| d.len()).unwrap_or(0);

                reply.attr(&TTL, &Self::file_attr(size as u64));
            }
            _ => reply.error(ENOENT),
        }
    }

    fn read(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        size: u32,
        reply: ReplyData,
    ) {
        if ino != FILE_INODE {
            reply.error(ENOENT);
            return;
        }

        let fs = self.fs.lock().unwrap();

        if let Some(data) = fs.read_object(&1) {
            let start = offset as usize;
            let end = (start + size as usize).min(data.len());

            if start < data.len() {
                reply.data(&data[start..end]);
            } else {
                reply.data(&[]);
            }
        } else {
            reply.error(ENOENT);
        }
    }

    fn write(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        data: &[u8],
        _flags: u32,
        reply: ReplyWrite,
    ) {
        if ino != FILE_INODE {
            reply.error(ENOENT);
            return;
        }

        let mut fs = self.fs.lock().unwrap();

        let mut buffer = fs.read_object(&1).cloned().unwrap_or_default();

        let offset = offset as usize;

        if buffer.len() < offset {
            buffer.resize(offset, 0);
        }

        if buffer.len() < offset + data.len() {
            buffer.resize(offset + data.len(), 0);
        }

        buffer[offset..offset + data.len()].copy_from_slice(data);

        fs.write_object(1, buffer);

        reply.written(data.len() as u32);
    }
}
