use std::ffi::CString;
use std::os::raw::c_char;
use crypto::digest::Digest;
use crypto::sha3;

extern "C" {
  pub fn __set_mem_len(len: i32);
}

#[no_mangle]
pub fn chain_name() -> *mut c_char {
  let chain_name = "Ethereum";
  let cstr = CString::new(chain_name).unwrap();
  let str_ptr = cstr.into_raw();
  let str_len = chain_name.len() as i32;
  unsafe {
    __set_mem_len(str_len);
  }
  str_ptr
}

#[no_mangle]
pub fn to_address(pubkey: &str) -> *mut c_char {
  let pubkey_bytes = hex::decode(pubkey).unwrap();
  let keccak256ed = keccak256(pubkey_bytes.as_slice());
  let address_bytes = &keccak256ed[12..];
  let address = to_string(address_bytes);
  let str_len = address.len() as i32;
  let str_ptr = CString::new(address.as_str()).unwrap().into_raw();
  unsafe {
    __set_mem_len(str_len)
  }
  str_ptr
}

fn keccak256(raw: &[u8]) -> [u8; 32] {
  let mut keccak = sha3::Sha3::keccak256();
  keccak.input(raw.as_ref());
  let mut output = [0u8; 32];
  keccak.result(&mut output);
  output
}

fn to_string(raw: &[u8]) -> String {
  let address = hex::encode(raw);
  let keccak256ed = hex::encode(keccak256(raw));
  let mut result: Vec<char> = Vec::new();
  for (i, c) in address.chars().into_iter().enumerate() {
    if c >= 'a' && keccak256ed.chars().nth(i).unwrap() >= '8' {
      result.push(c.to_ascii_uppercase());
    } else {
      result.push(c);
    }
  }
  format!("0x{}", result.iter().collect::<String>())
}
