extern crate rand;
use core::num::flt2dec::Sign;

use crate::{ledger::blockchain::{Blockchain}, hashtools::hashtools};
use cursive::event::Key;
use ed25519_dalek::{self, Keypair, SecretKey, PublicKey, Signer, Signature, Verifier};
use rand::rngs::OsRng;

pub struct Wallet{
    pub public_key: String,
    pub private_key: String
}

impl Wallet {
    /*
    Generates new key pair and assigns public and private keys
    */
    pub fn new() -> Self{
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);
        Self{
            public_key: hashtools::Hashtools::to_hex_string(&keypair.secret.to_bytes()),
            private_key: hashtools::Hashtools::to_hex_string(&keypair.public.to_bytes()),
        } 
    }

    pub fn create_signature(public_key: &String, private_key: &String,  hash: &String) -> String{
        let keypair =  Keypair{ 
            secret: SecretKey::from_bytes(&hashtools::Hashtools::to_byte_array(private_key)).expect("Invalid Secret Key"), 
            public: PublicKey::from_bytes(&hashtools::Hashtools::to_byte_array(public_key)).expect("Invalid Public Key"), 
        };
        let signature = hashtools::Hashtools::to_hex_string(&keypair.sign(hash.as_bytes()).to_bytes());
        signature
    }
    
    pub fn validate_signature(public_key: &String, private_key: &String,  hash: &String, signature: &String) -> Bool{
        let public_key = PublicKey::from_bytes(&hashtools::Hashtools::to_byte_array(public_key)).expect("Invalid Public Key");
        let signature_bytes = hashtools::Hashtools::to_byte_array(&signature);
        let sig: Signature = Signature::try_from(&signature_bytes).expect("Invalid Signature");
        public_key.verify(hash.as_bytes(), &sig).is_ok();
    }

    pub fn validate_private_key(public_key: &String, private_key: &String) -> String{
        // let test_hash: &[u8] = b"0000abc1e11b8d37cle1232a2ea6d290cddb0c678058c37aa766f813cbbb366e";
        // let sig: Vec<u8> = Self::create_signature(&keys, test_hash);
        // Self::validate_signature(&keys, test_hash, &sig)
        String::from("validate private key")
    }

    pub fn to_string(&self, bc: &Blockchain) -> String{
        format!("===Wallet Info===\nPublic Key: {:x?}\nPrivate Key: {:x?}\nBalance: {}\n===Wallet Info End===", 
        &self.public_key, 
        &self.private_key, 
        bc.get_balance(&self.public_key))
    }

}