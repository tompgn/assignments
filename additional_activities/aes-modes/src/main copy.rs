// //! In Module 1, we discussed Block ciphers like AES. Block ciphers have a fixed length input.
// //! Real wold data that we wish to encrypt _may_ be exactly the right length, but is probably not.
// //! When your data is too short, you can simply pad it up to the correct length.
// //! When your data is too long, you have some options.
// //!
// //! In this exercise, we will explore a few of the common ways that large pieces of data can be broken
// //! up and combined in order to encrypt it with a fixed-length block cipher.
// //!
// //! WARNING: ECB MODE IS NOT SECURE.
// //! Seriously, ECB is NOT secure. Don't use it irl. We are implementing it here to understand _why_ it
// //! is not secure and make the point that the most straight-forward approach isn't always the best, and
// //! can sometimes be trivially broken.

// use aes::cipher::{generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit};
// use aes::Aes128;

// ///We're using AES 128 which has 16-byte (128 bit) blocks.
// const BLOCK_SIZE: usize = 16;

// fn main(){
// 	let message = "Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap Some very long crap.";
// 	let key : [u8; 16]= "0123456789ABCDEF".as_bytes().try_into().expect("iccorrect length.");
// 	let em = ecb_encrypt(message.as_bytes().to_vec(),key);
// 	println!("cipher = {:?}",&em);
// 	let dm = ecb_decrypt(em,key);
// 	println!("plain = {:?}", std::str::from_utf8(&dm));
// 	let message2 = "Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap Another very long crap.";
// 	let em_cbc = cbc_encrypt(message2.as_bytes().to_vec(),key);
// 	println!("cipher_cbc = {:?}",&em_cbc);
// 	let dm_cbc = cbc_decrypt(em_cbc,key);
// 	println!("plain_cbc = {:?}", std::str::from_utf8(&dm_cbc));
// }

// /// Simple AES encryption
// /// Helper function to make the core AES block cipher easier to understand.
// fn aes_encrypt(data: [u8; BLOCK_SIZE], key: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
// 	// Convert the inputs to the necessary data type
// 	let mut block = GenericArray::from(data);
// 	let key = GenericArray::from(*key);

// 	let cipher = Aes128::new(&key);

// 	cipher.encrypt_block(&mut block);

// 	block.into()
// }

// /// Simple AES encryption
// /// Helper function to make the core AES block cipher easier to understand.
// fn aes_decrypt(data: [u8; BLOCK_SIZE], key: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
// 	// Convert the inputs to the necessary data type
// 	let mut block = GenericArray::from(data);
// 	let key = GenericArray::from(*key);

// 	let cipher = Aes128::new(&key);

// 	cipher.decrypt_block(&mut block);

// 	block.into()
// }

// /// Before we can begin encrypting our raw data, we need it to be a multiple of the
// /// block length which is 16 bytes (128 bits) in AES128.
// ///
// /// The padding algorithm here is actually not trivial. The trouble is that if we just
// /// naively throw a bunch of zeros on the end, there is no way to know, later, whether
// /// those zeros are padding, or part of the message, or some of each.
// ///
// /// The scheme works like this. If the data is not a multiple of the block length,  we
// /// compute how many pad bytes we need, and then write that number into the last several bytes.
// /// Later we look at the last byte, and remove that number of bytes.
// ///
// /// But if the data _is_ a multiple of the block length, then we have a problem. We don't want
// /// to later look at the last byte and remove part of the data. Instead, in this case, we add
// /// another entire block containing the block length in each byte. In our case,
// /// [16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16]
// <<<<<<< Updated upstream
// =======
// /// 
// /// This function also chops the data into blocks for us.
// >>>>>>> Stashed changes
// fn pad(mut data: Vec<u8>) -> Vec<u8> {
// 	// When twe have a multiple the second term is 0
// 	let number_pad_bytes = BLOCK_SIZE - data.len() % BLOCK_SIZE;

// 	for _ in 0..number_pad_bytes {
// 		data.push(number_pad_bytes as u8);
// 	}

// 	data
// }

// <<<<<<< Updated upstream
// /// Groups the data into BLOCK_SIZE blocks. Assumes the data is already
// /// a multiple of the block size. If this is not the case, call `pad` first.
// fn group(data: Vec<u8>) -> Vec<[u8; BLOCK_SIZE]> {
// =======
// fn split_blocks(mut data: Vec<u8>) -> Vec<[u8; 16]> {
// >>>>>>> Stashed changes
// 	let mut blocks = Vec::new();
// 	let mut i = 0;
// 	while i < data.len() {
// 		let mut block: [u8; BLOCK_SIZE] = Default::default();
// 		block.copy_from_slice(&data[i..i + BLOCK_SIZE]);
// 		blocks.push(block);

// 		i += BLOCK_SIZE;
// 	}

// 	blocks
// }

