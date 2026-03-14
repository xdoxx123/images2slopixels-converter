use std::{fs::File, io::Write, path::PathBuf};

use anyhow::Ok;
use clap::{Parser};
use image::{DynamicImage, GenericImageView, ImageReader};
fn rgbtohex(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
#[derive(clap::Parser)]
struct Args {
    #[arg(long,short)]
    pub path:PathBuf
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let path = args.path;
    let mut file = File::create("poo.sif")?;
    file.write_all(b"SLUDGE")?;
    let convert = ImageReader::open(path)?.decode()?.to_rgb8();
    let mut realdeal = DynamicImage::ImageRgb8(convert);
    realdeal = realdeal.resize(realdeal.width().clamp(1, 255), realdeal.height().clamp(1, 255), image::imageops::FilterType::Nearest);
    file.write_all(&[realdeal.width() as u8])?;
    file.write_all(&[realdeal.height() as u8])?;
    for (_x, _y, pixel) in realdeal.pixels() {
        let hex = rgbtohex(pixel.0[0], pixel.0[1], pixel.0[2]);
        let bytes = [
            ((hex >> 16) & 0xFF) as u8, 
            ((hex >> 8) & 0xFF) as u8,  
            (hex & 0xFF) as u8,         
        ];
        file.write_all(&bytes)?;
    }
    file.flush()?;
    println!("wrote data?");

    Ok(())
}
