use chrono::Utc;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    index: u64,
    timestamp: i64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp(),
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            serde_json::to_string(&self.transactions).unwrap(),
            self.previous_hash,
            self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut chain = Vec::new();
        chain.push(Block::new(0, Vec::new(), String::from("0")));
        
        Blockchain {
            chain,
            difficulty,
            pending_transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, sender: String, recipient: String, amount: f64) {
        self.pending_transactions.push(Transaction {
            sender,
            recipient,
            amount,
        });
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let transactions = self.pending_transactions.clone();
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut block = Block::new(self.chain.len() as u64, transactions, previous_hash);
        
        block.mine_block(self.difficulty);
        self.chain.push(block);

        self.pending_transactions = vec![Transaction {
            sender: String::from("system"),
            recipient: miner_address,
            amount: 10.0,
        }];
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    pub fn print_blockchain(&self) {
        for block in &self.chain {
            println!("\nBlock #{}", block.index);
            println!("Timestamp: {}", block.timestamp);
            println!("Previous Hash: {}", block.previous_hash);
            println!("Hash: {}", block.hash);
            println!("Transactions: {}", serde_json::to_string_pretty(&block.transactions).unwrap());
            println!("Nonce: {}", block.nonce);
        }
    }
}
