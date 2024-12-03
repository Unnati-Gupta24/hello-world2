use jsonwebtoken::{decode, DecodingKey, Validation};
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct JwtValidationInput {
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JwtValidationOutput {
    is_valid: bool,
    claims: Option<String>,
}

fn main() {
    // Read the input
    let input: JwtValidationInput = env::read();

    // Placeholder secret key (in real-world, this would be a secure secret)
    let secret = b"your-256-bit-secret";

    // Attempt to decode the JWT
    let result = match decode::<serde_json::Value>(
        &input.token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    ) {
        Ok(token_data) => JwtValidationOutput {
            is_valid: true,
            claims: Some(format!("{:?}", token_data.claims)),
        },
        Err(err) => JwtValidationOutput {
            is_valid: false,
            claims: Some(err.to_string()),
        },
    };

    // Write the output
    env::write(&result);
}