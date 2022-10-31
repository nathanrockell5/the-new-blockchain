use std::{time::{SystemTime}};
use chrono::{DateTime, Utc};
use crate::{hashtools::{hashtools::{Hashtools}}, wallet::transaction::{Transaction}};

pub struct Block {
    pub index: u32,
    pub time_stamp: DateTime<Utc>,
    pub current_hash: String,
    pub previous_hash: String,
    pub reward: f64,
    pub transaction_list: Vec<Transaction>,
    pub validator_list: Vec<String>,
    // pub merkleRoot: String,
}

impl Block {
    // //Ran when new blockchain is initiated. 
    pub fn genesis() -> Self {
        let time_stamp = Utc::now();
        Self {
            index: 0,
            time_stamp,
            current_hash: Hashtools::create_hash(format!("{}{}{}", 0, time_stamp.to_string(), "")),
            previous_hash: String::from(""),
            reward: 50.0,
            transaction_list: vec![],
            validator_list: vec![],
        }
    }

    pub fn new(last_block: &Block, transaction_list: Vec<Transaction>, validator_list: Vec<String>) -> Self {
        let index = last_block.index+1;
        let time_stamp: DateTime<Utc> = SystemTime::now().into();
        
        let mut transaction_string: String = String::from("");
        for t in &transaction_list {
            transaction_string.push_str(&t.to_string());
        }
        let current_hash = Hashtools::create_hash(format!("{}{}{}{}{}", index, time_stamp.to_string(), last_block.current_hash, 50.0, transaction_string));
        Self { 
            index, 
            time_stamp, 
            current_hash, 
            previous_hash: last_block.current_hash.clone(), 
            reward: 50.0,
            transaction_list,
            validator_list,
        }        
    }
    pub fn mine_and_forge(&self){

    }



    /* 
    pub fn create_validator_rewards_transaction(&self, rewards_wallet: &Wallet) -> Vec<Transaction>{
        let mut fee: f64 = 0.0;
        for t in &self.transaction_list{
            fee += t.fee
        }
        let mut transactions: Vec<Transaction> = vec![];
        for v in &self.validator_list{
            transactions.push(Transaction::new(String::from("Validator Rewards"), String::from(""), v.staker_public_key, &(fee/VALIDATOR_LIST_LENGTH+REWARD_VALUE)));
        }
        return transactions
    }
    pub fn merkle_root(&mut self) -> String{
        let mut hashes: Vec<String> = vec![];
        for t in &self.transaction_list{
            hashes.push(t.hash.clone());
        }

        if hashes.len() == 0 {
            return String::from("");
        }
        if hashes.len() == 1 {
            return hashtools::hashtools::Hashtools::combine_hash(&hashes[0], &hashes[0]);
        }
        while hashes.len() !=1 {
            let mut merkle_branches: Vec<String> = vec![];
            for i in (0..hashes.len()).step_by(2){
                if i == hashes.len()-1{
                    merkle_branches.push(hashtools::hashtools::Hashtools::combine_hash(&hashes[i], &hashes[i]))
                }else{
                    merkle_branches.push(hashtools::hashtools::Hashtools::combine_hash(&hashes[i], &hashes[i+1]))
                }
            }
            hashes = merkle_branches;
        }
        return String::from("Merkle Root");
    } 
    */
    pub fn to_string(&self) -> String{
        return format!("====== Block Start ======\nIndex: {}\nTime Stamp: {}\nCurrent Hash: {}\nPrevious Hash: {}\nValidator Reward: {}\n====== Block End ======", self.index, self.time_stamp.to_string(), self.current_hash, self.previous_hash, self.reward)
    }
}