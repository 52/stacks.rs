use stacks_rs::crypto::hex_to_bytes;
use stacks_rs::StacksPrivateKey;
use stacks_rs::StacksPublicKey;

pub fn get_private_key() -> StacksPrivateKey {
    let pk_hex = "edf9aee84d9b7abc145504dde6726c64f369d37ee34ded868fabd876c26570bc";
    let pk_bytes = hex_to_bytes(pk_hex).unwrap();
    StacksPrivateKey::from_slice(&pk_bytes).unwrap()
}

pub fn get_sponsor_key() -> StacksPrivateKey {
    let pk_hex = "9888d734e6e80a943a6544159e31d6c7e342f695ec867d549c569fa0028892d4";
    let pk_bytes = hex_to_bytes(pk_hex).unwrap();
    StacksPrivateKey::from_slice(&pk_bytes).unwrap()
}

pub fn get_multi_sig_keys() -> (Vec<StacksPrivateKey>, Vec<StacksPublicKey>) {
    let pk_hex = vec![
        "6d430bb91222408e7706c9001cfaeb91b08c2be6d5ac95779ab52c6b431950e0",
        "2a584d899fed1d24e26b524f202763c8ab30260167429f157f1c119f550fa6af",
        "d5200dee706ee53ae98a03fba6cf4fdcc5084c30cfa9e1b3462dcdeaa3e0f1d2",
    ];

    let mut private_keys = vec![];
    let mut public_keys = vec![];

    for hex in pk_hex {
        let pk_bytes = hex_to_bytes(hex).unwrap();
        let private_key = StacksPrivateKey::from_slice(&pk_bytes).unwrap();
        let public_key = private_key.public_key(&secp256k1::Secp256k1::new());
        private_keys.push(private_key);
        public_keys.push(public_key);
    }

    (private_keys, public_keys)
}
