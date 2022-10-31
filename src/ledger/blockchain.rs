use std::{cmp};
use crate::{wallet::{transaction::Transaction}, hashtools::{hashtools::Hashtools}};
use super::{block::Block};

const TRANSACTIONS_PER_BLOCK: usize = 5;
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub transaction_pool: Vec<Transaction>,
    pub stake_pool: Vec<String>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis = Block::genesis();
        let mut blocks = vec![];
        blocks.push(genesis);
        Blockchain{
            blocks,
            transaction_pool: vec![],
            stake_pool: vec![]
        }
    }
    
    pub fn get_balance(&self, public_key: &String)-> f64 {
        let mut balance = 0.0;
        for b in &self.blocks{
            for t in &b.transaction_list{
                if t.recipient_public_key == public_key.to_owned() {
                    balance += t.amount;
                }else if t.sender_public_key == public_key.to_owned(){
                    balance -= t.amount + t.fee;
                }
            }
        }
        balance 
    }

    pub fn print_block(&self, i: usize) -> String{
        if i >= self.blocks.len(){
            return String::from("Index out of range")
        }else{
            return self.blocks[i].to_string()
        }
        
    }

    pub fn get_last_block(&self) -> &Block{
        let result = &self.blocks[self.blocks.len()-1];
        result
    }

    pub fn get_transaction_pool(&mut self) -> Vec<Transaction>{
        let n: usize = cmp::min(TRANSACTIONS_PER_BLOCK, self.transaction_pool.len());
        let transactions = self.transaction_pool.drain(0..n).collect();
        transactions
    }

    pub fn validate_block_hash(b: &Block) -> bool{
        let mut transaction_string: String = String::from("");
        for t in &b.transaction_list {
            transaction_string.push_str(&t.to_string());
        }
        let rehash: String = Hashtools::create_hash(format!("{}{}{}{}{}", b.index, b.time_stamp.to_string(), b.previous_hash, b.reward, transaction_string));
        rehash.eq(&b.current_hash)
    }

    pub fn to_string(&self) -> String{
        let mut string: String = String::from("");
        string.push_str("========Blockchain Start========\n");
        for b in &self.blocks {
            string.push_str(&b.to_string());
            string.push_str("\n");
        }
        string.push_str("========Blockchain End========");
        string
    } 
}


