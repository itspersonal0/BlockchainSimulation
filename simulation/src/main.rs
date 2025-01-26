use blockchain_simulation::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new(4);

    println!("Mining first block...");
    blockchain.add_transaction(
        String::from("Alice"),
        String::from("Bob"),
        50.0,
    );
    blockchain.add_transaction(
        String::from("Bob"),
        String::from("Charlie"),
        30.0,
    );
    blockchain.mine_pending_transactions(String::from("miner1"));

    println!("Mining second block...");
    blockchain.add_transaction(
        String::from("Charlie"),
        String::from("David"),
        20.0,
    );
    blockchain.add_transaction(
        String::from("David"),
        String::from("Alice"),
        15.0,
    );
    blockchain.mine_pending_transactions(String::from("miner1"));

    println!("\nBlockchain contents:");
    blockchain.print_blockchain();

    println!("\nIs blockchain valid? {}", blockchain.is_chain_valid());
}
