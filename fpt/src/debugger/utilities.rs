use std::fs::File;
use std::io::{Result, Write};

/// Writes a Gameboy frame to a PGM file
pub fn write_pgm_screenshot(frame: &crate::ppu::Frame, filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;

    // Write the header for a 160x144 PGM image with 4 shades of gray
    write!(file, "P2\n# Game Boy screenshot: {filename}\n160 144\n3\n")?;

    // Our Game Boy's framebuffer seems to have a direct correspondence to this!
    for line in frame.array_chunks::<160>() {
        let pgm_line = line
            .iter()
            .map(|p| (b'3' - *p) as char) // ASCII from '0' to '3'
            .intersperse(' ')
            .collect::<String>()
            + "\n";

        file.write_all(pgm_line.as_bytes())?;
    }

    Ok(())
}
