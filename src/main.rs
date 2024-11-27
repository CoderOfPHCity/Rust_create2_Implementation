mod create2test;

use ethers::types::Address;
use ethers::utils::hex;
use hex_literal::hex;
use tiny_keccak::{Hasher, Keccak};

#[derive(Debug, PartialEq)]
pub enum Create2Error {
    EmptyBytecode,
    InsufficientBalance,
    DeploymentFailed,
}

pub struct Create2Factory {
    pub deployer: Address,
}

impl Create2Factory {
    pub fn new(deployer: Address) -> Self {
        Self { deployer }
    }

    pub fn compute_address(
        &self,
        salt: [u8; 32],
        bytecode_hash: [u8; 32],
    ) -> Result<Address, Create2Error> {
        if bytecode_hash.iter().all(|&b| b == 0) {
            return Err(Create2Error::EmptyBytecode);
        }
        //

        let deployer_bytes = self.deployer.as_bytes();
        let buffer = [&[0xff], deployer_bytes, &salt, &bytecode_hash].concat();

        // let mut hasher = Keccak::v256();
        // hasher.update(&buffer);
        // let mut output = [0u8; 32];
        // hasher.finalize(&mut output);

        let output = {
            let mut hash = Keccak::v256();
            hash.update(&buffer);
            let mut out = [0u8; 32];
            hash.finalize(&mut out);
            out
        };
        let mut address = [0u8; 20];
        address.copy_from_slice(&output[12..32]);

        Ok(Address::from_slice(&address))
    }
}

pub struct Create2Helper;

impl Create2Helper {
    pub fn generate_salt(salt_string: &str) -> [u8; 32] {
        let mut salt = [0u8; 32];
        let salt_bytes = salt_string.as_bytes();
        salt[..salt_bytes.len()].copy_from_slice(salt_bytes);
        salt
    }
}

fn main() {
    let deployer_address = "0xEB74E11f33Cdc0d72b172984205493243F83a578"
        .parse::<ethers::types::Address>()
        .expect("Invalid address format");

    // Example salt and bytecode hash
    let salt = hex!("abcdef0123456789abcdef0123456789abcdef0123456789abcdef0987654321");
    let bytecode_hash = hex!("abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789");

    let factory = Create2Factory::new(deployer_address);

    match factory.compute_address(salt, bytecode_hash) {
        Ok(address) => println!("Computed CREATE2 address: {}", address),
        Err(e) => println!("Error computing address: {:?}", e),
    }
}
