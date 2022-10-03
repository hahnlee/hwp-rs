use openssl::{
    error::ErrorStack,
    symm::{Cipher, Crypter, Mode},
};

/// openssl의 cipher함수에 pad를 끄는 기능을 적용한 패치 
/// 
/// openssl의 aes_128_ecb를 사용하면 padding이 있는걸로 간주하기때문에 제대로 압축해제되지 않는다
/// 
/// 참고:
/// https://github.com/sfackler/rust-openssl/blob/openssl-v0.10.42/openssl/src/symm.rs#L690
pub fn decrypt_aes_128_ecb(key: &[u8], data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_128_ecb();
    let mut c = Crypter::new(cipher, Mode::Decrypt, key, None)?;
    // 패치한 부분
    c.pad(false);
    let mut out = vec![0; data.len() + cipher.block_size()];
    let count = c.update(data, &mut out)?;
    let rest = c.finalize(&mut out[count..])?;
    out.truncate(count + rest);
    Ok(out)
}
