mod block;
mod blockchain;
mod miner;
mod network;
mod transaction;
mod wallet;
use miner::mine_block;
use crate::blockchain::Blockchain;


fn main() {
    // Example setup
    let index = 0;
    let data = "Genesis Block".to_string();
    let previous_hash = "0".to_string();
    // Call miner to mine a new block
    let genesis = mine_block(index, data, previous_hash);
    let mut blockchain = Blockchain::new(genesis);

    println!("🧱 Added genesis to the blockchain: {:#?}", blockchain.latest());
}