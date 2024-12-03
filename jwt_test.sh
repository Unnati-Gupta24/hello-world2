#!/bin/bash

# Function to generate a base64 encoded random string
generate_random_base64() {
    # Generate random bytes and encode to base64
    openssl rand -base64 $1
}

# Function to create a simple JWT-like token
generate_jwt() {
    # Header (base64 encoded)
    header=$(echo -n '{"alg":"HS256","typ":"JWT"}' | base64 | tr -d '=' | tr '/+' '_-')

    # Payload (base64 encoded) with some random data
    payload=$(echo -n "{\"user\":\"test_$(generate_random_base64 4)\",\"exp\":$(date +%s)}" | base64 | tr -d '=' | tr '/+' '_-')

    # Signature (just a placeholder, not cryptographically secure)
    signature=$(echo -n "$header.$payload" | openssl dgst -sha256 -hmac "secret" -binary | base64 | tr -d '=' | tr '/+' '_-')

    # Combine to form a JWT
    echo "$header.$payload.$signature"
}

# Number of random tokens to generate
NUM_TOKENS=5

# Generate and test multiple random tokens
for ((i=1; i<=NUM_TOKENS; i++)); do
    # Generate a random JWT-like token
    TOKEN=$(generate_jwt)
    
    echo "Attempt $i: Testing token: $TOKEN"
    
    # Run the host program with the generated token
    cargo run --bin host "$TOKEN"
    
    # Small delay between runs
    sleep 0.5
done

# Also test with some predefined tokens
echo "Testing predefined tokens..."
PREDEFINED_TOKENS=(
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjoiMTIzNDU2Nzg5MCJ9.fake_signature"
    "invalid.token.format"
)

for token in "${PREDEFINED_TOKENS[@]}"; do
    echo "Testing predefined token: $token"
    cargo run --bin host "$token"
    sleep 0.5
done