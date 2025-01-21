# TP_Rust_Ditherpunk

## Réponses aux questions et explications :

  ### Question 2 - Pour ouvrir une image depuis un fichier, on utilise ``` ImageReader::open("myimage.png")?.decode()?; ``` On obtient un DynamicImage, à quoi correspond ce type? Comment obtenir une image en mode rbg8 à partir de ce DynamicImage?

  Le type DynamicImage est un enum fournie que la bibliothèque image fournit et qui sert a représenter une image. Cela permet d' encapsuler différents types d’images en fonction :

  - De leurs nombre de canaux (RGB, RGBA, ...).
  - De leurs type de données (u8, u16, ...).

  Avec cette bibliothèque, on peut gérer plusieurs formats d'images sans avoir à spécifier le type exact au moment de la lecture.

  ### Question 3 - Sauver l’image obtenue au format png. Que se passe-t-il si l’image de départ avait un canal alpha?

  Pour sauvegarder l'image, on utilise : 
  ```rgb_image.save("image.png")?;```

  Si l'image d'origine contient un canal alpha (transparence), celui-ci est perdu lorsque l'image est convertie en rgb8, car rgb8 ne conserve que 3 canaux (Rouge, Vert, Bleu), alors qu'une image avec transparence utilise 4 canaux (Rouge, Vert, Bleu, Alpha).

Par conséquent, les zones transparentes de l'image d'origine seront remplacées par du noir ou une couleur arbitraire dans l'image en rgb8. Donc toute information liée à la transparence est irrémédiablement perdue lors de la conversion.

### Question 4 - Afficher dans le terminal la couleur du pixel (32, 52) d’une image

Pour afficher la couleur du pixel (32,52) d'une image, nous avons fait le code rust suivant :

```rust
    // Charger l'image
    let image = ImageReader::open("image.png")?.decode()?;
    // Met l'image en rgb8 pour avoir avec des pixels RGB
    let rgb_image = image.to_rgb8();
    // Get les couleurs du pixel à la position (32, 52)
    let pixel = rgb_image.get_pixel(32, 52);
    // Afficher (R, G, B)
    println!("La couleur du pixel (32, 52) est : R={}, G={}, B={}", pixel[0], pixel[1], pixel[2]);
    Ok(())
```
Avec notre image de test, le résultat dans le terminal est :
```bash
La couleur du pixel (32, 52) est : R=227, G=227, B=208
```

### Question 5 -Passer un pixel sur deux d’une image en blanc. Est-ce que l’image obtenue est reconnaissable ?

Pour passer un pixel sur deux d'une image en blanc et garder la couleur original pour les autres, nous avons fait le code rust suivant : 

```rust
// Charge l'image
let image = ImageReader::open("image.png")?.decode()?;
//
// Convertir l'image en mode rgb8
let mut rgb_image = image.to_rgb8();
//
// Obtenir les dimensions de l'image
let (width, height) = rgb_image.dimensions();
//
// Parcours les pixels de l'image
for y in 0..height {
    for x in 0..width {
        if (x + y) % 2 == 0 {
            // Passage des piexls en blanc
            rgb_image.put_pixel(x, y, image::Rgb([255, 255, 255]));
        }
    }
}
//
// Sauvegarder l'image modifiée
rgb_image.save("output_Q5.png")?;
//
println!("image sauvegardée (output_Q5.png).");
Ok(())
```

Enfin pour répondre à la question, "Est-ce que l’image obtenue est reconnaissable ?", oui elle l'est toujours, l'image semble juste plus "claire"

**Image d'origine :**
![image](/ditherpunk/image/image.png)

**Image en sortie :**
![image](/ditherpunk/image/output_Q5.png)


### Question 7 : Passage en monochrome par seuillage

cargo run -- image/iut.jpg seuil --dark-color 0,0,0 --light-color 255,255,255
**Image d'origine :**
![image](/ditherpunk/image/iut.jpg)

**Image en sortie :**
![image](/ditherpunk/image/Question8.png)

### Question 9 : Comment calculer la distance entre deux couleurs ?

La distance entre deux couleurs peut être mesurée de différentes manières. Une méthode couramment utilisée pour comparer les couleurs est la distance Euclidienne dans l'espace des couleurs RVB (Rouge, Vert, Bleu).

La formule pour la distance Euclidienne entre deux couleurs C1=(R1,G1,B1)C1​=(R1​,G1​,B1​) et C2=(R2,G2,B2)C2​=(R2​,G2​,B2​) est la suivante :

d(C1,C2)=Racine_carré((R1−R2)²+(G1−G2)²+(B1−B2)²)

Cela donne une mesure de la "distance" en termes de différence de teinte, de saturation et de luminosité entre deux couleurs. Plus cette distance est faible, plus les couleurs sont similaires.

### Question 10: Commande à exécuter


cargo run -- image/iut.jpg palette --n-couleurs 8

**Image d'origine :**
![image](/ditherpunk/image/iut.jpg)

**Image en sortie :**
![image](/ditherpunk/image/Question10.png)


### Question 11: Votre application doit se comporter correctement si on donne une palette vide.

Si la palette est vide, chaque pixel est remplacé par la couleur noire par défaut. Cela garantit que l'application ne plante pas et produit une sortie cohérente.

### Question 12 - Implémenter le tramage aléatoire des images.

Pour faire un tramage aléatoire sur une image, voici le code rust que nous avons utiliser :

```rust
// Charger l'image
let image = ImageReader::open("image/image.png")?.decode()?;
//
// image en mode rgb8
let mut rgb_image = image.to_rgb8();
//
// Obtenir les dimensions de l'image
let (width, height) = rgb_image.dimensions();
//
// Initialiser un générateur de nombres aléatoires
let mut rng = rand::thread_rng();
//
// fait le tramage aléatoire
for y in 0..height {
    for x in 0..width {
        let pixel = rgb_image.get_pixel(x, y);
//
        // calculer la luminosité (moyenne des canaux R, G, B)
        let lumi = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) as f64 / 3.0 / 255.0;
//
        // aléatoire entre 0 et 1
        let rdm_seuil: f64 = rng.gen();
//
        if lumi > rdm_seuil {
            // en blanc
            rgb_image.put_pixel(x, y, image::Rgb([255, 255, 255]));
        } else {
            // en noir
            rgb_image.put_pixel(x, y, image::Rgb([0, 0, 0]));
        }
    }
}
//
// image modifiée
rgb_image.save("image/output_Q12.png")?;
//
println!("Image sauvegardée (image/output_Q12.png).");
Ok(())
```

Pour appliquer un **dither aléatoire**, on utilise la commande suivante :

```bash
cargo run -- image/iut.jpg randdither
```

**Image d'origine :**
![image](/ditherpunk/image/iut.jpg)

**Image en sortie :**
![image](/ditherpunk/image/output_Q12.png)


### Question 15 - Implementation de la matrice de Bayer

cargo run -- image/iut.jpg seuil --bayer-order 3
**Image d'origine :**
![image](/ditherpunk/image/iut.jpg)

**Image en sortie :**
![image](/ditherpunk/image/Question15.png)

## Liens utiles :

  Inspiration du TP : https://surma.dev/things/ditherpunk/
