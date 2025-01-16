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
![image](/ditherpunk/image.png)

**Image en sortie :**
![image](/ditherpunk/output_Q5.png)

## Liens utiles :

  Inspiration du TP : https://surma.dev/things/ditherpunk/
