use super::*;
use crate::hex::decode;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_address_determinism() {
        let deployer = "0xEB74E11f33Cdc0d72b172984205493243F83a578"
            .parse::<ethers::types::Address>()
            .expect("Invalid address format");
        let factory = Create2Factory::new(deployer);
        let salt = Create2Helper::generate_salt("test");

        let bytecode = decode(
            "6080604052348015600f57600080fd5b50606780601d6000396000f3fe6080604052600080fd00",
        )
        .unwrap();
        let bytecode_hash = hash_bytecode(&bytecode);
        //let bytecode = b"test bytecode";

        let addr1 = factory.compute_address(salt, bytecode_hash);
        let addr2 = factory.compute_address(salt, bytecode_hash);

        assert_eq!(addr1, addr2, "Addresses should be deterministic");
    }

    #[test]

    fn test_create2_address_computation() {
        let deployer = "0xEB74E11f33Cdc0d72b172984205493243F83a578"
            .parse::<ethers::types::Address>()
            .expect("Invalid address format");

        let factory = Create2Factory::new(deployer);
        let salt = Create2Helper::generate_salt("test_salt");

        let bytecode = decode(
            "6080604052348015600f57600080fd5b50606780601d6000396000f3fe6080604052600080fd00",
        )
        .unwrap();
        let bytecode_hash = hash_bytecode(&bytecode);

        let address_result = factory.compute_address(salt, bytecode_hash);
        let address = address_result.unwrap();

        println!("Computed address: {}", address);

        assert_eq!(address.as_bytes().len(), 20, "Address should be 20 bytes");
    }

    fn hash_bytecode(bytecode: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak::v256();
        hasher.update(bytecode);
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);
        output
    }
}
