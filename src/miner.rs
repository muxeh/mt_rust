use crate::block::Block;
use crate::blockchain::Blockchain;

use chrono::Utc;




//find a valid block 
pub fn mine_block( index: u64, data: String, previous_hash: String) -> Block {
    let mut nonce = 0;
    loop {

            let timestamp = Utc::now().to_rfc3339();
            let block = Block::new(timestamp,index, data.clone(), previous_hash.clone(), nonce);
    
            if block.is_valid() {
                return block;
            }
            nonce += 1;
        }



}

//now we need to broadcast the block to the network 
pub fn broadcast_block(block:Block){

}