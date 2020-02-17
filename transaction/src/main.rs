use serde::{Deserialize, Serialize};
extern crate hex;
extern crate cryptonight;
use cryptonight::cryptonight;
extern crate rand;
use rand::Rng;
use std::time::{Duration, Instant};
use indicatif::ProgressBar;                                                                                       

static TC:u64 = 200;                                                                                              


static tc: u64 = 200;

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
fn gen(amount: u64) -> Vec<Transaction> {
    let mut i: u64 = 0;
    let mut txns: Vec<Transaction> = vec![];
    let mut rng = rand::thread_rng();
    let pb = ProgressBar::new(tc);                                                                                
    println!("Generating {:?} Transactions", TC);                                                                 
    while i < amount {
        let mut txn = Transaction { 
            hash: String::from("hash"),
            amount: rng.gen(), 
            extra: String::from(""), 
            flag: 'n', 
            sender_key: (hash(String::from("skey...".to_owned() + &rng.gen::<u64>().to_string()))),
            receive_key: (hash(String::from("rc".to_owned() + &rng.gen::<u64>().to_string()))),
            access_key: (hash(String::from("sk".to_owned() + &rng.gen::<u64>().to_string()))),
            gas_price: rng.gen::<u16>() as u64,
            max_gas: rng.gen::<u16>() as u64,
            gas: rng.gen::<u16>() as u64,
            nonce: rng.gen(),
            signature: String::from(hash(String::from("sig...".to_owned() + &rng.gen::<u64>().to_string()))),
        };
        txn.hash();
        pb.inc(1);
        txns.push(txn);
        i += 1;
    }
    pb.finish_with_message("Generated Transactions.");                                                       
    return txns;
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
                if self.signature != "" {
                    return true;
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

fn main() {
    println!("Avrio Transaction Benchmark Version 0.1.0");
    println!("Enter Number Of Txns To Generate And Validate");
    println!("Generating {:?} txns", TC);
    let txns = gen(TC);
    println!("Done");
    let now = Instant::now();
    for tx in txns {
        let out = &"Tx ".to_owned() + tx.hash + &" valid: ".to_owned() + tx.validateTransaction() as i32;
        println!("{:?}", out);
        for _ in 0..=out.len() {
            print!("{}", (8u8 as char));
        }
    }
    println!("Validated {:?} Transactions In {:?} Milliecconds. {:?} TPS", TC, now.elapsed().as_millis(), now.elapsed().as_millis() / (tc as u128));
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
