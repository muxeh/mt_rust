mod block;
mod blockchain;
mod miner;
mod network;
mod transaction;
mod wallet;

fn main() {
    println!("RustChain CLI running...");
    //testing
    // Create a block
    let mut block = block::Block::new(0, "My Block".to_string(), "0x00".to_string());
    println!("My Block:\n{}", block);
    // Example CLI stub
    // let mut blockchain = Blockchain::new();
    // blockchain.add_block("Some data".to_string());
}
