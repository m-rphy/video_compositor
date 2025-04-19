use std::fs::File;
use std::io::{BufWriter, Write};
use byteorder::{WriteBytesExt, BigEndian};

pub fn write_raw_video(
    path: &str,
    width: u32,
    height: u32,
    frames: &[Vec<u8>],
) -> std::io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);

    writer.write_u32::<BigEndian>(width)?;
    writer.write_u32::<BigEndian>(height)?;
    writer.write_u32::<BigEndian>(frames.len() as u32)?;

    for frame in frames {
        writer.write_all(frame)?;
    }

    Ok(())
}

