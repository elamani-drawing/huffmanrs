use crate::HuffmanNode;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Huffman {
    huffman_tree: Option<Box<HuffmanNode>>,
    code_table: Option<HashMap<char, String>>,
}

impl Clone for Huffman {
    fn clone(&self) -> Self {
        Huffman {
            huffman_tree: self.huffman_tree.clone(),
            code_table: self.code_table.clone(),
        }
    }
}

impl Huffman {
    /// Crée une nouvelle instance de Huffman.
    ///
    /// # Returns
    ///
    /// Une nouvelle instance de Huffman.
    ///
    /// # Examples
    ///
    /// ```
    /// use huffmanrs::Huffman;
    ///
    /// fn main() {
    ///     let huffman = Huffman::new();
    ///
    ///     // Utiliser l'instance de Huffman à travers .build puis .encode et .decode
    /// }
    /// ```
    pub fn new() -> Self {
        Huffman {
            huffman_tree: None,
            code_table: None,
        }
    }
    /// Obtenir l'arbre de Huffman.
    pub fn get_huffman_tree(&self) -> &Option<Box<HuffmanNode>> {
        &self.huffman_tree
    }

    /// Définir l'arbre de Huffman.
    pub fn set_huffman_tree(&mut self, tree: Option<Box<HuffmanNode>>) {
        self.huffman_tree = tree;
    }

    /// Obtenir la table de codes.
    pub fn get_code_table(&self) -> &Option<HashMap<char, String>> {
        &self.code_table
    }

    /// Définir la table de codes.
    pub fn set_code_table(&mut self, table: Option<HashMap<char, String>>) {
        self.code_table = table;
    }

    /// Construit l'arbre de Huffman et la table d'encodage correspondante à partir d'un texte.
    ///
    /// Cette méthode construit l'arbre de Huffman et la table de codes correspondante en utilisant
    /// le texte fourni. L'arbre de Huffman est utilisé pour l'encodage et le décodage ultérieur.
    ///
    /// # Arguments
    ///
    /// * `text` - Le texte à partir duquel seront construits l'arbre de Huffman et la table de codes.
    ///
    /// # Returns
    ///
    /// Une chaîne de caractères indiquant le succès de la construction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use huffmanrs::Huffman;
    ///
    /// fn main() {
    ///     let mut huffman = Huffman::new();
    ///     let text = "hello world";
    ///     huffman.build(text);
    ///     // Utiliser l'instance de Huffman à travers .encode et .decode
    /// }
    /// ```
    pub fn build(&mut self, text: &str) {
        let frequence_table: HashMap<char, u32> = Huffman::build_frequency_table(text);
        let huffman_tree: Option<Box<HuffmanNode>> = Huffman::build_huffman_tree(&frequence_table);
        let mut code_table: HashMap<char, String> = HashMap::new();
        self.huffman_tree = huffman_tree.clone();
        let root: Box<HuffmanNode> = huffman_tree.unwrap();
        Huffman::build_code_table(&root, format!(""), &mut code_table);
        self.code_table = Some(code_table);
    }

    /// Cette méthode décode le texte encodé en utilisant l'arbre de Huffman associé à cette
    /// instance spécifique de Huffman. Le texte encodé doit avoir été précédemment encodé.
    ///
    /// # Arguments
    ///
    /// * `encoded_text` - Le texte encodé à décoder.
    ///
    /// # Returns
    ///
    /// Le texte décodé correspondant au texte encodé fourni.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use huffmanrs::Huffman;
    ///
    /// fn main() {
    ///     let mut huffman = Huffman::new();
    ///     let text_de_reference = "hello world";
    ///     huffman.build(text_de_reference);
    ///
    ///     let clear_text = format!("hello world");
    ///     let encoded_text = match huffman.encode(clear_text.as_str()) {
    ///         Ok(text) => text,
    ///         Err(error) => {
    ///             println!("Error: {}", error);
    ///             return;
    ///         }
    ///     };
    ///     let decoded_text = huffman.decode(encoded_text.as_str());
    ///     
    ///     match decoded_text {
    ///         Ok(text) => assert_eq!(text, clear_text),
    ///         Err(error) => println!("Error: {}", error),
    ///     }
    ///     // Utiliser le texte décodé
    /// }
    /// ```
    pub fn decode(&self, encoded_text: &str) -> Result<String, String> {
        if let Some(huffman_tree) = &self.huffman_tree {
            Ok(Huffman::decode_text(encoded_text, huffman_tree))
        } else {
            Err(String::from("Code table is not available"))
        }
    }

