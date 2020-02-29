extern crate avrio_blockchain;
extern crate avrio_core;
use avrio_blockchain::{Block, Header};
use avrio_core::transaction::Transaction;
use avrio_database::{getData, saveData};
use cryptonight::cryptonight;
extern crate hex;
pub struct Chain {
    public_key: String,
    blocks: Vec<Block>,
    digest: String,
}

const CHAIN_COUNT: u8 = 10;
const BLOCKS_PER_CHAIN: u8 = 10;

fn main() {
    let mut chains: Vec<Chain> = vec![];
    let i: usize = 0;
    for i in 0..=CHAIN_COUNT {
        let mut chain_thing = Chain {
                public_key: hex::encode(cryptonight("chain-".to_string() + &i.to_string().as_bytes(), cryptonight("chain-".to_string() + &i.to_string().to_bytes().len(), 0),
                blocks: vec![],
                digest: "".to_string(),
             }
        chains.push(
            chain_thing
          );
        let block_i: usize = 0;
        for block_i in 0..=BLOCKS_PER_CHAIN {
            let ts: u64 = (block_i + 1000000) as u64,
            chains[i as usize].blocks.push(Block {
                header: Header {
                    version_major: 0,
                    version_breaking: 0,
                    version_minor: 1,
                    chain_key: chains[i as usize].public_key,
                    prev_hash: hex::encode(cryptonight("prevhashforblock".to_string() + &i.to_string().as_bytes(), cryptonight("chain-".to_string() + &i.to_string().as_bytes().len(), 0).to_owned(),
                    height: block_i as u64,
                    timestamp: ts,
                },
                hash: "".to_string(),
                txns: vec![Transaction::default(); 5],
                signature: "".to_string(),
                node_signatures: vec!["".to_string(); 11],
            });
            chains[i as usize].blocks[block_i as usize].hash();
            println!(
                "generated block: {} / {} for chain: {}",
                block_i, BLOCKS_PER_CHAIN, chains[i as usize].public_key
            );
        }
    }
    for chain in chains {
        for block in chain.blocks {
            saveData(
                serde_json::to_string(&block).unwrap(),
                "./".to_string() + &block.header.chain_key + &".db".to_string(),
                block.hash.clone(),
            );
            println!(
                "Block: {:?}",
                getData(
                    "./".to_string() + &block.header.chain_key + &".db".to_string(),
                    block.hash.clone(),
                )
            );
        }
    }
}
