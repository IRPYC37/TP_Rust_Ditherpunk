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

## Liens utiles :

  Inspiration du TP : https://surma.dev/things/ditherpunk/
