cargo build --no-default-features
cargo build --no-default-features --features uuid
cargo build --no-default-features --features serde
cargo build --no-default-features --features random
cargo build --no-default-features --features std
cargo build --no-default-features --features std,uuid
cargo build --no-default-features --features std,serde
cargo build --no-default-features --features std,random
cargo build --no-default-features --features std,uuid,serde,random

cargo test --features std,uuid,serde,random