// <<<<<<< Updated upstream
// /// Does the opposite of the group function
// fn un_group(blocks: Vec<[u8; BLOCK_SIZE]>) -> Vec<u8> {
// 	todo!()
// =======
// /// Does the opposite of the pad function.
// fn merge_blocks(data: Vec<[u8; 16]>) -> Vec<u8> {
// 	let mut message: Vec<u8> = Vec::new();
// 	let mut i = 0;
// 	for block in data.iter() {
// 		message.extend_from_slice(&block[..]);
// 	}
// 	message
// }

// /// Does the opposite of the pad function.
// fn un_pad(mut message: Vec<u8>) -> Vec<u8>{
// 	let number_pad_bytes = message.last().unwrap().clone();
// 	for _ in 0..number_pad_bytes {
// 		message.pop();
// 	}
// 	message
// >>>>>>> Stashed changes
// }

// /// Does the opposite of the pad function.
// fn un_pad(data: Vec<[u8; 16]>) -> Vec<u8> {
// 	todo!()
// }

// /// The first mode we will implement is the Electronic Code Book, or ECB mode.
// /// Warning: THIS MODE IS NOT SECURE!!!!
// ///
// /// This is probably the first thing you think of when considering how to encrypt
// /// large data. In this mode we simply encrypt each block of data under the same key.
// /// One good thing about this mode is that it is parallelizable. But to see why it is
// /// insecure look at: https://www.ubiqsecurity.com/wp-content/uploads/2022/02/ECB2.png
// fn ecb_encrypt(plain_text: Vec<u8>, key: [u8; 16]) -> Vec<u8> {
// <<<<<<< Updated upstream
// 	todo!()
// }

// /// Opposite of ecb_encrypt.
// fn ecb_decrypt(cipher_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {
// 	todo!()
// }

// =======
// 	let plain_blocks = split_blocks(pad(plain_text));
// 	let cipher_blocks = plain_blocks.iter().map(|&pb| aes_encrypt(pb, &key)).collect();

// 	let cipher_text = merge_blocks(cipher_blocks);

// 	cipher_text
// }

// /// Opposite of ecb_encrypt.
// fn ecb_decrypt(cipher_text: Vec<u8>, key: [u8; 16]) -> Vec<u8> {
// 	let cipher_blocks = split_blocks(cipher_text);

// 	let plain_blocks = cipher_blocks.iter().map(|&cb| aes_decrypt(cb, &key)).collect();

// 	let plain_text = un_pad(merge_blocks(plain_blocks));

// 	plain_text
// }

// >>>>>>> Stashed changes
// /// The next mode, which you can implement on your own is cipherblock chaining.
// /// This mode actually is secure, and it often used in real world applications.
// ///
// /// In this mode, the ciphertext from the first block is XORed with the
// /// plaintext of the next block before it is encrypted.
// <<<<<<< Updated upstream
// fn cbc_encrypt(plain_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {
// 	// Remember to generate a random initialization vector for the first block.

// 	todo!()
// }

// fn cbc_decrypt(cipher_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {
// 	todo!()
// }
// =======
// fn cbc_encrypt(plain_text: Vec<u8>, key: [u8; 16]) -> Vec<u8> {
// 	let init : [u8; 16]= "AAAAAAAAAAAAAAAA".as_bytes().try_into().expect("incorrect length.");
// 	let mut plain_blocks = split_blocks(pad(plain_text));
// 	let mut acc = init.clone();
// 	let mut cipher_blocks : Vec<[u8; 16]> = Vec::new();
// 	for mut block in plain_blocks.iter_mut() {
// 		block.iter_mut().zip(acc.iter()).for_each(|(x1, x2)| *x1 ^= *x2);
// 		acc = aes_encrypt(block.clone(), &key);
// 		cipher_blocks.push(acc.clone());
// 	}

// 	let cipher_text = merge_blocks(cipher_blocks);

// 	cipher_text
// }

// fn cbc_decrypt(cipher_text: Vec<u8>, key: [u8; 16]) -> Vec<u8> {
// 	let init : [u8; 16]= "AAAAAAAAAAAAAAAA".as_bytes().try_into().expect("incorrect length.");
// 	let cipher_blocks = split_blocks(cipher_text);

// 	let mut acc = init.clone();
// 	let mut plain_blocks : Vec<[u8; 16]> = Vec::new();

// 	for block in cipher_blocks.into_iter() {
// 		let mut plain_block_xored = aes_decrypt(block.clone(), &key);
// 		plain_block_xored.iter_mut().zip(acc.iter()).for_each(|(x1, x2)| *x1 ^= *x2);
// 		plain_blocks.push(plain_block_xored.clone());
// 		acc = block;

// 	}

// 	let plain_text = un_pad(merge_blocks(plain_blocks));

// 	plain_text
// }
// >>>>>>> Stashed changes
