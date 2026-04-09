use anyhow::Ok;
use clap::Parser;
use image::GenericImage;
use image::Rgba;
use image::{DynamicImage, GenericImageView, ImageReader};
use std::io::Read;
use std::{fs::File, io::Write, path::PathBuf};
fn rgbtohex(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}
#[derive(clap::Subcommand)]
enum Commands {
    Encode {
        #[arg(long, short)]
        path: PathBuf,
        #[arg(long, short, default_value_t = 255)]
        width: u8,
        #[arg(long, default_value_t = 255)]
        height: u8,
    },
    Decode {
        #[arg(long, short)]
        path: PathBuf,
    },
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Encode {
            path,
            width,
            height,
        } => {
            let path = path;
            let mut file = File::create("out.sif")?;
            file.write_all(b"SLUDGE")?;
            let convert = ImageReader::open(path)?.decode()?.to_rgb8();
            let mut realdeal = DynamicImage::ImageRgb8(convert);
            realdeal = realdeal.resize(
                realdeal.width().clamp(1, width as u32),
                realdeal.height().clamp(1, height as u32),
                image::imageops::FilterType::Nearest,
            );
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
        }
        Commands::Decode { path } => {
            let mut file = File::open(path.clone())?;
            let mut magic: [u8; 6] = [0; 6];
            file.read_exact(&mut magic)?;
            if &magic != b"SLUDGE" {
                println!("not sif");
                return Ok(());
            }
            let mut metadata: [u8; 2] = [0; 2];
            file.read_exact(&mut metadata)?;
            println!("{:#?}", metadata);
            let mut pixeldata = vec![0u8; metadata[0] as usize * metadata[1] as usize * 3];
            file.read_exact(&mut pixeldata)?;
            let mut dynimage = DynamicImage::new_rgb8(metadata[0] as u32, metadata[1] as u32);
            for x in 0..metadata[0] {
                for y in 0..metadata[1] {
                    let index = (y as usize * metadata[0] as usize + x as usize) * 3 as usize;
                    let r = pixeldata[index];
                    let g = pixeldata[index + 1];
                    let b = pixeldata[index + 2];
                    //println!("{},{},{}", r, g, b);
                    dynimage.put_pixel(x as u32, y as u32, Rgba([r, g, b, 255]));
                }
            }
            let output = File::create("out.png")?;
            dynimage.write_to(
                &mut std::io::BufWriter::new(output),
                image::ImageFormat::Png,
            )?;
        }
    }
    Ok(())
}
