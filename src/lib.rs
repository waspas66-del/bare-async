#![no_std]

//! # Bare-Async
//! 
//! Minimalist toolkit for building manual, stackless state machines.
//! Optimized for systems where even the smallest overhead matters.

/// Represents the result of a single step in a state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepResult<T> {
    /// The operation is complete, returning the output value.
    Ready(T),
    /// The operation is blocked and waiting for an external event.
    Pending,
}

/// The core trait for manual, auditable coroutines.
/// 
/// Unlike standard Futures, RawCoroutine allows passing an explicit 
/// context to every execution step.
pub trait RawCoroutine {
    /// The value produced upon completion.
    type Output;
    
    /// The execution context (e.g., hardware registers or shared state).
    type Context;

    /// Advances the state machine one step.
    fn step(&mut self, cx: &mut Self::Context) -> StepResult<Self::Output>;
}
pub mod examples;
