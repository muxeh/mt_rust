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
    let mut index = 0;
    let data = "Genesis Block".to_string();
    let previous_hash = "0".to_string();
    // Call miner to mine a new block
    let genesis = mine_block(index, data, previous_hash);
    let mut blockchain = Blockchain::new(genesis);

    println!("🧱 Added genesis to the blockchain: {:#?}", blockchain.latest());
    index = index +1;

    let max_or_tyler = ["max mined a block!", "Tyler Mined a block!"];
    let mut who_done_it = 0;
    loop {
        let previous_hash: String = blockchain.latest().hash.clone();
        let data = max_or_tyler[who_done_it % 2].to_string();
        let new_block = mine_block(index, data, previous_hash);
        if blockchain.try_add_block(new_block) {
            println!("🧱 We Mined a Block!!: {:#?}", blockchain.latest());
            index += 1;
            who_done_it += 1;
        } else {
            println!("❌ Block rejected!");
        }
    }
    
}