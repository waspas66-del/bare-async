# Bare-Async

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust: no--std](https://img.shields.io/badge/Rust-no--std-blue.svg)](#)
[![Performance: Zero--Cost](https://img.shields.io/badge/Performance-Zero--Cost-red.svg)](#)

**Bare-Async** is a minimalist toolkit for building manual, stackless state machines in systems-level Rust. It is specifically designed for bare-metal, embedded, and safety-critical applications where heap allocation is forbidden and every CPU cycle must be auditable.

## Why Bare-Async?

Standard Rust `async/await` is powerful but often hides state transitions and adds hidden overhead through complex `Waker` mechanics. In resource-constrained environments, you need **explicit control**.

- **Zero Allocation:** No `Box`, no `Arc`, no heap usage.
- **Naked Executor:** A polling-based runtime with no hidden background magic.
- **Deterministic Latency:** You control exactly when and how the state machine advances.
- **No-Std by Design:** Built for the "bare metal" from the very first line of code.

## Core Abstraction

The heart of the library is the `RawCoroutine` trait. It decouples the state machine from the executor, allowing you to pass any custom context (like hardware registers or system uptime) directly into the execution logic.

```rust
pub trait RawCoroutine {
    type Output;
    type Context;

    fn step(&mut self, cx: &mut Self::Context) -> StepResult<Self::Output>;
}
use bare_async::{RawCoroutine, StepResult, Delay, block_on};

// Create a simple 5-tick timer
let timer = Delay::new(5);

// Drive it to completion using the naked executor
block_on(timer, ());

Philosophy
We believe that high-performance concurrency should be transparent. Bare-Async doesn't hide the state machine behind compiler magic—it gives you the tools to build it, audit it, and run it with maximum efficiency and zero hidden costs.

## Support the Mission

If this library saved your hardware from bloating or your brain from exploding, feel free to fuel the next update. No pressure, just keeping the gears greased (USDT preferred):

* **TRC20 (USDT):** `TXuVtJ4KU8CDVGrx7fH6K3vcQarA4EpgzF` — for Tron fans who like their coffee strong.
* **BEP20 (USDT):** `0x2dd5129b53816e7e4156bfb3c93e2293491f1e65` — Binance lovers, you know the drill!
* **SOL (USDT/SOL):** `BD9rLoT1jg3FBa3GUtwMeUxbztSwXogZqQLe2AsR9Nba` — Solana speed for your espresso fix.
