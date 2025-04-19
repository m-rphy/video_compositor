use std::fs::File;
use std::io::{BufWriter, Write};
use byteorder::{WriteBytesExt, BigEndian};

use image::{RgbImage, ImageBuffer};
use std::fs;
use std::path::Path;


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


pub fn write_frames_as_pngs(
    output_dir: &str,
    width: u32,
    height: u32,
    frames: &[Vec<u8>],
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir)?;

    for (i, frame) in frames.iter().enumerate() {
        let mut rgb_data = vec![0u8; frame.len()];
        for j in 0..(frame.len() / 3) {
            let b = frame[j * 3];
            let r = frame[j * 3 + 1];
            let g = frame[j * 3 + 2];

            rgb_data[j * 3]     = r;
            rgb_data[j * 3 + 1] = g;
            rgb_data[j * 3 + 2] = b;
        }

        let img: RgbImage = ImageBuffer::from_raw(width, height, rgb_data)
            .expect("Invalid image buffer");

        let filename = format!("{}/frame_{:05}.png", output_dir, i);
        img.save(Path::new(&filename))?;
    }

    Ok(())
}
