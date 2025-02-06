use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Option<Node>,
    pub leaves: Vec<Vec<u8>>,
}

fn main() {
    let data = vec![
        b"transaction1".to_vec(),
        b"transaction2".to_vec(),
        b"transaction3".to_vec(),
        b"transaction4".to_vec(),
    ];

    let tree = MerkleTree::new(data.clone());
    let proof = tree.get_proof(&data[0]);
    
    assert!(MerkleTree::verify(
        &tree.root.unwrap().hash,
        &data[0],
        &proof
    ));
}

#[derive(Debug, Clone)]
pub struct Node {
    pub hash: Vec<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl MerkleTree {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        let mut tree = MerkleTree {
            root: None,
            leaves: data.clone(),
        };
        if !data.is_empty() {
            tree.root = Some(tree.build_tree(&data));
        }
        tree
    }

    fn hash(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    fn build_tree(&self, nodes: &[Vec<u8>]) -> Node {
        if nodes.len() == 1 {
            return Node {
                hash: Self::hash(&nodes[0]),
                left: None,
                right: None,
            };
        }

        let mid = nodes.len() / 2;
        let left = self.build_tree(&nodes[..mid]);
        let right = self.build_tree(&nodes[mid..]);
        
        let mut combined = left.hash.clone();
        combined.extend(&right.hash);

        Node {
            hash: Self::hash(&combined),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    pub fn get_proof(&self, leaf: &[u8]) -> Vec<Vec<u8>> {
        let mut proof = Vec::new();
        if let Some(root) = &self.root {
            self.build_proof(root, leaf, &mut proof);
        }
        proof
    }

    fn build_proof(&self, node: &Node, leaf: &[u8], proof: &mut Vec<Vec<u8>>) -> bool {
        if node.left.is_none() && node.right.is_none() {
            return node.hash == Self::hash(leaf);
        }

        if let Some(left) = &node.left {
            if self.build_proof(left, leaf, proof) {
                if let Some(right) = &node.right {
                    proof.push(right.hash.clone());
                }
                return true;
            }
        }

        if let Some(right) = &node.right {
            if self.build_proof(right, leaf, proof) {
                if let Some(left) = &node.left {
                    proof.push(left.hash.clone());
                }
                return true;
            }
        }

        false
    }

    pub fn verify(root: &[u8], leaf: &[u8], proof: &[Vec<u8>]) -> bool {
        let mut current = Self::hash(leaf);
        
        for sibling in proof {
            let mut combined = current.clone();
            combined.extend(sibling);
            current = Self::hash(&combined);
        }

        current == root.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree() {
        let data = vec![
            b"transaction1".to_vec(),
            b"transaction2".to_vec(),
            b"transaction3".to_vec(),
            b"transaction4".to_vec(),
        ];

        let tree = MerkleTree::new(data.clone());
        let proof = tree.get_proof(&data[0]);
        
        assert!(MerkleTree::verify(
            &tree.root.unwrap().hash,
            &data[0],
            &proof
        ));
    }
}