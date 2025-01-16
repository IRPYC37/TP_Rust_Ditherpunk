use argh::FromArgs;
use image::{RgbImage, Rgb};
use image::io::Reader as ImageReader;

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {
    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(option)]
    output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {
    /// couleur pour les pixels sombres (format R,G,B)
    #[argh(option, default = "String::from(\"0,0,0\")")]
    dark_color: String,

    /// couleur pour les pixels clairs (format R,G,B)
    #[argh(option, default = "String::from(\"255,255,255\")")]
    light_color: String,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {
    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize,
}

fn distance(c1: (u8, u8, u8), c2: (u8, u8, u8)) -> f64 {
    let (r1, g1, b1) = c1;
    let (r2, g2, b2) = c2;
    (((r1 as f64 - r2 as f64).powi(2) + (g1 as f64 - g2 as f64).powi(2) + (b1 as f64 - b2 as f64).powi(2)) as f64).sqrt()
}

fn apply_palette(image: &RgbImage, palette: &[(u8, u8, u8)]) -> RgbImage {
    let mut new_image = RgbImage::new(image.width(), image.height());

    for (x, y, pixel) in image.enumerate_pixels() {
        let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
        let closest_color = palette.iter()
            .min_by(|&&c1, &&c2| {
                distance((r, g, b), c1)
                    .partial_cmp(&distance((r, g, b), c2))
                    .unwrap()
            })
            .unwrap_or(&(0, 0, 0)); // Default to black if palette is empty

        new_image.put_pixel(x, y, Rgb([closest_color.0, closest_color.1, closest_color.2]));
    }

    new_image
}

fn parse_color(color: &str) -> Rgb<u8> {
    let parts: Vec<u8> = color.split(',')
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    Rgb([parts[0], parts[1], parts[2]])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: DitherArgs = argh::from_env();

    let img = ImageReader::open(&args.input)?.decode()?;
    let rgb_image = img.to_rgb8();

    match args.mode {
        Mode::Seuil(opts) => {

            // Passer un pixel sur deux en blanc
            let mut new_image = rgb_image.clone();
            for (x, y, pixel) in new_image.enumerate_pixels_mut() {
                if (x + y) % 2 == 0 {
                    *pixel = Rgb([255, 255, 255]);
                }
            }
            new_image.save("./image/Question7.png")?;

            // Seuillage en monochrome avec couleurs personnalisées
            let dark_color = parse_color(&opts.dark_color);
            let light_color = parse_color(&opts.light_color);
            let mut threshold_image = rgb_image.clone();
            for pixel in threshold_image.pixels_mut() {
                let luminance = 0.299 * pixel[0] as f64 + 0.587 * pixel[1] as f64 + 0.114 * pixel[2] as f64;
                if luminance > 128.0 {
                    *pixel = light_color;
                } else {
                    *pixel = dark_color;
                }
            }
            threshold_image.save("./image/Question8.png")?;
        },
        Mode::Palette(opts) => {
            let palette = vec![
                (0, 0, 0), (255, 255, 255), (255, 0, 0), (0, 255, 0),
                (0, 0, 255), (255, 255, 0), (0, 255, 255), (255, 0, 255)
            ];
            let new_image = apply_palette(&rgb_image, &palette[..opts.n_couleurs]);
            if let Some(output) = args.output {
                new_image.save(output)?;
            } else {
                new_image.save("./image/Question10.png")?;
            }
        }
    }

    Ok(())
}