DOCS_RS=1 cargo build --no-default-features
DOCS_RS=1 cargo build --no-default-features --features uuid
DOCS_RS=1 cargo build --no-default-features --features serde
DOCS_RS=1 cargo build --no-default-features --features random
DOCS_RS=1 cargo build --no-default-features --features time
DOCS_RS=1 cargo build --no-default-features --features std
DOCS_RS=1 cargo build --no-default-features --features std,uuid
DOCS_RS=1 cargo build --no-default-features --features std,serde
DOCS_RS=1 cargo build --no-default-features --features std,random
DOCS_RS=1 cargo build --no-default-features --features std,time
DOCS_RS=1 cargo build --no-default-features --features std,uuid,serde,random,time

DOCS_RS=1 cargo test --features std,uuid,serde,random,time
