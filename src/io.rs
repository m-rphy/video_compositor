use memmap2::Mmap;
use std::fs::File;
use std::io::{self, Cursor};
use byteorder::{BigEndian, ReadBytesExt};

pub struct MappedVideo {
    pub width:  u32,
    pub height: u32,
    frame_count: u32,
    mmap: Mmap,                 // owns the bytes; nothing borrows from it
}

impl MappedVideo {
    /// Read metadata and keep a copy of the mmap.
    pub fn new(mmap: Mmap) -> io::Result<Self> {
        let mut cur = Cursor::new(&mmap[..]);

        let width       = cur.read_u32::<BigEndian>()?;
        let height      = cur.read_u32::<BigEndian>()?;
        let frame_count = cur.read_u32::<BigEndian>()?;

        Ok(Self { width, height, frame_count, mmap })
    }

    /// Total number of frames.
    #[inline]
    pub fn frames(&self) -> u32 { self.frame_count }

    /// Borrow frame *i* (zero‑based) as a slice.
    #[inline]
    pub fn frame(&self, i: u32) -> &[u8] {
        assert!(i < self.frame_count, "frame index out of bounds");

        let frame_size = (self.width * self.height * 3) as usize;
        let header_len = 12;
        let offset     = header_len + i as usize * frame_size;
        &self.mmap[offset .. offset + frame_size]
    }
}

pub fn read_raw_video_mmap(path: &str) -> io::Result<MappedVideo> {
    let file = File::open(path)?;
    // SAFETY: read‑only map of a file that stays open for the life of the
    // `MappedVideo`.
    let mmap = unsafe { Mmap::map(&file)? };
    MappedVideo::new(mmap)
}

