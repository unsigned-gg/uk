# uk

> uk (Unsigned Kernel & Microkernel Primitives)  
> Pure-Rust microkernel domain abstractions, zero-cost systems interfaces, and bare-metal sympathy.  
> Dual-licensed under `Apache-2.0 OR MIT`.

## Philosophical Foundations
- **Champion Inspiration:** Bryan Cantrill & Ralf Jung
- **Zero Days Open Source Initiative (ZOSI):** Machine sympathy, low-level efficiency, and verified contracts.
- **Architecture:** Zero-I/O domain model in `uk-domain` separated from systems runtime in `uk`.

## Crates
- `crates/uk`: Systems implementation and runtime interfaces.
- `crates/uk-domain`: Pure zero-I/O domain models, validation, and algebraic state machines.

## Building
All builds must specify `-j 2` to prevent memory exhaustion on shared multi-agent hosts:
```bash
cargo build -j 2
cargo test -j 2
```
