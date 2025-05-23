DOCS_RS=1 cargo build --no-default-features &&\
DOCS_RS=1 cargo build --no-default-features --features uuid &&\
DOCS_RS=1 cargo build --no-default-features --features serde &&\
DOCS_RS=1 cargo build --no-default-features --features random &&\
DOCS_RS=1 cargo build --no-default-features --features time &&\
DOCS_RS=1 cargo build --no-default-features --features language &&\
DOCS_RS=1 cargo build --no-default-features --features language-full &&\
DOCS_RS=1 cargo build --no-default-features --features uuid,serde,random,time,language &&\
DOCS_RS=1 cargo build --no-default-features --features std &&\
DOCS_RS=1 cargo build --no-default-features --features std,uuid &&\
DOCS_RS=1 cargo build --no-default-features --features std,serde &&\
DOCS_RS=1 cargo build --no-default-features --features std,random &&\
DOCS_RS=1 cargo build --no-default-features --features std,time &&\
DOCS_RS=1 cargo build --no-default-features --features std,language &&\
DOCS_RS=1 cargo build --no-default-features --features std,language-full &&\
DOCS_RS=1 cargo build --no-default-features --features std,uuid,serde,random,time,language &&\
DOCS_RS=1 cargo build --no-default-features --features std,uuid,serde,random,time,language --target wasm32-wasip1 &&\

DOCS_RS=1 cargo test --features std,uuid,serde,random,time,language &&\
cargo test --features std,uuid,serde,random,time,language &&\

cargo run --example count_languages --features language &&\
cargo run --example count_languages --features language-full &&\
cargo run --example random_hitokoto &&\

exit
