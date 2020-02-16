use serde::{Deserialize, Serialize};
extern crate hex;
extern crate cryptonight;
use cryptonight::cryptonight;

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
    pub nonce: u8,
    pub signature: String,
}

impl Transaction {
    fn typeTransaction(&self) -> String {
        return match (self.extra) {
            "" => "normal",
            "r" => "reward",
            "fnr" => "fullnode registration",
            "unr" => "username registraion",
            "l" => "fund lock",
            "b" => "burn",
            _ => "message",
        };
    }

    fn validateTransaction(&self) -> bool {
        /* assume ammount is correct for benchmark
        let mut acc = getAccount(self.sender_key);
        if acc.balance == 0 {
            return false;
        }
        */
        if self.amount < 0.0001 {
            // the min amount sendable (1 miao)
            return false;
        }
        if self.access_key != sender_key {
            if  100000 < self.amount {
                return false;
            else if self.hash() != self.hash {
                return false;
            }
            else if self.extra.len() > 100 {
                return false;
            }
            } else {
                if self.signature != "" {
                    return true;
                }
            }
        }
    }
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.ammount);
        bytes.extend(self.extra);
        bytes.extend(self.flag);
        bytes.extend(self.sender_key);
        bytes.extend(self.receive_key);
        bytes.extend(self.gas * self.gas_price); // aka fee
        bytes.extend(self.nonce);
        bytes
    }
    fn hash(&self) {
        let asbytes = self.bytes();
        let out = cryptonight(&asbytes, asbytes.len(), 0);
        self.hash = hex::encode(out);
}
