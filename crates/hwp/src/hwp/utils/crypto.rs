use aes::cipher::KeyInit;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt};
use aes::Aes128;

pub fn decrypt_aes_128_ecb(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut blocks = Vec::new();
    (0..data.len()).step_by(16).for_each(|x| {
        blocks.push(GenericArray::clone_from_slice(&data[x..x + 16]));
    });

    let cipher = Aes128::new_from_slice(&key).unwrap();
    cipher.decrypt_blocks(&mut blocks);

    blocks.into_iter().flatten().map(|x| x as u8).collect()
}
