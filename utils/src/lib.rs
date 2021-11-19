
use rand::rngs::StdRng;
use rand::SeedableRng;
use sha2::{Sha256, Digest};
use js_sys::Uint8Array;
use dusk_bytes::Serializable;
use bip0039::{Count, Mnemonic};
use dusk_pki::{
    SecretSpendKey

};






#[no_mangle]
pub extern fn add_one(x: u32)-> u32{
    x + 1 
}



// pub extern fn generate_secret_key()-> Uint8Array {

    
//     let mnemonic = Mnemonic::generate(Count::Words12);
//     let _phrase = mnemonic.phrase();
//     let seed = mnemonic.to_seed("");


//     let mut hasher = Sha256::new();
//     hasher.update(seed);
//     let mut rng = StdRng::from_seed(hasher.finalize().into());
//     let genesis_ssk = SecretSpendKey::random(&mut rng);
//     Uint8Array::from(&genesis_ssk.to_bytes()[..])
// }