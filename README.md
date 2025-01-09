# TP_Rust_Ditherpunk

## Réponses aux questions et explications :

  ### Question 2 - Pour ouvrir une image depuis un fichier, on utilise ``` ImageReader::open("myimage.png")?.decode()?; ``` On obtient un DynamicImage, à quoi correspond ce type? Comment obtenir une image en mode rbg8 à partir de ce DynamicImage?

  Le type DynamicImage est un enum fournie que la bibliothèque image fournit et qui sert a représenter une image. Cela permet d' encapsuler différents types d’images en fonction :

  - De leurs nombre de canaux (RGB, RGBA, ...).
  - De leurs type de données (u8, u16, ...).

  Avec cette bibliothèque, on peut gérer plusieurs formats d'images sans avoir à spécifier le type exact au moment de la lecture.



## Liens utiles :

  Inspiration du TP : https://surma.dev/things/ditherpunk/
