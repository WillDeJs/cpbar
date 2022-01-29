//!
//! Console Progress Bar
//!
//! Creates a loading progress bar on the console.
//! Based on: Will Crichton talk on Strange Loop Conference "Type-Driven API Desgin in Rust"
//! See talk here <https://www.youtube.com/watch?v=bnnacleqg6k>
//!
//! Will create a progress bar  and show progress for each entry of the iterator processed.
//!
//! Example usage:
//! ```
//! use crate::cpbar::*;
//!
//! let vector = vec![1,2,4,5,6,6,7,8,9,0];
//! for element in ProgressBar::new(vector.iter()) {
//!     // execute operation with elements
//!     expensive_operation(element);
//! }
//!
//!
//! fn expensive_operation(element: &i32) {
//!     std::thread::sleep(std::time::Duration::from_millis(100));
//! }
//! ```
//!
//! It is possible to have bounded bars and pass a given delimiter as bound:
//!
//! ```
//! use cpbar::*;
//!
//! let vector = vec![1,2,4,5,6,6,7,8,9,0];
//! for element in ProgressBar::new(vector.iter()).with_bounds().with_delims(('<', '>')) {
//!     // execute operation with elements
//! }
//! ```
//!
#[allow(dead_code)]
#[warn(missing_docs)]

/// Constant used to clear screen line on console when printing
#[doc(hidden)]
const CLEAR: &str = "\x1b[0J\x1b[1A";
const MAX_COLUMN_WIDTH: usize = 30;

/// ProgressBar bar structure. Crates a progress bar from an iterable element given.
/// Displays the progress as items on the iterator are consumed.
pub struct ProgressBar<Iter, Bound> {
    iter: Iter,
    index: usize,
    start: std::time::Instant,
    bound: Bound,
}

/// Trait for internal usage. Used to print the progess of each entry.
#[doc(hidden)]
pub trait ProgressBarDisplay
where
    Self: Sized,
{
    fn display<Iter>(&self, progress: &ProgressBar<Iter, Self>);
}

/// Unbounded iterator type state. This is used by the internal API
/// in order to accept unbounded iterators.
#[doc(hidden)]
pub struct Unbounded;

// Bounded iterator type state. This is used by the internal API
/// in order to accept bounded iterators.
#[doc(hidden)]
pub struct Bounded {
    bound: usize,
    delims: (char, char),
}

impl ProgressBarDisplay for Bounded {
    fn display<Iter>(&self, progress: &ProgressBar<Iter, Self>) {
        let percent = (progress.index * 100) / self.bound;
        let elapsed_time = std::time::Instant::now() - progress.start;
        if self.bound < MAX_COLUMN_WIDTH {
            println!(
                "{}{:3}% {}{}{}{} {}/{} {:.4} Secs",
                CLEAR,
                percent,
                self.delims.0,
                "▓".repeat(progress.index),
                "░".repeat(self.bound - progress.index),
                self.delims.1,
                progress.index,
                self.bound,
                elapsed_time.as_secs_f64()
            );
        } else {
            let ticks = MAX_COLUMN_WIDTH * percent / 100;
            println!(
                "{}{:3}% {}{}{}{} {}/{} {:.4} Secs",
                CLEAR,
                percent,
                self.delims.0,
                "▓".repeat(ticks),
                "░".repeat(MAX_COLUMN_WIDTH - ticks),
                self.delims.1,
                progress.index,
                self.bound,
                elapsed_time.as_secs_f64()
            );
        }
    }
}

impl ProgressBarDisplay for Unbounded {
    fn display<Iter>(&self, progress: &ProgressBar<Iter, Self>) {
        let elapsed_time = std::time::Instant::now() - progress.start;
        println!(
            "{}[{} in {:.4} Secs] ",
            CLEAR,
            progress.index,
            elapsed_time.as_secs_f32()
        );
    }
}

impl<Iter> ProgressBar<Iter, Unbounded>
where
    Iter: Iterator,
{
    /// Create a new progress bar.  Requires an iterator to be passed as argument.
    ///
    /// # Example
    /// ```
    /// use cpbar::*;
    /// let progress_bar = ProgressBar::new((0..6));
    /// for _ in progress_bar {
    ///     // do operation
    /// }
    /// ```
    pub fn new(iter: Iter) -> Self {
        println!();
        Self {
            iter,
            index: 0,
            start: std::time::Instant::now(),
            bound: Unbounded,
        }
    }
}

impl<Iter, Bound> Iterator for ProgressBar<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressBarDisplay,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.bound.display(&self);
        self.index += 1;
        self.iter.next()
    }
}
impl<Iter> ProgressBar<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    /// Initializes a progress bar with known number of iterable items.
    ///
    /// # Example
    ///
    /// ```
    /// use cpbar::*;
    /// let progress_bar = ProgressBar::new((0..6)).with_bounds();
    /// ```
    ///
    pub fn with_bounds(self) -> ProgressBar<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        ProgressBar {
            iter: self.iter,
            start: std::time::Instant::now(),
            bound,
            index: self.index,
        }
    }
}

impl<Iter> ProgressBar<Iter, Bounded>
where
    Iter: ExactSizeIterator,
{
    /// Adds custom delimetering chracters to bounded progress bar.
    ///
    /// # Example
    /// ```
    /// use cpbar::*;
    /// let progress_bar = ProgressBar::new((0..6))
    ///                 .with_bounds()
    ///                 .with_delims(('<','>'));
    /// ```
    ///
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}
