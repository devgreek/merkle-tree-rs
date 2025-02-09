use ring::digest::{Context, SHA256};

#[derive(Debug)]
enum Node {
    Leaf(Vec<u8>), // Data
    Internal(Box<Node>, Box<Node>), // Left, Right children
}

fn calculate_hash(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}

impl Node {
    fn new_leaf(data: Vec<u8>) -> Self {
        Node::Leaf(data)
    }

    fn new_internal(left: Node, right: Node) -> Self {
        Node::Internal(Box::new(left), Box::new(right))
    }

    fn compute_hash(&self) -> Vec<u8> {
        match self {
            Node::Leaf(data) => calculate_hash(data),
            Node::Internal(left, right) => {
                let left_hash = left.compute_hash();
                let right_hash = right.compute_hash();
                let combined = [&left_hash[..], &right_hash[..]].concat();
                calculate_hash(&combined)
            }
        }
    }
}

fn main() {
    let data1 = b"Data Block 1".to_vec();
    let data2 = b"Data Block 2".to_vec();
    let data3 = b"Data Block 3".to_vec();

    let leaf1 = Node::new_leaf(data1);
    let leaf2 = Node::new_leaf(data2);
    let leaf3 = Node::new_leaf(data3);

    let internal1 = Node::new_internal(leaf1, leaf2);
    let root = Node::new_internal(internal1, leaf3); // Unbalanced tree

    let merkle_root = root.compute_hash();
    println!("Merkle Root: {:?}", merkle_root);


    // Example of adding a new leaf (making it more unbalanced):
    let data4 = b"Data Block 4".to_vec();
    let leaf4 = Node::new_leaf(data4);
    let new_root = Node::new_internal(root, leaf4);
    let new_merkle_root = new_root.compute_hash();
    println!("New Merkle Root (after adding data): {:?}", new_merkle_root);
}