    /// Encode le texte à l'aide de la table de codes associée à cette instance.
    ///
    /// Cette méthode encode le texte en utilisant la table de codes associée à cette
    /// instance spécifique de Huffman. La table de codes doit avoir été préalablement
    /// construite à l'aide de la méthode `build`.
    ///
    /// # Arguments
    ///
    /// * `text` - Le texte à encoder.
    ///
    /// # Returns
    ///
    /// Le texte encodé correspondant au texte fourni.
    ///
    /// # Examples
    ///
    /// ```
    /// use huffmanrs::Huffman;
    ///
    /// fn main() {
    ///     let mut huffman = Huffman::new();
    ///     let text_de_reference = "heellllooo";
    ///     huffman.build(text_de_reference);
    ///
    ///     let clear_text = "hello";
    ///     let encoded_text = huffman.encode(clear_text);
    ///
    ///     // Vérification du résultat
    ///     match encoded_text {
    ///         Ok(text) => assert_eq!(text, format!("1101110010")),
    ///         Err(error) => println!("Error: {}", error),
    ///     }
    ///     // Utiliser le texte encode ou le decode à l'aide de .decode
    /// }
    /// ```
    pub fn encode(&self, encoded_text: &str) -> Result<String, String> {
        if let Some(code_table) = &self.code_table {
            Ok(Huffman::encode_text(encoded_text, code_table))
        } else {
            Err(String::from("Code table is not available"))
        }
    }

    /// Construit une table de fréquence des caractères à partir d'un texte.
    ///
    /// # Arguments
    ///
    /// * `text` - Le texte à partir duquel construire la table de fréquence.
    ///
    /// # Returns
    ///
    /// Une `HashMap` contenant les caractères du texte et leur fréquence respective.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use huffmanrs::Huffman;
    ///
    /// fn main() {
    ///     let text = "hello world";
    ///     let frequency_table = Huffman::build_frequency_table(text);
    ///     println!("{:?}", frequency_table);
    ///
    ///     // Vérifie que le résultat est correct
    ///     assert_eq!(frequency_table.get(&'h'), Some(&1));
    ///     assert_eq!(frequency_table.get(&'e'), Some(&1));
    ///     assert_eq!(frequency_table.get(&'l'), Some(&3));
    ///     assert_eq!(frequency_table.get(&'o'), Some(&2));
    ///     assert_eq!(frequency_table.get(&'w'), Some(&1));
    ///     assert_eq!(frequency_table.get(&'r'), Some(&1));
    ///     assert_eq!(frequency_table.get(&'d'), Some(&1));
    /// }
    /// ```
    pub fn build_frequency_table(text: &str) -> HashMap<char, u32> {
        let mut frequency_table: HashMap<char, u32> = HashMap::new();

        // Parcour chaque caractère dans le texte
        for c in text.chars() {
            // Incrémente la fréquence du caractère s'il existe déjà dans la table
            let count = frequency_table.entry(c).or_insert(0);
            *count += 1;
        }
        // retourne la table
        frequency_table
    }

