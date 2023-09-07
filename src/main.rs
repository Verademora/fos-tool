use std::{
    fs::File,
    io::{self, BufReader},
};


// fn get_file_header(reader: &mut dyn Read) -> io::Result<()> {
//     let mut imgbuf = ImageBuffer::new(save_img_width, save_img_height);
//     for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
//         let r = reader.read_u8()?;
//         let g = reader.read_u8()?;
//         let b = reader.read_u8()?;
//         *pixel = image::Rgb([r, g, b]);
//     }
//     let file_name = format!(
//         "Save_{}_{}_{}_{}.png",
//         save_number,
//         pc_name.replace(' ', "_"),
//         pc_location.replace(' ', "_"),
//         playtime.replace("00", "0").replace('.', "_")
//     );
//     imgbuf.save(file_name).unwrap();
// 
//     Ok(())
// }

fn main() -> io::Result<()> {
    let file = File::open("test4.fos")?;
    let mut reader = BufReader::new(file);

    let thing = fos_tool::FalloutNVHeader::read_from(&mut reader)?;
    dbg!(thing);
    // get_file_header(&mut reader)?;

    Ok(())
}
