import gleam/erlang

// A type representing the output of the hash function
pub type Keccak256Hash {
  Keccak256Hash(gleam/binary.Binary)
}

// External declaration of the Rust function via Erlang
@external(erlang, "erlang_nif_module", "keccak_256_nif")
fn keccak_256(input: gleam/binary.Binary) -> Keccak256Hash

pub fn hash_keccak_256(input: String) -> Result(Keccak256Hash, Nil) {
  let input_binary = erlang.bitstring_to_binary(input)
  let hash = keccak_256(input_binary)
  Ok(hash)
}
