mod xor;
mod rsa;

use rand::Rng;
use base64::{Engine as _, engine::general_purpose};

use xor::*;

// 16 bytes
type Key = [u8; 16];

pub fn gen_key() -> Key {
    rand::thread_rng().gen::<Key>()
}

pub fn encode(input: &[u8], key: &[u8; 16]) -> String {
    let mut buf: Vec<u8> = [input, key].concat();
    xor_transform(&mut buf[16..], &xor_derive_key(key, 4));
    buf.reverse();
    xor_transform(&mut buf[16..], &XOR_CLIENT_KEY);
    let my_rsa = rsa::MyRSA::new();
    general_purpose::STANDARD.encode(my_rsa.encrypt(&buf))
}

pub fn decode(input: &str, key: &[u8; 16]) -> Result<Vec<u8>, base64::DecodeError> {
    let my_rsa = rsa::MyRSA::new();
    let buf = general_purpose::STANDARD.decode(input)?;
    let buf = my_rsa.decrypt(&buf);
    let mut output = buf[16..].to_vec();
    xor_transform(&mut output, &xor_derive_key(&buf[..16], 12));
    output.reverse();
    xor_transform(&mut output, &xor_derive_key(key, 4));
    Ok(output)
}