    /// Construit un arbre de Huffman à partir d'une table de fréquence.
    ///
    /// # Arguments
    ///
    /// * `frequency_table` - La table de fréquence des caractères.
    ///
    /// # Returns
    ///
    /// Un `Option<Box<HuffmanNode>>` contenant la racine de l'arbre de Huffman.
    ///
    /// # Examples
    ///
    /// # Examples
    ///
    /// ```rust
    /// use huffmanrs::Huffman;
    /// use std::collections::HashMap;
    ///
    /// fn main() {
    ///     /// Création de la table de fréquence des caractères
    ///     let mut frequency_table: HashMap<char, u32> = HashMap::new();
    ///     frequency_table.insert('a', 5);
    ///     frequency_table.insert('b', 2);
    ///     frequency_table.insert('c', 1);
    ///     frequency_table.insert('d', 3);
    ///
    ///     /// Construction de l'arbre de Huffman
    ///     let huffman_tree = Huffman::build_huffman_tree(&frequency_table);
    ///
    ///     // Vérification de la structure de l'arbre de Huffman
    ///     assert_eq!(huffman_tree.is_some(), true);
    ///     let root = huffman_tree.unwrap();
    ///     assert_eq!(root.character(), None);
    ///     assert_eq!(root.frequency(), 11);
    ///
    ///     let left_child = root.left().unwrap();
    ///     assert_eq!(left_child.character(), Some('a'));
    ///     assert_eq!(left_child.frequency(), 5);
    ///
    ///     let right_child = root.right().unwrap();
    ///     assert_eq!(right_child.character(), None);
    ///     assert_eq!(right_child.frequency(), 6);
    /// }
    /// ```
    pub fn build_huffman_tree(frequency_table: &HashMap<char, u32>) -> Option<Box<HuffmanNode>> {
        let mut priority_queue: BinaryHeap<Box<HuffmanNode>> = BinaryHeap::new();

        for (&character, &frequency) in frequency_table {
            priority_queue.push(Box::new(HuffmanNode::new(
                Some(character),
                frequency,
                None,
                None,
            )));
        }

        while priority_queue.len() > 1 {
            let left_child = priority_queue.pop()?;
            let right_child = priority_queue.pop()?;

            let parent = HuffmanNode::new(
                None,
                left_child.frequency() + right_child.frequency(),
                Some(left_child),
                Some(right_child),
            );

            priority_queue.push(Box::new(parent));
        }

        priority_queue.pop()
    }

    /// Construit une table de codes à partir d'un arbre de Huffman.
    ///  
    /// # Arguments
    ///
    /// * `node` - Le noeud Huffman actuel à traiter.
    /// * `prefix` - Le préfixe actuel pour la construction du code binaire.
    /// * `code_table` - La table de codes à remplir avec les caractères et leurs codes.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use huffmanrs::{Huffman, HuffmanNode};
    /// use std::collections::HashMap;
    ///
    /// fn main() {
    ///     // Création d'un arbre de Huffman de démonstration
    ///     let leaf_a = HuffmanNode::new(Some('a'), 2, None, None);
    ///     let leaf_b = HuffmanNode::new(Some('b'), 1, None, None);
    ///     let inner = HuffmanNode::new(None, 1, Some(Box::new(leaf_a)), Some(Box::new(leaf_b)));
    ///     let root = HuffmanNode::new(None, 1, Some(Box::new(inner)), None);
    ///
    ///     // Construction de la table de codes
    ///     let mut code_table = HashMap::new();
    ///     Huffman::build_code_table(&root, String::new(), &mut code_table);
    ///
    ///     // Affichage de la table de codes résultante
    ///     for (character, code) in &code_table {
    ///         println!("Caractère: {}, Code: {}", character, code);
    ///     }
    ///
    ///     // Vérification de la table de codes résultante
    ///     assert_eq!(code_table.get(&'a'), Some(&"00".to_string()));
    ///     assert_eq!(code_table.get(&'b'), Some(&"01".to_string()));
    /// }
    /// ```
    pub fn build_code_table(
        node: &HuffmanNode,
        prefix: String,
        code_table: &mut HashMap<char, String>,
    ) {
        // Si le noeud contient un caractère, nous l'ajoutons à la table de codes en associant le caractère à son code binaire correspondant (le préfixe actuel).
        if let Some(character) = node.character() {
            code_table.insert(character, prefix);
        } else {
            // Si le noeud n'a pas de caractère, cela signifie qu'il s'agit d'un noeud interne de l'arbre.
            // Nous traitons récursivement les noeuds gauche et droit en appelant build_code_table avec des préfixes mis à jour. Les préfixes sont mis à jour en ajoutant '0' pour le noeud gauche et '1' pour le noeud droit.
            if let Some(ref left) = node.left() {
                let mut new_prefix = prefix.clone();
                new_prefix.push('0');
                Huffman::build_code_table(left, new_prefix, code_table);
            }
            if let Some(ref right) = node.right() {
                let mut new_prefix = prefix.clone();
                new_prefix.push('1');
                Huffman::build_code_table(right, new_prefix, code_table);
            }
        }
    }

