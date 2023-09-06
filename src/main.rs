use byteorder::{LittleEndian, ReadBytesExt};
use image::ImageBuffer;
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

pub const MAGIC_ID_SIZE: usize = 11;
pub const LANGUAGE_SIZE: usize = 64;

fn get_file_header(reader: &mut dyn Read) -> io::Result<()> {
    let mut magic_id = String::new();
    for _ in 0..MAGIC_ID_SIZE {
        let c = reader.read_u8()?.into();
        magic_id.push(c);
    }

    if magic_id != *"FO3SAVEGAME" {
        panic!("Invalid save file");
    }

    let _header_size = reader.read_u32::<LittleEndian>()?;
    let _header_mystery = reader.read_u32::<LittleEndian>()?;
    reader.read_u8()?;

    let mut language = String::new();
    for _ in 0..LANGUAGE_SIZE {
        let c = reader.read_u8()?;
        if c != 0x00 {
            language.push(c as char);
        }
    }
    reader.read_u8()?;

    let save_img_width = reader.read_u32::<LittleEndian>()?;
    reader.read_u8()?;
    let save_img_height = reader.read_u32::<LittleEndian>()?;
    reader.read_u8()?;

    let save_number = reader.read_u32::<LittleEndian>()?;
    reader.read_u8()?;

    let pc_name_len = reader.read_u16::<LittleEndian>()?;
    reader.read_u8()?;
    let mut pc_name = String::new();
    for _ in 0..pc_name_len {
        let c = reader.read_u8()?.into();
        pc_name.push(c);
    }
    reader.read_u8()?;

    let pc_title_len = reader.read_u16::<LittleEndian>()?;
    reader.read_u8()?;
    let mut pc_title = String::new();
    for _ in 0..pc_title_len {
        let c = reader.read_u8()?.into();
        pc_title.push(c);
    }
    reader.read_u8()?;

    let _pc_level = reader.read_u32::<LittleEndian>()?;
    reader.read_u8()?;

    let pc_location_len = reader.read_u16::<LittleEndian>()?;
    reader.read_u8()?;
    let mut pc_location = String::new();
    for _ in 0..pc_location_len {
        let c = reader.read_u8()?.into();
        pc_location.push(c);
    }
    reader.read_u8()?;

    let playtime_len = reader.read_u16::<LittleEndian>()?;
    reader.read_u8()?;
    let mut playtime = String::new();
    for _ in 0..playtime_len {
        let c = reader.read_u8()?.into();
        playtime.push(c);
    }
    reader.read_u8()?;

    let mut imgbuf = ImageBuffer::new(save_img_width, save_img_height);
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = reader.read_u8()?;
        let g = reader.read_u8()?;
        let b = reader.read_u8()?;
        *pixel = image::Rgb([r, g, b]);
    }
    let file_name = format!(
        "Save_{}_{}_{}_{}.png",
        save_number,
        pc_name.replace(' ', "_"),
        pc_location.replace(' ', "_"),
        playtime.replace("00", "0").replace('.', "_")
    );
    imgbuf.save(file_name).unwrap();

    Ok(())
}

fn main() -> io::Result<()> {
    let file = File::open("test.fos")?;
    let mut reader = BufReader::new(file);

    get_file_header(&mut reader)?;

    Ok(())
}
