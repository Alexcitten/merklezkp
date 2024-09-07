# Merkle ZKP

A Rust-based project integrating Merkle trees with Zero-Knowledge Proofs (ZKPs).

## Overview
This project demonstrates my ability to implement cryptographic data structures, specifically **Merkle Trees**, combined with **Zero-Knowledge Proofs** for verifiable computation. The Merkle tree ensures data integrity by constructing efficient, verifiable proofs of inclusion, while the ZKP component provides a mechanism for proof generation without revealing sensitive information.

## Key Features
- **Merkle Trees**: Ensures data integrity by constructing efficient proofs of inclusion.
- **Zero-Knowledge Proofs**: Verifiable computation without revealing sensitive information.
- **zk-SNARKs**: Implemented via the `bellman` library, providing secure proof creation and verification for small inputs.
- **Rust Performance**: Leverages Rust’s performance and memory safety to handle complex cryptographic operations with minimal overhead.

## My Role
In this project, I implemented the core cryptographic primitives and ensured the efficiency and security of proof generation and verification. By combining **Merkle Trees** and **zk-SNARKs**, I demonstrated the ability to integrate advanced cryptographic protocols into practical applications, proving the inclusion of specific data without revealing the data itself.

## How to Use
   ```bash
   git clone https://github.com/Alexcitten/merklezkp
   cd merklezkp
   cargo build
   cargo run
   ```

## Expected Output

Upon running the program, you should see the following output, which demonstrates the generation of a Merkle root, proof for the first element, and validation of the proof:
   ```bash
    Merkle Root: 58c89d709...131c102cfd
    Proof for first element: [("3e23e81...59c009d", true), ("d3a0...4891a", true)]
    Leaf hash: ca9781...7785afee48bb
    Is proof valid? true
   ```
This demonstrates that the program successfully constructs a Merkle tree, generates a valid proof for the first element, and verifies that the proof matches the root.

## LICENSE
This project is licensed under the MIT License