use anyhow::Result;
use methods::{HELLO_GUEST_ELF, HELLO_GUEST_ID};
use risc0_zkvm::{Executor, ExecutorEnv, ProverOpts, Session};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct JwtValidationInput {
    token: String,
}

fn main() -> Result<()> {
    // Check if a JWT token is provided as a command-line argument
    let token = std::env::args()
        .nth(1)
        .expect("Please provide a JWT token as an argument");

    // Prepare input for the guest
    let input = JwtValidationInput { token };

    // Create an executor environment
    let env = ExecutorEnv::builder()
        .write_json(&input)?
        .build()?;

    // Create an executor
    let mut exec = Executor::from_elf(env, HELLO_GUEST_ELF)?;

    // Run the executor
    let session = exec.run()?;

    // Verify the session
    session.verify(HELLO_GUEST_ID)?;

    Ok(())
}
