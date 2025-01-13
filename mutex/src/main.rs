#![allow(dead_code, unused)]
use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
// Implementation of Mutex simplified
// With spinlock which should never be the case
// https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html
// Never implement spin locks!!!!

const LOCKED: bool = true;
const UNLOCKED: bool = false;

struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub fn new(v: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(v),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // 2 operations for load store
        // while self.locked.load(Ordering::Relaxed) != UNLOCKED {}
        // maybe another thread runs here
        // if we uncomment here race condition!
        // std::thread::yield_now();
        // self.locked.store(LOCKED, Ordering::Relaxed);

        // x86: CAS
        // ARM: LDREX STREX
        //  - compare_exchange: impl using the loop of LDREX and STREX
        //  - compare_exchange_weak: LDREX STREX allow to fail on any of the steps and return error
        while self
            .locked
            // this one is getting exclusive access to the value
            // which is expensive
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol: stay in S when locked
            // so we are doing another loop here to wait and read the value by shared access
            // without ownership bouncing
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                std::hint::spin_loop();
            }
        }

        // Safety: we hold a lock, no other threads can create a mutable reference
        let res = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        res
    }
}

use std::thread::spawn;

#[test]
fn spawn_a_lot_threads() {
    let x: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handles: Vec<_> = (0..10)
        .map(|_| {
            spawn(move || {
                for _ in 0..100 {
                    x.with_lock(|v| *v += 1);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(x.with_lock(|v| *v), 10 * 100);
}

#[test]
fn too_relaxed() {
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let t1 = spawn(move || {
        let r1 = y.load(Ordering::Relaxed);
        x.store(r1, Ordering::Relaxed);
        r1
    });
    let t2 = spawn(move || {
        let r2 = x.load(Ordering::Relaxed);
        y.store(42, Ordering::Relaxed);
        r2
    });

    let _r1 = t1.join().unwrap();
    let _r2 = t2.join().unwrap();
    // Could be the case when
    // r1 == r2 == 42
}

fn main() {
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || {
        x.store(true, Ordering::Release);
    });
    let _ty = spawn(move || {
        y.store(true, Ordering::Release);
    });
    let t1 = spawn(move || {
        while !x.load(Ordering::SeqCst) {}
        if y.load(Ordering::SeqCst) {
            z.fetch_and(1, Ordering::Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Ordering::SeqCst) {}
        // allowed to see that x == false even if the tx was executed
        // CPU reading from cache of the value that was set in the main thread
        // no direct connections to the tx store operation
        // if x.load(Ordering::Acquire) {
        if x.load(Ordering::SeqCst) {
            z.fetch_and(1, Ordering::Relaxed);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();

    let z = z.load(Ordering::SeqCst);
    // What are possible values of z?
    // - Is 0 possible?
    //   Restrictions:
    //      We know that t1 should be run after tx
    //      We know that t2 should be run after ty
    //   Given that:
    //      .. tx .. t1
    //        ty t2 tx t1 -> t1 incr z
    //        ty tx t2 t1 -> t1 & t2 incr z
    //        tx t1 ty t2 -> t2 inc z
    //   Seem impossible to have a thread schedule where z == 0
    //
    //           t2    t1,t2
    //   MO(x): false  true
    //
    //           t1    t2,t1
    //   MO(y): false  true
    //
    //   So we need to use SeqCst (sequentially consistent) Ordering
    //   which is stronger that Require Release and not only garantees the
    //   operation order but also garantees that the all operations
    //   must be seen as happening in the correct order in ALL threads
    //
    // - Is 1 possible?
    //   Yes: tx t1 ty t2
    // - Is 2 possible?
    //   Yes: tx ty t1 t2
}
