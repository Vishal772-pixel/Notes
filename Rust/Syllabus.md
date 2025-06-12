Rust for Web3 — Full Roadmap + Syllabus

Phase 1: Rust Fundamentals

Start with core language concepts. Don't rush.

1. Rust Setup + Hello World

Install Rust: rustup, cargo

Project structure (Cargo.toml, main.rs)

Compile and run


2. Basic Syntax

Variables, mutability

Data types

Control flow (if, loop, while, for)


3. Functions

Defining and calling functions

Return types

Expressions vs Statements


4. Ownership & Borrowing

The heart of Rust!

Ownership rules

Borrowing (&, &mut)

Slices

Lifetimes (basic intro)


5. Structs & Enums

Structs, Tuple Structs

Enums (like Option, Result)

Pattern matching with match


6. Error Handling

Option<T> and Result<T, E>

unwrap, expect, ? operator


7. Collections

Vectors, HashMaps, Strings

Iterators and Loops


8. Traits and Generics

Defining traits

Implementing traits

Generic functions and structs


9. Modules and Crates

Organizing code with modules

pub, mod, use

External crates from crates.io



---

Phase 2: Intermediate Rust Concepts

10. Lifetimes (Deep Dive)

Lifetimes explained

Elision rules

Lifetime annotations in structs and functions


11. Closures and Iterators

Closures syntax (|x| x + 1)

Capturing environment

Functional patterns: map, filter, collect


12. Smart Pointers

Box, Rc, Arc, RefCell

When and why to use them


13. Concurrency

Threads and std::thread

Message passing

Mutex, Arc for shared state


14. Error Handling (Advanced)

Creating custom error types

thiserror and anyhow crates


15. Testing in Rust

Writing unit and integration tests

Using cargo test



---

Phase 3: Web3-Oriented Rust

Path A: Solana (Anchor Framework)

Learn Solana’s architecture

Rust programs (smart contracts) on-chain

Anchor framework:

#[derive(Accounts)], #[program], #[account]

CPIs, cross-program invocations

PDA (Program-Derived Addresses)


Local testing: Solana CLI, Anchor test suite


Path B: Substrate (Polkadot)

Learn Substrate architecture

FRAME development

Runtime modules in Rust

Pallets and extrinsics

Ink! (for smart contracts in Substrate)



---

Phase 4: Full Projects for Web3 in Rust

NFT Minting DApp with Solana + Anchor

Token Swap/DEX contract

DAO contract

Substrate blockchain with custom pallet

Rust backend with Axum/Actix to serve blockchain data



---

Bonus: Tooling & Ecosystem

cargo CLI mastery

rust-analyzer (VS Code plugin)

Format/lint: rustfmt, clippy

Dependency mgmt: Cargo.toml, features

Async Rust: tokio, async/.await



---

Suggested Learning Platforms

Rust Book (Official)

Rustlings (interactive exercises)

Solana Docs

Substrate Docs

Zero to Production in Rust