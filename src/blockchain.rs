use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    // I dont think this should permanantly be a public function. This would allow everyone to start a new chain? 
    pub fn new(genesis: Block) -> Self {
        Blockchain { 
 
            chain: vec![genesis] 
            
        }
    }

    // We wont need to change this
    pub fn latest(&self) -> &Block {
        self.chain.last().unwrap()
    }




    // this feels like its not exactly what we should do. I think what we will do here is in miner.rs try to add a block, but the issue is that 
    // it doesnt account for any nodes validating the block before adding it. We will need to change this later 

    pub fn try_add_block(&mut self, block: Block) -> bool {
        let latest = self.latest();
        if block.previous_hash == latest.hash && block.is_valid() {
            self.chain.push(block);
            true
        } else {
            false
        }
    }

    // unclear how we are using this but I think different nodes will need to call this. 
    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if current.previous_hash != previous.hash || !current.is_valid() {
                return false;
            }
        }
        true
    }
}