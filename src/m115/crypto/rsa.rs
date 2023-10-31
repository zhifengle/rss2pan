use num_bigint::{BigInt, Sign};
use rand::Rng;

pub struct MyRSA {
    n: BigInt,
    e: BigInt,
    key_size: usize,
}

impl MyRSA {
    pub fn new() -> Self {
        let n: BigInt = BigInt::parse_bytes(b"8686980c0f5a24c4b9d43020cd2c22703ff3f450756529058b1cf88f09b8602136477198a6e2683149659bd122c33592fdb5ad47944ad1ea4d36c6b172aad6338c3bb6ac6227502d010993ac967d1aef00f0c8e038de2e4d3bc2ec368af2e9f10a6f1eda4f7262f136420c07c331b871bf139f74f3010e3c4fe57df3afb71683", 16).unwrap();
        let e: BigInt = BigInt::parse_bytes(b"10001", 16).unwrap();
        let (_, l) = n.to_bytes_be();
        Self {
            n,
            e,
            key_size: l.len(),
        }
    }
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        for chunk in data.chunks(self.key_size - 11) {
            self.encrypt_slice(chunk, &mut buf);
        }

        buf
    }
    fn encrypt_slice(&self, input: &[u8], buf: &mut Vec<u8>) {
        // padding
        let pad_size = self.key_size - input.len() - 3;
        let mut pad_data: Vec<u8> = vec![0; pad_size];
        rand::thread_rng().fill(pad_data.as_mut_slice());
        // Prepare message
        let mut msg_buf: Vec<u8> = vec![0; self.key_size];
        msg_buf[1] = 2;
        for (i, v) in pad_data.iter().enumerate() {
            msg_buf[i + 2] = v % 0xff + 0x01;
        }
        msg_buf[pad_size + 2] = 0;
        msg_buf[pad_size + 3..].clone_from_slice(input);
        let msg = BigInt::from_bytes_be(Sign::Plus, msg_buf.as_slice());
        // RSA Encrypt
        let (_, ret) = msg.modpow(&self.e, &self.n).to_bytes_be();
        if self.key_size > ret.len() {
            let start_len = self.key_size - ret.len();
            buf.extend(vec![0; start_len]);
        }
        buf.extend(ret);
    }
    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        for chunk in data.chunks(self.key_size) {
            self.decrypt_slice(chunk, &mut buf);
        }
        buf
    }
    fn decrypt_slice(&self, input: &[u8], buf: &mut Vec<u8>) {
        let msg = BigInt::from_bytes_be(Sign::Plus, input);
        let (_, ret) = msg.modpow(&self.e, &self.n).to_bytes_be();
        for (i, b) in ret.iter().enumerate() {
            if *b == 0 && i != 0 {
                buf.extend(&ret[i + 1..]);
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_encrypt() {
        let rsa = MyRSA::new();
        let r = rsa.encrypt(b"test input");
        println!("{:?}", r);
    }
    #[test]
    fn t_bigint() {
        // let r = BigInt::from(2).modpow(&BigInt::from(16), &BigInt::from(6000));
        // println!("{}", BigInt::parse_bytes(b"10001", 16).unwrap());
        let msg_buf = [0, 2];
        let r = BigInt::from_bytes_be(Sign::Plus, msg_buf.as_slice());
        println!("{}", r);
    }
}