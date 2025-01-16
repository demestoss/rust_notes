use std::{
    fmt::Debug, rc::Rc, sync::atomic::{AtomicUsize, Ordering}
};

use rand::prelude::*;
use sorting::*;

#[derive(Clone, Debug)]
struct SortEvaluator<T: Debug> {
    t: T,
    cmps: Rc<AtomicUsize>,
}

impl<T: PartialEq + Debug> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.fetch_add(1, Ordering::Relaxed);
        self.t == other.t
    }
}

impl<T: Eq + Debug> Eq for SortEvaluator<T> {}

impl<T: PartialOrd + Debug> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmps.fetch_add(1, Ordering::Relaxed);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord + Debug> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmps.fetch_add(1, Ordering::Relaxed);
        self.t.cmp(&other.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(AtomicUsize::new(0));

    for n in [0, 10, 100, 1000, 10000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: counter.clone(),
            });
        }

        for _ in 0..10 {
            values.shuffle(&mut rand);

            let took = bench(BubbleSort, &values, &counter);
            println!("bubble: {n} {} {}", took.0, took.1);

            let took = bench(InsertionSort { smart: false }, &values, &counter);
            println!("insertion dumb: {n} {} {}", took.0, took.1);

            let took = bench(InsertionSort { smart: true }, &values, &counter);
            println!("insertion smart: {n} {} {}", took.0, took.1);

            let took = bench(SelectionSort, &values, &counter);
            println!("selection: {n} {} {}", took.0, took.1);

            let took = bench(MergeSort, &values, &counter);
            println!("merge: {n} {} {}", took.0, took.1);

            let took = bench(QuickSort, &values, &counter);
            println!("quick: {n} {} {}", took.0, took.1);

            let took = bench(StdSort, &values, &counter);
            println!("std: {n} {} {}", took.0, took.1);
        }
    }
}

fn bench<T: Ord + Clone + Debug, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &AtomicUsize,
) -> (usize, f64) {
    let mut values = values.to_vec();
    counter.store(0, Ordering::Relaxed);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = counter.load(Ordering::Relaxed);
    dbg!(&values.iter().map(|x| &x.t).collect::<Vec<_>>());
    assert!(values.is_sorted());
    (count, took.as_secs_f64())
}
