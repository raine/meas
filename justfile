build:
  cargo build

run-w *FLAGS:
  fd .rs | entr -r cargo run {{FLAGS}}

install:
  cargo install --path .
