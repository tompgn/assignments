//! This is minimal Merkle tree implementation with proof checking

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};

fn main() {
	let s = "";
	let h = hash(&s);
	println!("{}", h);
}

/// We'll use Rust's built-in hashing which returns a u64 type.
/// This alias just helps us understand when we're treating the number as a hash
type HashValue = u64;

/// Helper function that makes the hashing interface easier to understand.
fn hash<T: Hash>(t: &T) -> HashValue {
	let mut s = DefaultHasher::new();
	t.hash(&mut s);
	s.finish()
}

/// Given a vector of data blocks this function adds padding blocks to the end
/// until the length is a power of two which is needed for Merkle trees.
fn pad_base_layer(blocks: &mut Vec<&str>) {
	let mut n=blocks.len(); // n>0 or error
	let mut pow = 1;
	while n >= 1 {
		pow = pow << 1;
		n = n >> 1;
	}
	for _ in 0..(pow-blocks.len()) {
		blocks.push("");
	}
}

/// Helper function to combine two hashes and compute the hash of the combination.
/// This will be useful when building the intermediate nodes in the Merkle tree.
///
/// There are many correct ways to do this, but the easiest one and the one that I recommend
/// is to convert the hashes to strings and concatenate them.
fn concatenate_hash_values(left: HashValue, right: HashValue) -> HashValue {
	left.to_string().push_str(&right.to_string());
	hash(&left)
}

/// Calculates the Merkle root of a sentence. We consider each word in the sentence to
/// be one block. Words are separated by one or more spaces.
///
/// Example:
/// Sentence: "Trust me, bro!"
/// "Trust", "me," "bro!"
/// Notice that the punctuation like the comma and exclamation point are included in the words
/// but the spaces are not.
fn calculate_merkle_root(sentence: &str) -> HashValue {
	let mut blocks: Vec<&str> = sentence.split(' ').collect();
	pad_base_layer(&mut blocks);
	let mut hash_blocks: Vec<HashValue> = blocks.iter().map(|x| hash(&x.to_owned())).collect();
	while 1<hash_blocks.len(){
		hash_blocks = hash_blocks.chunks(2).map(|chunk| {
				concatenate_hash_values(chunk[0],chunk[1])
			}).collect();
	}
	hash_blocks[0]
}

/// A representation of a sibling node along the Merkle path from the data
/// to the root. It is necessary to specify which side the sibling is on
/// so that the hash values can be combined in the same order.
enum SiblingNode {
	Left(HashValue),
	Right(HashValue),
}

/// Generates a Merkle proof that one particular word is contained
/// in the given sentence. You provide the sentence and the index of the word
/// which you want a proof.
///
/// Panics if the index is beyond the length of the word.
///
/// Example: I want to prove that the word "Trust" is in the sentence "Trust me, bro!"
/// So I call generate_proof("Trust me, bro!", 0)
/// And I get back the merkle root and list of intermediate nodes from which the
/// root can be reconstructed.
fn generate_proof(sentence: &str, index: usize) -> (HashValue, Vec<SiblingNode>) {
	// let blocks: Vec<HashValue> = sentence.split(" ").map(|x| hash(&x.to_owned())).collect();
	let mut blocks: Vec<&str> = sentence.split(' ').collect();
	let mut key = hash(&blocks[index].to_owned());
	pad_base_layer(&mut blocks);
	let mut hash_blocks: Vec<HashValue> = blocks.iter().map(|x| hash(&x.to_owned())).collect();
	let mut proof: Vec<SiblingNode> = Vec::new();
	while 1<hash_blocks.len(){
		hash_blocks = hash_blocks.chunks(2).map(|chunk| {
			let rt = concatenate_hash_values(chunk[0],chunk[1]);
			if chunk[0]==key {
				key = rt;
				proof.push(SiblingNode::Right(chunk[1]));
			}
			else if chunk[1]==key {
				key = rt;
				proof.push(SiblingNode::Left(chunk[0]));
			}
		rt
		}).collect();
	}
	(hash_blocks[0],proof)
}

/// Checks whether the given word is contained in a sentence, without knowing the whole sentence.
/// Rather we only know the merkle root of the sentence and a proof.
fn validate_proof(root: &str, word: &str, proof: Vec<SiblingNode>) -> bool {
	let mut key = hash(&word.to_owned());
	let hash_root=root.parse::<u64>().unwrap();
	for p in proof.iter() {
		match p {
			SiblingNode::Left(h0) => key = concatenate_hash_values(*h0,key),
			SiblingNode::Right(h1) => key = concatenate_hash_values(key,*h1),
		}
	}
	key==hash_root
}

#[test]
fn we_should_probably_have_some_tests() {
	let sentence = "Trust me, bro!";
	let hash_root = calculate_merkle_root(sentence);
	let str_root = hash_root.to_string();
	println!("Root as string: {:?}",str_root);
	let (root, proof) = generate_proof(sentence,0);
	assert_eq!(root,hash_root);
	println!("Root: {:?}",root);
	let valid = validate_proof(&str_root,"Trust",proof);
	let (root, proof) = generate_proof(sentence,0);
	let invalid = validate_proof(&str_root,"me,",proof);
	println!("invalid == false? {}",invalid);
	assert!(valid, "Invalid proof.");
	assert!(!invalid, "Fake proof appears valid.");

}
