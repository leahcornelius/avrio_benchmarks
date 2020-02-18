use serde::{Deserialize, Serialize};
extern crate hex;
extern crate cryptonight;
use cryptonight::cryptonight;
extern crate rand;
use rand::Rng;
use std::time::{Duration, Instant};
use indicatif::ProgressBar;
use ring::{
    rand as randc,
    signature::{self, KeyPair},
};
#[macro_use]
extern crate log;
extern crate simple_logger;
                                                                                              

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    info!("Avrio Transaction Benchmark Version 0.1.0");
    let mut trans_count:u64 = 9;
    loop {
    trans_count += 1;
    let TC: u64 = trans_count;
    info!("Generating {:?} txns", TC);
    let txns = gen(TC).unwrap();
    info!("Done");
    let now = Instant::now();
    let mut i:u64 = 0;
    for tx in txns {
        i += 1;
        let result:i32 = tx.validateTransaction() as i32;
        let out: String;
        if result == 1 {
            out = "Tx ".to_owned() + &i.to_string()  + &"/".to_owned() + &TC.to_string() + &" Hash: ".to_owned() + &tx.hash + &" valid: ".to_owned() + &((result).to_string());
            info!("{}", out);
        } else {
            out = "Tx ".to_owned() + &i.to_string()  + &"/".to_owned() + &TC.to_string() + &" Hash: ".to_owned() + &tx.hash + &" valid: ".to_owned() + &((result).to_string()) + &" invalid".to_owned();
            warn!("{}", out);
        }
        for _ in 0..=out.len() {
            print!("{}", (8u8 as char));
        }
    }
    println!("");
    info!("Validated {:?} Transactions In {:?} Secconds. {:?} TPS", TC, now.elapsed().as_millis() +1 / 1000, ((TC as u64)/ (now.elapsed().as_millis() as u64) /1000));

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub hash: String,
    pub amount: u64,
    pub extra: String,
    pub flag: char,
    pub sender_key: String,
    pub receive_key: String,
    pub access_key: String,
    pub gas_price: u64,
    pub max_gas: u64,
    pub gas: u64, // gas used
    pub nonce: u64,
    pub signature: String,
}
fn gen(amount: u64) -> Result<Vec<Transaction>, ()> {
    let mut i: u64 = 0;
    let mut txns: Vec<Transaction> = vec![];
    let mut rng = rand::thread_rng();
    let rngc = randc::SystemRandom::new();
    let pb = ProgressBar::new(amount);                                                                                
    info!("Generating {:?} Transactions", amount);                                                                 
    while i < amount {
        let mut txn = Transaction { 
            hash: String::from(""),
            amount: rng.gen(), 
            extra: String::from(""), 
            flag: 'n', 
            sender_key: String::from(""),
            receive_key: (hash(String::from("rc".to_owned() + &rng.gen::<u64>().to_string()))),
            access_key: String::from(""),
            gas_price: rng.gen::<u16>() as u64,
            max_gas: rng.gen::<u16>() as u64,
            gas: rng.gen::<u16>() as u64,
            nonce: rng.gen(),
            signature: String::from(""),
        };
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rngc).unwrap();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let peer_public_key_bytes = key_pair.public_key().as_ref();
        txn.sender_key = hex::encode(peer_public_key_bytes);
        txn.hash();
        // Sign the hash
        let msg: &[u8] = txn.hash.as_bytes();
        txn.signature = hex::encode(key_pair.sign(msg));
        let peer_public_key =
        signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
        //peer_public_key.verify(msg, hex::decode(&txn.signature.to_owned()).unwrap().as_ref()).unwrap();
        pb.inc(1);
        txns.push(txn);
        i += 1;
    }
    pb.finish_with_message("Generated Transactions.");                                                       
    return Ok(txns);
}
impl Transaction {
/*    fn typeTransaction(&self) -> String {
        return match (self.extra) {
            "" => "normal",
            "r" => "reward",
            "fnr" => "fullnode registration",
            "unr" => "username registraion",
            "l" => "fund lock",
            "b" => "burn",
            _ => "message",
        };
    }*/

    fn validateTransaction(&self) -> bool {
        /* assume ammount is correct for benchmark
        let mut acc = getAccount(self.sender_key);
        if acc.balance == 0 {
            return false;
        }
        */
        if self.amount < 1 {
            // the min amount sendable (1 miao)
            return false;
        }
        if self.access_key != self.sender_key {
            if  0 > self.amount {
                return false;
            } else if self.hashReturn() != self.hash {
                return false;
            }
            else if self.extra.len() > 100 {
                return false;
            }
             else {
                 let peer_public_key_bytes = hex::decode(&self.sender_key.to_owned()).unwrap();
                 let peer_public_key =
                    signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
                match peer_public_key.verify(self.hash.as_bytes(), &hex::decode(&(self.signature).to_owned()).unwrap()).unwrap() {
                    () => return true,
                    _ => return false,
                 }
            }
        }
    return true;
    }
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend((self.amount.to_string()).bytes());
        bytes.extend((self.extra.to_owned()).bytes());;
        bytes.extend(self.flag.to_string().bytes());
        bytes.extend(self.sender_key.bytes());
        bytes.extend(self.receive_key.bytes());
        bytes.extend(((self.gas * self.gas_price.to_owned()).to_string()).bytes()); // aka fee
        bytes.extend((self.nonce.to_owned().to_string()).bytes());
        bytes
    }
    fn hash(&mut self) {
        let asbytes = self.bytes();
        unsafe {
            let out = cryptonight(&asbytes, asbytes.len(), 0);
        
            self.hash = hex::encode(out);
        }
    }
    fn hashReturn(&self) -> String {
        let asbytes = self.bytes();
        unsafe {
            let out = cryptonight(&asbytes, asbytes.len(), 0);
            return hex::encode(out);
        }
    }
}                                                                     

fn hashBytes(asbytes: Vec<u8>) -> String{;
    unsafe {
        let out = cryptonight(&asbytes, asbytes.len(), 0);
        return hex::encode(out);
    }
}

fn hash(subject: String) -> String {
    let asBytes = subject.as_bytes();
    unsafe {
        let out = cryptonight(&asBytes, asBytes.len(), 0);
        return hex::encode(out);
    }
} 
