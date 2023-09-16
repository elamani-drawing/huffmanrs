use std::{cmp::Ordering, fmt};

#[derive(Debug, Eq, Clone)]
pub struct HuffmanNode {
    character: Option<char>,
    frequency: u32,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl fmt::Display for HuffmanNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left_character : Option<char> = match &self.left {
            Some(node) => node.character,
            None => None,
        };

        let right_character : Option<char> = match &self.right {
            Some(node) => node.character,
            None => None,
        };

        write!(
            f,
            "(val: {:?}, f: {}, l: {:?}, r: {:?})",
            self.character, self.frequency, left_character, right_character
        )
    }
}

impl HuffmanNode {
    /// Crée un nouveau noeud de l'arbre de Huffman.
    ///
    /// # Arguments
    ///
    /// * `character` - Le caractère associé au noeud.
    /// * `frequency` - La fréquence du caractère.
    /// * `left` - Le sous-arbre gauche.
    /// * `right` - Le sous-arbre droit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use huffmanrs::HuffmanNode;
    ///
    /// fn main() {
    ///     let left = HuffmanNode::new(Some('a'), 2, None, None);
    ///     let right = HuffmanNode::new(Some('b'), 3, None, None);
    ///     let node = HuffmanNode::new(None, 5, Some(Box::new(left)), Some(Box::new(right)));
    /// }
    /// ```
    pub fn new(
        character: Option<char>,
        frequency: u32,
        left: Option<Box<HuffmanNode>>,
        right: Option<Box<HuffmanNode>>,
    ) -> Self {
        HuffmanNode {
            character,
            frequency,
            left,
            right,
        }
    }
    /// Récupère le caractère associé au noeud.
    pub fn character(&self) -> Option<char> {
        self.character
    }

    /// Récupère la fréquence du noeud.
    pub fn frequency(&self) -> u32 {
        self.frequency
    }

    /// Obtenir le sous-arbre gauche du noeud.
    pub fn left(&self) -> Option<&HuffmanNode> {
        self.left.as_ref().map(|node| node.as_ref())
    }

    /// Obtenir le sous-arbre droit du noeud.
    pub fn right(&self) -> Option<&HuffmanNode> {
        self.right.as_ref().map(|node| node.as_ref())
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency).reverse()
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}
