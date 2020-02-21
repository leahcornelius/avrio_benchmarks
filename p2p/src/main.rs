#[macro_use]
extern crate log;
#[macro_use]
use serde::{Deserialize, Serialize};
#[macro_use]
extern crate unwrap;
//extern crate avrio_config;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::thread;
use std::str;
extern crate hex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::error::Error;
extern crate simple_logger;



#[derive(Serialize, Deserialize, Debug, Default)]
pub struct P2pdata {
    pub message_bytes: usize, // The length in bytes of message
    pub message_type: u16, // The type of data
    pub message: String,   // The serialized data
}


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Peer {
    pub id: String,
    pub socket: SocketAddr,     // socket (ip, port) of a peer
    pub info: PeerTracker, // stats about recived and sent bytes from this peer
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tracker {
    pub sent_bytes: u32,
    pub received_bytes: u32,
    pub peers: u32,
    pub uptime: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PeerTracker {
    pub sent_bytes: u32,
    pub recieved_bytes: u32,
}
fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 128];
    while match stream.read(&mut data) {
        Ok(_) => {
            deformMsg(&String::from_utf8(data.to_vec()).unwrap());
            true
        }
        Err(_) => {
            debug!(
                "Terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn rec_server() -> u8 {
    let listener = TcpListener::bind("127.0.0.1:56789").unwrap();
    // accept connections and process them, spawning a new thread for each one
    info!(
        "P2P Server Launched on 127.0.0.1:{}",
        56789
    );
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!(
                    "New incoming connection: {}",
                    stream.peer_addr().unwrap()
                );

                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                warn!(
                    "handling peer connection to peer resulted in  error: {:?}",
                    e
                );
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
    return 1;
}
fn new_connection(socket: SocketAddr) -> Result<Peer,Box<dyn Error>> {
    // This Fucntion handles all the details of conecting to a peer, geting id and constructing a Peer struct
    let mut stream = TcpStream::connect(socket)?;
    let self_config = config();
    /*Once we have established a connection over TCP we now send vital data as a hanshake,
    This is in the following format
    network id,our peer id, our node type;
    The recipitent then verifyes this then they send the same hand shake back to us;
    */
    let msg = hex::encode(self_config.network_id)
        + ","
        + &self_config.identitiy
        + ","
        + &self_config.node_type.to_string();
    let _ = stream.write(formMsg(msg,0x1a).as_bytes()); // send our handshake
    let mut buffer = [0, 128];
    let _ = stream.read(&mut buffer);
    let pid: String = process_handshake(String::from_utf8(buffer.to_vec())?)?;
    let mut info = PeerTracker {
        sent_bytes: 128,
        recieved_bytes: 128,
    };
    return Ok(Peer {
        id: pid,
        socket,
        info,
    });
}

fn process_block(s: String) {
    info!("Block {}", s);
}

fn process_transaction(s: String) {
    info!("Transaction {}", s);
}

fn process_registration(s: String) {
    info!("Certificate {}", s);
}

fn process_handshake(s: String) -> Result<String, String> {
    let id: String;
    let network_id_hex = hex::encode(config().network_id);
    let peer_network_id_hex: &String = &s[0..network_id_hex.len()].to_string();
    if network_id_hex != peer_network_id_hex.to_owned() {
        return Err(String::from("Incorrect network id"));
    } else {
        let val = s[peer_network_id_hex.len()+1..s.len()].to_string();
        drop(s);
        let v: Vec<&str> = val.split(",").collect();
        id = v[0].to_string();
    }
    return Ok(id);
}


pub enum p2p_errors {
  None,
  TimeOut,
  InvalidSocket,
  Other,
}

fn formMsg(data_s: String, data_type: u16) -> String {
    let data_len = data_s.len();
    let msg: P2pdata = P2pdata {
        message_bytes: data_len,
        message_type: data_type,
        message: data_s,
    };
    return serde_json::to_string(&msg).unwrap();
}

fn deformMsg(msg: &String) { // deforms message and excutes appropriate function to handle resultant data
    let mut msg_d:P2pdata = serde_json::from_str(msg).unwrap_or_else(|e| {
        debug!("Bad Packets recieved from peer, packets: {}. Parsing this gave error {:?}", msg, e);
        return P2pdata::default();
    });
    match msg_d.message_bytes {
        0 => return,
        _ => (),
    }
    match msg_d.message_type {
        0x0a => process_block(msg_d.message),
        0x0b => process_transaction(msg_d.message),
        0x0c => process_registration(msg_d.message),
        0x1a => { match process_handshake(msg_d.message) { _=> (),} },
        _ => warn!("Bad Messge type from peer. Message type {}. (If you ae getting, lots of these check for updates)", msg_d.message_type.to_string()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version_major: u8,
    pub version_breaking: u8,
    pub version_minor: u8,
    pub coin_name: String,
    pub node_drop_off_threshold: u8,
    pub decimal_places: u8,
    pub max_connections: u16,
    pub max_threads: u8,
    pub chain_key: String,
    pub state: u8,
    pub ip_host: Vec<u16>,
    pub seednodes: Vec<Vec<u16>>,
    pub ignore_minor_updates: bool,
    pub p2p_port: u16,
    pub rpc_port: u16,
    pub allow_cors: char,
    pub buffer_bytes: u16,
    pub network_id: Vec<u8>,
    pub node_type: char,
    pub identitiy: String,
    pub key_file_path: String,
    pub log_level: u8, 
    pub min_intrest: f32,
    pub max_intrest: f32,
    pub max_reward: u32,
    pub min_vote: u8,
    pub probatory_epoch_count: u8,
    
}


pub fn config() -> Config {
    let mut file = File::open("node.conf").unwrap_or_else(|e| {
        error!("Failed to Open Config file: {}", e);
        panic!();
    });
    let mut data: String = String::from("");
    file.read_to_string(&mut data).unwrap();
    drop(file);
    let conf: Config = serde_json::from_str(&data).unwrap_or_else(|e| {
        error!("Failed To Deserilise Config: {}", e);
        panic!();
    });
    return conf;
}

impl Default for Config {
    fn default () -> Config {
        Config
        {
             version_major: 0,
             version_breaking: 0,
             version_minor: 1,
             coin_name: String::from("Avrio"),
             node_drop_off_threshold: 30,
             decimal_places: 4,
             max_connections: 50,
             max_threads: 4,
             chain_key: "".to_string(),
             state: 0,
             ip_host: vec![127,0,0,1,12345],
             seednodes: vec![
                 vec![127,0,0,1],
                 vec![127,0,0,1],
             ],
             ignore_minor_updates: false,
             p2p_port: 12345,
             rpc_port: 54321,
             allow_cors: 'n',
             buffer_bytes: 128,
             network_id: vec![
                 0x61, 0x76, 0x72, 0x69, 0x6f, 0x20, 0x6e, 0x6f, 0x6f, 0x64, 0x6c, 0x65,
             ],
             node_type: 'n',
             identitiy: String::from(""),
             key_file_path: "wallet.keys".to_string(),
             log_level: 2, // 0,1,2,3,4,5 trace, debug, info, warn, error, fatal respectivly
             min_intrest: 0.5,
             max_intrest: 3.0,
             max_reward: 25000,
             min_vote: 65, // min vote to not be banned
             probatory_epoch_count: 10,
        }
    }
}

fn main() {
  simple_logger::init_with_level(log::Level::Debug).unwrap();
  info!("p2p Test Version 0.1.0"); 
  let conf = Config::default();
  conf.create().unwrap();
  info!("{:?}", rec_server());
  info!("Attempting connection to self");
  info!("{:?}", new_connection(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 54321)));
  info!("Done");
}

impl Config {
    pub fn create(&self) -> io::Result<()> { // create file
        let mut file = File::create("node.conf")?;
        file.write_all(serde_json::to_string(self).unwrap().as_bytes())?;
        drop(file);

        Ok(())
    }
    pub fn save(&self) -> io::Result<()> { // save to exisiting/ update
        let mut conf_file = File::create("node.conf").unwrap_or_else(|e| {
            error!("Failed to Open Config file: {}", e);
            panic!();
        });

        conf_file.write_all(serde_json::to_string(self).unwrap_or_else(|e| { error!("Config Parsing failed with error {}", e); panic!(); }).as_bytes())?;
        drop(conf_file);
        Ok(())
    }
}
