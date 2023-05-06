//! Some useful util functions

use base64::{self, engine::general_purpose, Engine};
use rand::Rng;
use rsa::{pkcs8::DecodePublicKey, traits::PaddingScheme, Pkcs1v15Encrypt, RsaPublicKey};
use std::io::Write;

use crate::error::Error;

/// Parse unformatted pure Base64 public key to PKCS#8 RSA encoded public key
///
/// ## Input format:
///
/// ```
/// # let raw_pub =
/// "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC9XrJWcWbj0LhDBzN4uwEOLA/U\
/// JKmCkkbvlVgN/qei3e/jVFpxR6D3fzshnv5QNB4+BJ/rjRWbbxCJ0djzPxsLS1dJ\
/// +bDwagZWZ9hNXARTq4K0uxw6Ol5jGD9Od6w5n5uxyaEk9/edvYwMhthIxC/uADRp\
/// 2pNSutwyLX3bUJnHZwIDAQAB";
/// ```
///
/// ## Return format:
///
/// ```
/// # let result =
/// "\
/// -----BEGIN PUBLIC KEY-----
/// MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC9XrJWcWbj0LhDBzN4uwEOLA/U
/// JKmCkkbvlVgN/qei3e/jVFpxR6D3fzshnv5QNB4+BJ/rjRWbbxCJ0djzPxsLS1dJ
/// +bDwagZWZ9hNXARTq4K0uxw6Ol5jGD9Od6w5n5uxyaEk9/edvYwMhthIxC/uADRp
/// 2pNSutwyLX3bUJnHZwIDAQAB
/// -----END PUBLIC KEY-----";
/// ```
///
pub fn parse_public_key_pem(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut result = String::from("-----BEGIN PUBLIC KEY-----\n");
    for (i, item) in bytes.iter().enumerate() {
        result.push(*item as char);
        if (i + 1) % 64 == 0 {
            result.push('\n')
        }
    }
    result.push_str("\n-----END PUBLIC KEY-----");

    result
}

/// Return MD5 Hex string
pub fn md5<T: Into<String>>(input: T) -> String {
    let md5 = md5::compute(input.into());
    format!("{:x}", md5)
}

/// Encrypt password by `PKCS1v15(MD5(<password>))`
///
/// **Input** `Base64` encoded public key string likes:
///
/// ```
/// # let raw_pub =
/// "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC9XrJWcWbj0LhDBzN4uwEOLA/U\
/// JKsmCkkbvlVgN/qei3e/jVFpxR6D3fzshnv5QNB4+BJ/rjRWbbxCJ0djzPxsLS1dJ\
/// +bDwagZWZ9hNXARTq4K0uxw6Ol5jGD9Od6w5n5uxyaEk9/edvYwMhthIxC/uADRp\
/// 2pNSutwyLX3bUJnHZwIDAQAB";
/// ```
///
/// **Return** `Base64` encoded string
pub fn encrypt_password(pwd: &str, pub_base64: &str) -> Result<String, Error> {
    let pub_der = general_purpose::STANDARD.decode(pub_base64)?;
    let public_key = RsaPublicKey::from_public_key_der(&pub_der)?;

    let mut rng = rand::thread_rng(); // Random generator
    let pass_md5 = md5(pwd);

    let encrypted = Pkcs1v15Encrypt.encrypt(&mut rng, &public_key, pass_md5.as_bytes())?;

    let result = general_purpose::STANDARD.encode(encrypted);
    Ok(result)
}

/// Write string to file
///
/// Auto create file
pub fn file_write(path: &str, s: &str) -> Result<(), Error> {
    let mut f = std::fs::File::create(path)?;
    f.write_all(s.as_bytes())?;
    Ok(())
}

/// PKCS#7 Padding method
pub fn pkcs7_padding(message: &str, block_size: usize) -> String {
    let padding_size = block_size - message.len() % block_size;
    let padding_char = padding_size as u8 as char;
    let padding: String = (0..padding_size).map(|_| padding_char).collect();
    format!("{}{}", message, padding)
}

const LOWER_CHAR_NUM_LIST: &[u8; 36] = b"abcdefghijklmnopqrstuvwxyz1234567890";

