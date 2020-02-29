use avrio_blockchain::Block;
use avrio_core::Transaction;
use avrio_database::{getData, saveData};
use cryptonight::cryptonight;
pub struct Chain {
  public_key: String,
  blocks: Vec<Block>,
  digest: String,
}
const CHAIN_COUNT: u8 = 10;
const BLOCKS_PER_CHAIN: u8 = 10;
fn main() {
  let mut chains: Vec<Chain> = vec![];
  for i in 0..= CHAIN_COUNT {
    chains.push(Chain {
      public_key: cryptonight("cisuhbfiw".to_string() + &i.to_string()),
      blocks: vec![],
      digest: "".to_string(),
    });
    for block_i in 0..=BLOCKS_PER_CHAIN {
      chains[i].blocks.push(
        Block::default
      )
    }
  }
  for chain in chains {
    for block in chain.blocks {
      saveData(
        serde_json::to_string(&block),
        "./".to_string() + &block.chain_key + ".db".to_string(),
        block.hash,
      );
    }
  }
}
