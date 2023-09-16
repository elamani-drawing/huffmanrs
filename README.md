# Huffmanrs

Huffmanrs est une bibliothèque Rust permettant d'encoder et de décoder du texte en utilisant le codage de Huffman.

## Fonctionnalités
- Encode du texte.
- Décode du code.

## Installation
Pour utiliser Huffmanrs dans votre projet, ajoutez la dépendance suivante à votre fichier Cargo.toml :
```toml
[dependencies]
huffmanrs = "0.1.0"
```
## Example
```rust
use huffmanrs::{HuffmanNode, Huffman};

fn main() { 
    // Créer une instance de huffman
    let mut huffman = Huffman::new();
    // Demande un texte en entré pour calculer les fréquences et construire l'arbre d'encodage
    huffman.build("hello");
    // Exemple d'encodage
    dbg!(huffman.encode("abbccc")); // encode le mot abbccc en 101111000
    // Exemple de décodage
    dbg!(huffman.decode("101111000")); // decode le code 101111000 en abbccc

    // L'API de Huffman est disponible via Huffman::, vous pouvez donc construire votre propre implémentation de Huffman et réutiliser les composants qui vous intéressent, tels que Huffman::build_huffman_tree etc.
}
```
Pour plus d'exemple d'utilisation, vous pouvez lire en détail la documentation et les tests.
## Attention !!!!!!!!
`Huffman::build_frequency_table` n'attribue aucune priorité aux motifs, donc si plusieurs motifs ont la même fréquence, comme le "h", "e" et "o" dans "hello", l'exécution successive de ce code ne devrait pas vous donner le même encodage.
```rust
use huffmanrs::{HuffmanNode, Huffman};

fn main() { 
    // Créer une instance de huffman
    let mut huffman = Huffman::new();
    // Demande un texte en entré pour construire l'arbre de fréquence
    huffman.build("hello");
    // Exemple d'encodage
    dbg!(huffman.encode("hello")); // sortie possible :1001111100, 1000111101, 0010111101 etc.

    // Si ce comportement ne vous satisfait pas, vous pouvez réimplémenter Huffman::build_huffman_tree.
}
```
Dans ce genre de cas, il est essentiel pour le décodage d'avoir accès à la table de fréquence ou à l'arbre qui a été généré.


## Contributions
Les contributions sont les bienvenues ! Si vous souhaitez améliorer ou ajouter des fonctionnalités à Huffmanrs, veuillez ouvrir une pull request sur GitHub.

## License
Ce projet est sous [``licence MIT``](LICENSE). Veuillez consulter le fichier [``LICENSE``](LICENSE) pour plus d'informations.