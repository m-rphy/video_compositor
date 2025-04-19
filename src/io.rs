use std::fs::File;
use std::io::{self, Read, BufReader};
use byteorder::{ReadBytesExt, BigEndian};

pub struct RawVideo {
    pub width: u32,
    pub height: u32,
    pub frames: Vec<Vec<u8>>, // each frame is width * height * 3 bytes
}

pub fn read_raw_video(path: &str) -> io::Result<RawVideo> {
    let mut reader = BufReader::new(File::open(path)?);

    let width = reader.read_u32::<BigEndian>()?;
    let height = reader.read_u32::<BigEndian>()?;
    let frame_count = reader.read_u32::<BigEndian>()?;

    let frame_size = (width * height * 3) as usize;
    let mut frames = Vec::with_capacity(frame_count as usize);

    for _ in 0..frame_count {
        let mut buf = vec![0u8; frame_size];
        reader.read_exact(&mut buf)?;
        frames.push(buf);
    }

    Ok(RawVideo { width, height, frames })
}

