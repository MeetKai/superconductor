echo "Checking /" &&
cargo deny --workspace --all-features check -A duplicate &&
echo "Checking /shaders" &&
cargo deny --workspace --all-features --manifest-path shaders/Cargo.toml check -A duplicate &&
echo "Checking /rust-gpu-cli-builder" &&
cargo deny --workspace --all-features --manifest-path rust-gpu-cli-builder/Cargo.toml check -A duplicate
