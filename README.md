# Merkle Tree Implementation in Rust

This project demonstrates the implementation of a Merkle Tree in Rust. A Merkle Tree is a binary tree in which every leaf node is a hash of a data block, and every non-leaf node is a hash of its children. Merkle Trees are used in distributed systems for efficient and secure verification of data integrity.

## Implementation

The implementation includes the following components:
- `MerkleTree` struct: Represents the Merkle Tree with methods to build the tree, generate proofs, and verify proofs.
- `Node` struct: Represents a node in the Merkle Tree.
- `main` function: Demonstrates the usage of the Merkle Tree by creating a tree, generating a proof, and verifying the proof.
- Unit tests: Verify the functionality of the Merkle Tree implementation.

### Key Methods

- `MerkleTree::new(data: Vec<Vec<u8>>) -> Self`: Constructs a new Merkle Tree from the given data.
- `MerkleTree::get_proof(&self, leaf: &[u8]) -> Vec<Vec<u8>>`: Generates a proof for a given leaf.
- `MerkleTree::verify(root: &[u8], leaf: &[u8], proof: &[Vec<u8>]) -> bool`: Verifies a proof against the root hash.

## Installation

To use this project, you need to have Rust installed. If you don't have Rust installed, you can install it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/yourusername/merkle-tree-rs.git
cd merkle-tree-rs
```

## Dependencies

This project uses the `sha2` crate for hashing. Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
sha2 = "0.9"
```

## Running the Project

To run the project, use the following command:

```sh
cargo run
```

## Running Tests

To run the tests, use the following command:

```sh
cargo test
```

## Example

Here is an example of how to use the Merkle Tree:

```rust
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
```

This example creates a Merkle Tree from a list of transactions, generates a proof for the first transaction, and verifies the proof against the root hash.

## License

This project is licensed under the MIT License.