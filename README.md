# Rust simple bank account
If you're uncovering this, welcome lol. I'm just documenting my learning process with rust

To run this, you need rust and cargo installed
On mac os with brew, I run
```brew install rust rust-analyzer rustup-init```

In the terminal, checking to make sure the rust code compiles correctly run ```cargo b``` and recieve logs such as
```rust
% cargo b
   Compiling rustplayground v0.1.0 (/tmp/rustplayground)
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
```

or cool error logs like
```rust
% cargo b
   Compiling rustplayground v0.1.0 (/tmp/rustplayground)
warning: unused variable: `amount`
  --> src/accounts/debt.rs:56:28
   |
56 |     fn withdraw(&mut self, amount: f64) -> Result<(), String> {
   |                            ^^^^^^ help: if this is intentional, prefix it with an underscore: `_amount`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `amount`
  --> src/accounts/investment.rs:54:28
   |
54 |     fn withdraw(&mut self, amount: f64) -> Result<(), String> {
   |                            ^^^^^^ help: if this is intentional, prefix it with an underscore: `_amount`

warning: unused variable: `amount`
  --> src/accounts/investment.rs:58:27
```

With successful builds and if you want to try to run it while also trying to build run ```cargo r```

*deployment.tf file is a work in progress