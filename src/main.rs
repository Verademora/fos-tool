use fos_tool::FalloutNVHeader;
use std::{
    fs::File,
    io::{self, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("test.fos")?;
    let mut reader = BufReader::new(file);

    let file_header = FalloutNVHeader::read_from(&mut reader)?;
    dbg!(file_header.save_img_size());
    file_header.extract_image(&mut reader)?;

    Ok(())
}
