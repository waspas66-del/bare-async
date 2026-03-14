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
pub trait RawCoroutine {
    /// The value produced upon completion.
    type Output;
    
    /// The execution context (e.g., hardware registers or shared state).
    type Context;

    /// Advances the state machine one step.
    fn step(&mut self, cx: &mut Self::Context) -> StepResult<Self::Output>;
}

// --- Example Implementation ---

/// A simple timer coroutine that counts down steps.
pub struct Delay {
    ticks: u32,
}

impl Delay {
    /// Creates a new delay with specified number of ticks.
    pub const fn new(ticks: u32) -> Self {
        Self { ticks }
    }
}

impl RawCoroutine for Delay {
    type Output = ();
    type Context = (); // Context isn't needed for this simple timer

    fn step(&mut self, _cx: &mut Self::Context) -> StepResult<Self::Output> {
        if self.ticks > 0 {
            self.ticks -= 1;
            StepResult::Pending
        } else {
            StepResult::Ready(())
        }
    }
}
// --- Naked Executor ---

/// A minimal blocking executor that drives a coroutine to completion.
/// This is a "naked" executor: no heap, no wakers, just pure polling.
pub fn block_on<C>(mut coroutine: C, mut cx: C::Context) -> C::Output 
where 
    C: RawCoroutine 
{
    loop {
        match coroutine.step(&mut cx) {
            StepResult::Ready(out) => return out,
            StepResult::Pending => {
                // В реальном железе здесь может быть команда 'wfi' (wait for interrupt)
                // чтобы процессор не молотил впустую и экономил заряд.
            }
        }
    }
}
