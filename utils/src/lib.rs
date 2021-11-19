
use rand::rngs::StdRng;
use rand::SeedableRng;
use sha2::{Sha256, Digest};
use js_sys::Uint8Array;
use dusk_bytes::Serializable;
use bip0039::{Count, Mnemonic};
use dusk_pki::{
    SecretSpendKey

};

use wasm_bindgen::prelude::*;





// #[no_mangle]
// pub extern fn add_one(x: u32)-> u32{
//     x + 1 
// }




// Create a static mutable byte buffer.
// We will use for passing memory between js and wasm.
// NOTE: global `static mut` means we will have "unsafe" code
// but for passing memory between js and wasm should be fine.
const WASM_MEMORY_BUFFER_SIZE: usize = 2;
static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];

// Function to store the passed value at index 0,
// in our buffer
#[wasm_bindgen]
pub fn store_value_in_wasm_memory_buffer_index_zero(value: u8) {
  unsafe {
    WASM_MEMORY_BUFFER[0] = value;
  }
}

// Function to return a pointer to our buffer
// in wasm memory
#[wasm_bindgen]
pub fn get_wasm_memory_buffer_pointer() -> *const u8 {
  let pointer: *const u8;
  unsafe {
    pointer = WASM_MEMORY_BUFFER.as_ptr();
  }

  return pointer;
}

// Function to read from index 1 of our buffer
// And return the value at the index
#[wasm_bindgen]
pub fn read_wasm_memory_buffer_and_return_index_one() -> u8 {
  let value: u8;
  unsafe {
    value = WASM_MEMORY_BUFFER[1];
  }
  return value;
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