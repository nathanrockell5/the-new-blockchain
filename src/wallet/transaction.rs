use chrono::{DateTime, Utc};
use crate::hashtools::{hashtools::{Hashtools}};
use super::wallet::{self, Wallet};

pub struct Transaction{
    pub hash: String,
    pub time_stamp: DateTime<Utc>,
    
    pub sender_public_key: String,
    pub recipient_public_key: String,
    pub signature: String,
    
    pub amount: f64,
    pub fee: f64,
}

impl Transaction{ 
    pub fn new(sender_public_key: String, sender_private_key: String, recipient_public_key: String, amount: &f64) -> Self{
        let time_stamp: DateTime<Utc> = Utc::now();
        let fee: f64 =  amount * 0.01;
        let hash = Hashtools::create_hash(format!("{}{}{}{}{}", 
        sender_public_key, 
        recipient_public_key, 
        time_stamp.to_string(), 
        amount.to_string(),             
        fee.to_string())
        );

        let signature: String = wallet::Wallet::create_signature(&sender_public_key, &sender_private_key, &hash);
        Self {
            hash,
            time_stamp, 
            sender_public_key,
            recipient_public_key,
            signature,            
            amount: *amount,
            fee,
        }
    }
    
    pub fn to_string(&self) -> String{
        format!("=== Transaction ===\nHash: {}\nTime Stamp: {}\nSignature: {}\nAmount: {}\nFee: {}\nSender Address: {}\nRecipient Address: {}\n=== End Transaction ===", 
        self.hash, 
        self.time_stamp.to_string(), 
        self.signature, 
        self.amount.to_string(), 
        self.fee.to_string(), 
        &self.sender_public_key, 
        &self.recipient_public_key)
    }
}