/// Random md5-style string generator
pub fn gen_random_fake_md5() -> String {
    let mut buf = Vec::with_capacity(32);
    for _ in 0..32 {
        buf.push(LOWER_CHAR_NUM_LIST[rand::thread_rng().gen_range(0..36)]);
    }
    unsafe { String::from_utf8_unchecked(buf) }
}

#[cfg(test)]
mod test {
    use base64::{engine::general_purpose, Engine};
    use rand::rngs::ThreadRng;
    use rsa::{pkcs1::DecodeRsaPrivateKey, RsaPrivateKey};

    use super::*;

    #[test]
    fn test_md5() {
        assert_eq!(
            md5("abcdefghijklmnopqrstuvwxyz"),
            "c3fcd3d76192e4007dfb496cca67e13b"
        )
    }

    #[test]
    fn test_gen_md5() {
        let md5 = gen_random_fake_md5();
        println!("{}", md5);
        assert_eq!(md5.len(), 32);
    }

    #[test]
    fn test_encrypt_password() -> Result<(), Error> {
        let private_key = "\
-----BEGIN RSA PRIVATE KEY-----
MIICWwIBAAKBgQC9XrJWcWbj0LhDBzN4uwEOLA/UJKmCkkbvlVgN/qei3e/jVFpx
R6D3fzshnv5QNB4+BJ/rjRWbbxCJ0djzPxsLS1dJ+bDwagZWZ9hNXARTq4K0uxw6
Ol5jGD9Od6w5n5uxyaEk9/edvYwMhthIxC/uADRp2pNSutwyLX3bUJnHZwIDAQAB
AoGANN3S+7788my6hDvmarYKPWKfqKHzkLg1hX0z7/Q/6H/9EIHkHevZTD8AywoQ
BWQHbVjtLF1ewt3myBMFdiMP8UOx0WVErcyuVRh8AUcRZIEwz73jmLmpRd8fVAzy
8uoijKvExt/fdu9aIfVmV4nXvL5dDpsoL/mVRDgNCZ+9mMECQQDzWLnqty25mgEs
73rJ8mhehifwblg44uO+9xpmKZhG3NFZW+beG1iPZklBVlaQ6m53e77VbVotC+LF
efsaOtU7AkEAxzd3q0REhF/FaFcq9TV3Eu3C4B/aqARKgkpJKiaCC4tnAqny7Rvd
/anxLBf8DFPYjPMkPrNqXoDA8rAC9TwDxQJBAPF6mHOMdvl5E7WNp6GCxYMXScbT
GQTKUgoMl8vNdujK84vjIMRDCqyyaftGO/zuRdSXnZWZQCT3aH9iPoWW4EUCQB1r
NYLXK/8YXYCRDsjzQkhLUDHkwld5er9O1QsicKXfyjB8hGE7ckbZZ8IJMLFpWFtI
NJwFxrl57gRotacdW7kCP2r3MkJqtHdrjUbaCJJCnHmX9BhYcBhaYS2yGFW9uyNT
5TGOrrzjz+CXBNrif3JkDbDYv2z/cCgd7kqV1kPl/g==
-----END RSA PRIVATE KEY-----";

        let raw_public_key = "\
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC9XrJWcWbj0LhDBzN4uwEOLA/U\
JKmCkkbvlVgN/qei3e/jVFpxR6D3fzshnv5QNB4+BJ/rjRWbbxCJ0djzPxsLS1dJ\
+bDwagZWZ9hNXARTq4K0uxw6Ol5jGD9Od6w5n5uxyaEk9/edvYwMhthIxC/uADRp\
2pNSutwyLX3bUJnHZwIDAQAB";

        // MD5("123456") = "e10adc3949ba59abbe56e057f20f883e"
        let cipher_text = encrypt_password("123456", raw_public_key)?;

        let private = RsaPrivateKey::from_pkcs1_pem(private_key).unwrap();

        let dec_data = Pkcs1v15Encrypt.decrypt(
            None::<&mut ThreadRng>,
            &private,
            &general_purpose::STANDARD.decode(cipher_text)?,
        )?;

        let dec_text = String::from_utf8(dec_data).unwrap();

        assert_eq!(dec_text, "e10adc3949ba59abbe56e057f20f883e");

        Ok(())
    }
}
