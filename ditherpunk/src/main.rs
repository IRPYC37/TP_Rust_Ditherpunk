use argh::FromArgs;
use image::{io::Reader as ImageReader};
use rand::Rng; // Pour générer des nombres aléatoires


/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
#[derive(Debug, Clone, PartialEq, FromArgs)]
struct DitherArgs {
    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    /// Mode de rendu monochrome par seuillage.
    Seuil(OptsSeuil),
    /// Mode de réduction à une palette de couleurs.
    Palette(OptsPalette),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs.
struct OptsPalette {
    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Question 4 :
    //
    // Charger l'image depuis un fichier
    //let image = ImageReader::open("image.png")?.decode()?;
    //
    // Convertir l'image en mode rgb8 pour travailler avec des pixels RGB
    //let rgb_image = image.to_rgb8();
    //
    // Obtenir la couleur du pixel à la position (32, 52)
    //let pixel = rgb_image.get_pixel(32, 52);
    //
    // Afficher la couleur du pixel (R, G, B)
    //println!("La couleur du pixel (32, 52) est : R={}, G={}, B={}", pixel[0], pixel[1], pixel[2]);
    //
    //Ok(())


    // Question 5 :
    //
    // Charge l'image
    //let image = ImageReader::open("image.png")?.decode()?;
    //
    // Convertir l'image en mode rgb8
    //let mut rgb_image = image.to_rgb8();
    //
    // Obtenir les dimensions de l'image
    //let (width, height) = rgb_image.dimensions();
    //
    // Parcours les pixels de l'image
    //for y in 0..height {
    //    for x in 0..width {
    //        if (x + y) % 2 == 0 {
    //            // Passage des piexls en blanc
    //            rgb_image.put_pixel(x, y, image::Rgb([255, 255, 255]));
    //        }
    //    }
    //}
    //
    // Sauvegarder l'image modifiée
    //rgb_image.save("output_Q5.png")?;
    //
    //println!("image sauvegardée (output_Q5.png).");
    //Ok(())

    //Question 12
    //
    // Charger l'image
    //let image = ImageReader::open("image.png")?.decode()?;
    //
    // image en mode rgb8
    //let mut rgb_image = image.to_rgb8();
    //
    // Obtenir les dimensions de l'image
    //let (width, height) = rgb_image.dimensions();
    //
    // Initialiser un générateur de nombres aléatoires
    //let mut rng = rand::thread_rng();
    //
    // fait le tramage aléatoire
    //for y in 0..height {
    //    for x in 0..width {
    //       let pixel = rgb_image.get_pixel(x, y);
    //
            // calculer la luminosité (moyenne des canaux R, G, B)
    //        let lumi = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) as f64 / 3.0 / 255.0;
    //
            // aléatoire entre 0 et 1
    //        let rdm_seuil: f64 = rng.gen();
    //
    //        if lumi > rdm_seuil {
                // en blanc
    //            rgb_image.put_pixel(x, y, image::Rgb([255, 255, 255]));
    //        } else {
                // en noir
    //            rgb_image.put_pixel(x, y, image::Rgb([0, 0, 0]));
    //        }
    //    }
    //}
    //
    // image modifiée
    //rgb_image.save("output_Q12.png")?;
    //
    //println!("Image sauvegardée (output_Q12.png).");
    //Ok(())

}
