use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, ErrorKind, Read};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct FalloutNVHeader {
    language: String,
    save_img_width: u32,
    save_img_height: u32,
    save_number: u32,
    pc_name: String,
    pc_title: String,
    pc_level: u32,
    pc_location: String,
    playtime: String,
}

pub const MAGIC_ID_SIZE: u32 = 11;
pub const LANGUAGE_SIZE: u32 = 64;
const HEADER_SEPERATOR_COUNT: u32 = 14;

fn read_string(size: u32, reader: &mut dyn Read) -> io::Result<String> {
    let mut s = String::new();
    for _ in 0..size {
        let c = reader.read_u8()?;
        if c != 0x00 {
            s.push(c as char);
        }
    }

    Ok(s)
}

impl FalloutNVHeader {
    pub fn save_img_size(&self) -> (u32, u32) {
        (self.save_img_width, self.save_img_height)
    }

    pub fn header_size(&self) -> u32 {
        MAGIC_ID_SIZE + LANGUAGE_SIZE
    }

    pub fn read_from(reader: &mut dyn Read) -> Result<Self, io::Error> {
        let magic_id = read_string(MAGIC_ID_SIZE, reader)?;

        if magic_id != *"FO3SAVEGAME" {
            let e = io::Error::new(ErrorKind::InvalidInput, "File Header error");
            return Err(e);
        }

        let _header_size = reader.read_u32::<LittleEndian>()?;
        let _header_mystery = reader.read_u32::<LittleEndian>()?;
        reader.read_u8()?;

        let language = read_string(LANGUAGE_SIZE, reader)?;
        reader.read_u8()?;

        let save_img_width = reader.read_u32::<LittleEndian>()?;
        reader.read_u8()?;
        let save_img_height = reader.read_u32::<LittleEndian>()?;
        reader.read_u8()?;

        let save_number = reader.read_u32::<LittleEndian>()?;
        reader.read_u8()?;

        let pc_name_len = reader.read_u16::<LittleEndian>()?.into();
        reader.read_u8()?;
        let pc_name = read_string(pc_name_len, reader)?;
        reader.read_u8()?;

        let pc_title_len = reader.read_u16::<LittleEndian>()?.into();
        reader.read_u8()?;
        let pc_title = read_string(pc_title_len, reader)?;
        reader.read_u8()?;

        let pc_level = reader.read_u32::<LittleEndian>()?;
        reader.read_u8()?;

        let pc_location_len = reader.read_u16::<LittleEndian>()?.into();
        reader.read_u8()?;
        let pc_location = read_string(pc_location_len, reader)?;
        reader.read_u8()?;

        let playtime_len = reader.read_u16::<LittleEndian>()?.into();
        reader.read_u8()?;
        let playtime = read_string(playtime_len, reader)?;
        reader.read_u8()?;

        Ok(Self{
                language,
                save_img_width,
                save_img_height,
                save_number,
                pc_name,
                pc_title,
                pc_level,
                pc_location,
                playtime,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
