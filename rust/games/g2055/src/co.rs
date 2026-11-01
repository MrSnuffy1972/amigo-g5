//! The main program on its own thread, in strict lock-step with the engine's ticks (the pattern of game 336's
//! `co.rs`). The original's routines wait for the next frame wherever they need to, often deep inside other routines;
//! running the program on its own thread lets each routine keep that shape. Each engine tick hands the program that
//! tick's input and waits until the program reaches its next frame wait, so the result is as deterministic as one
//! thread. Dropping the runner unwinds the program thread.

use std::any::Any;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread::JoinHandle;

/// Unwinds the program thread when the runner is dropped (never shown as a panic).
struct Stop;

/// The program's side: its frame wait.
pub struct Link {
    rx: Receiver<()>,
    tx: SyncSender<()>,
}

impl Link {
    /// Waits for the next frame: gives the tick back to the engine and returns when the next tick starts.
    pub fn frame(&mut self) {
        if self.tx.send(()).is_err() || self.rx.recv().is_err() {
            resume_unwind(Box::new(Stop));
        }
    }
}

/// The engine's side.
pub struct Co {
    tx: Option<SyncSender<()>>,
    rx: Receiver<()>,
    thread: Option<JoinHandle<Result<(), Box<dyn Any + Send>>>>,
    finished: bool,
}

impl Co {
    /// Starts the program; it first runs on the first `tick`.
    pub fn start(program: impl FnOnce(&mut Link) + Send + 'static) -> Co {
        let (tick_tx, tick_rx) = sync_channel::<()>(0);
        let (wait_tx, wait_rx) = sync_channel::<()>(0);
        let thread = std::thread::spawn(move || {
            let mut l = Link {
                rx: tick_rx,
                tx: wait_tx,
            };
            if l.rx.recv().is_err() {
                return Ok(());
            }
            match catch_unwind(AssertUnwindSafe(|| program(&mut l))) {
                Ok(()) => Ok(()),
                Err(e) if e.is::<Stop>() => Ok(()),
                Err(e) => Err(e),
            }
        });
        Co {
            tx: Some(tick_tx),
            rx: wait_rx,
            thread: Some(thread),
            finished: false,
        }
    }

    /// One tick: run the program until its next frame wait. A panic in the program is raised here.
    pub fn tick(&mut self) {
        if self.finished {
            return;
        }
        let sent = self
            .tx
            .as_ref()
            .map(|t| t.send(()).is_ok())
            .unwrap_or(false);
        if !sent || self.rx.recv().is_err() {
            self.finished = true;
            if let Some(Ok(Err(e))) = self.thread.take().map(JoinHandle::join) {
                resume_unwind(e);
            }
        }
    }

    /// The program has ended (it returned or panicked).
    pub fn finished(&self) -> bool {
        self.finished
    }
}

impl Drop for Co {
    fn drop(&mut self) {
        self.tx.take();
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn the_program_runs_one_frame_a_tick() {
        let n = Arc::new(Mutex::new(0));
        let m = n.clone();
        let mut co = Co::start(move |l| loop {
            *m.lock().unwrap() += 1;
            l.frame();
        });
        for _ in 0..5 {
            co.tick();
        }
        assert_eq!(*n.lock().unwrap(), 5);
        drop(co);
    }

    #[test]
    fn a_program_that_ends_is_finished() {
        let mut co = Co::start(|l| l.frame());
        co.tick();
        assert!(!co.finished());
        co.tick();
        assert!(co.finished());
    }
}
