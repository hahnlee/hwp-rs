use openssl::{
    error::ErrorStack,
    symm::{Cipher, Crypter, Mode},
};

pub fn decrypt_aes_128_ecb(key: &[u8], data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_128_ecb();
    let mut c = Crypter::new(cipher, Mode::Decrypt, key, None)?;
    c.pad(false);
    let mut out = vec![0; data.len() + cipher.block_size()];
    let count = c.update(data, &mut out)?;
    let rest = c.finalize(&mut out[count..])?;
    out.truncate(count + rest);
    Ok(out)
}
