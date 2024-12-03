// methods/src/lib.rs
use risc0_zkvm::MethodId;

// Placeholder for method IDs
pub const HELLO_GUEST_ID: MethodId = MethodId::new("hello_guest");
pub const HELLO_GUEST_ELF: &[u8] = include_bytes!("../guest/target/riscv-guest/release/hello_guest");