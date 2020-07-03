#[macro_use]
extern crate serde_derive;
mod error;

pub mod model;
pub mod module;
pub mod wallet_db;

pub use error::WalletError;

pub use ethtx::{RawTransaction,convert_token, address_legal as eth_address_legal};
pub use substratetx::{account_info_key, decode_account_info, event_decode};

pub type WalletResult<T> = std::result::Result<T, WalletError>;

#[derive(PartialEq, Clone)]
pub enum StatusCode {
    DylibError = -1,//Errors caused by external input parameters
    FailToGenerateMnemonic = 100,//Failed to generate mnemonic
    OK = 200, // normal
    PwdIsWrong,//Wrong password
}

impl Default for StatusCode {
    fn default() -> Self {StatusCode::OK}
}

#[derive(PartialEq, Clone)]
pub enum EthChainId {
    //Ethernet chain type number, used in the signing process
    MAIN = 1,
    ROPSTEN = 3,
    RINKEBY = 4,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ChainType {
    //Used to distinguish the chain types supported by the wallet
    BTC = 1,
    BtcTest = 2,
    ETH = 3,
    EthTest = 4,
    EEE = 5,
    EeeTest = 6,
    OTHER = 7,
}

impl From<i64> for ChainType {
    fn from(chain_type: i64) -> Self {
        match chain_type {
            1 => ChainType::BTC,
            2 => ChainType::BtcTest,
            3 => ChainType::ETH,
            4 => ChainType::EthTest,
            5 => ChainType::EEE,
            6 => ChainType::EeeTest,
            _ => ChainType::OTHER,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use substratetx::{Keccak256, Crypto};
    use hex;

    #[test]
    fn mnemonic_create_test() {
        //Mnemonic word creation test, signature test
         let mnemonic = substratetx::Sr25519::generate_phrase(18);
         let data = "substrate sign method test";
         let s = String::new();
         match substratetx::Ed25519::sign(&mnemonic, data.as_bytes()) {
             Ok(signed_data) => println!("{}", hex::encode(&signed_data[..])),
             Err(e) => println!("{}", e.to_string()),
         }
     }

     #[test]
     fn func_sign_test() {
         //Initialize the database
         wallet_db::init_wallet_database();
         //Create a wallet instance
        let wallet_instance = model::Wallet::default();
        let mnemonic = wallet_instance.crate_mnemonic(15);
        let rawtx = "0xac040600ff0a146e76bbdc381bd77bb55ec45c8bef5f52e2909114d632967683ec1eb4ea300b0040e59c301200000000979d3bb306ed9fbd5d6ae1eade033b81ae12a5c5d5aa32781153579d7f6d5504ed000000";
        let eee = module::EEE{};
        match eee.raw_tx_sign(rawtx, "9328ebd6-c205-439d-a016-ebe6ab1e5408", "123456".as_bytes()) {
            Ok(signed_data) => println!("tx sign result {}", signed_data),
            Err(e) => println!("{}", e.to_string()),
        }
    }

    #[test]
    fn transfer_test() {
        let mnemonic = "settle essay unique empty neutral pistol essence monkey combine service gun burden";
        let from = "5HNJXkYm2GBaVuBkHSwptdCgvaTFiP8zxEoEYjFCgugfEXjV";
        let to = "5GGzGJR54YNjMKhaYt6hHV3o99FZ6JKYEDCrzUg1HCz1tWPa";
        let value = "200000000000000";
        let genesis_hash = "0xabb0f2e62dfab481623438e14b5e1d4114a6e9a2f0d3f5e83f9192276e50cf34";
        let index = 0;
        let runtime_version = 1;
        let genesis_hash_bytes = hex::decode(genesis_hash.get(2..).unwrap()).unwrap();
        let mut genesis_h256 = [0u8; 32];
        genesis_h256.clone_from_slice(genesis_hash_bytes.as_slice());
        // Involving database access requires a series of data preparations for normal testing
        let eee = module::EEE{};
        match eee.generate_transfer(from, to, value, genesis_hash, index, runtime_version, "123456".as_bytes()) {
            Ok(sign_str) => {
                println!("{}", sign_str);
            }
            Err(err) => println!("{}", err)
        }
    }
}