    /// Encode le texte donné en utilisant une table de codes.
    ///
    /// # Arguments
    ///
    /// * `text` - Le texte à encoder.
    /// * `code_table` - La table de codes à utiliser pour l'encodage.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use huffmanrs::Huffman;
    /// use std::collections::HashMap;
    ///
    /// fn main() {
    ///     // Exemple d'utilisation de la fonction encode avec une table de codes
    ///
    ///     // Création d'une table de codes de démonstration
    ///     let mut code_table = HashMap::new();
    ///     code_table.insert('a', "0".to_string());
    ///     code_table.insert('b', "1".to_string());
    ///
    ///     // Encodage du texte "abab" en utilisant la table de codes
    ///     let encoded_text = Huffman::encode_text("abab", &code_table);
    ///
    ///     // Vérification du résultat attendu
    ///     assert_eq!(encoded_text, "0101");
    /// }
    /// ```
    pub fn encode_text(text: &str, code_table: &HashMap<char, String>) -> String {
        let mut encoded_text = String::new();

        // Parcour chaque caractère dans le texte
        for c in text.chars() {
            // Recherche le code correspondant dans la table de codes
            if let Some(code) = code_table.get(&c) {
                encoded_text.push_str(code);
            }
        }

        encoded_text
    }

    /// Décode le texte encodé donné en utilisant un arbre de Huffman.
    ///
    /// # Arguments
    ///
    /// * `encoded_text` - Le texte encodé à décoder.
    /// * `huffman_tree` - L'arbre de Huffman à utiliser pour le décodage.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use huffmanrs::{Huffman, HuffmanNode};
    ///
    /// fn main() {
    ///     // Exemple d'utilisation de la fonction decode avec un arbre de Huffman
    ///
    ///     // Création d'un arbre de Huffman de démonstration
    ///     let leaf_a = HuffmanNode::new(Some('a'), 1, None, None);
    ///     let leaf_b = HuffmanNode::new(Some('b'), 2, None, None);
    ///     let inner = HuffmanNode::new(None, 0, Some(Box::new(leaf_a)), Some(Box::new(leaf_b)));
    ///     let huffman_tree = HuffmanNode::new(None, 1, Some(Box::new(inner)), None);
    ///
    ///     // Décodage du texte encodé "0101" en utilisant l'arbre de Huffman
    ///     let decoded_text = Huffman::decode_text("0100", &huffman_tree);
    ///
    ///     // Vérification du résultat attendu
    ///     assert_eq!(decoded_text, "ba");
    /// }
    /// ```
    pub fn decode_text(encoded_text: &str, huffman_tree: &HuffmanNode) -> String {
        let mut decoded_text = String::new();
        let mut current_node = huffman_tree;
        // parcourt chaque bit de la chaîne encoded_text.
        for bit in encoded_text.chars() {
            if bit == '0' {
                if let Some(ref left) = current_node.left() {
                    current_node = left;
                }
            } else if bit == '1' {
                if let Some(ref right) = current_node.right() {
                    current_node = right;
                }
            }
            // Si le current_node contient un caractère, cela signifie que nous avons atteint une feuille de l'arbre de Huffman, et nous avons trouvé un caractère décodé.
            // Nous ajoutons ce caractère à la fin de decoded_text et réinitialisons current_node à l'arbre de Huffman d'origine pour commencer la recherche du prochain caractère à partir de la racine de l'arbre.
            if let Some(character) = current_node.character() {
                decoded_text.push(character);
                current_node = huffman_tree;
            }
        }

        decoded_text
    }
}
