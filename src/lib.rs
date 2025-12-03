#![doc = include_str!("../README.md")]

use std::iter;
use std::marker::PhantomData;

pub use exhaustive_macros::exhaustive_test;
pub use exhaustive_macros::Exhaustive;

mod impls;

pub trait Exhaustive: Sized {
    fn generate(u: &mut DataSourceTaker) -> Result<Self>;

    fn iter_exhaustive(max_choices: Option<usize>) -> impl Iterator<Item = Self> {
        let mut source = DataSource::new(max_choices.unwrap_or(usize::MAX));
        iter::from_fn(move || source.next_run().map(|mut u| Self::generate(&mut u))).flatten()
    }
}

#[derive(Copy, Clone)]
struct Choice {
    /// The last choice that was made
    choice: usize,
    /// Maximum choice, inclusive
    max_choice: usize,
}

pub struct DataSourceTaker<'a> {
    /// The amount of choices we can still make, only used for limiting the exponential growth
    choices_left: usize,
    buffer: &'a mut Vec<Choice>,
    buffer_idx: usize,
}

pub type Result<T> = std::result::Result<T, ChoiceError>;

#[derive(Debug)]
pub struct ChoiceError;

impl<'a> DataSourceTaker<'a> {
    pub fn reset(&mut self, max_choices: usize) {
        self.choices_left = max_choices;
        self.buffer_idx = 0;
    }

    pub fn choice(&mut self, range: usize) -> Result<usize> {
        assert!(range > 0);
        let Some(max_choice) = range.checked_sub(1) else {
            return Ok(0);
        };

        if self.choices_left == 0 {
            return Err(ChoiceError);
        }
        self.choices_left -= 1;

        if self.buffer_idx < self.buffer.len() {
            let data = self.buffer[self.buffer_idx];
            self.buffer_idx += 1;
            assert_eq!(data.max_choice, max_choice, "Choices are reproducible");
            return Ok(data.choice);
        }

        self.buffer.push(Choice {
            choice: 0,
            max_choice,
        });
        self.buffer_idx += 1;
        Ok(0)
    }

    pub fn iter_of<'b, T: Exhaustive>(&'b mut self) -> Result<DataSourceTakerIter<'a, 'b, T>> {
        let max_count = self.choice(self.choices_left + 1)?;
        Ok(DataSourceTakerIter {
            max_count_idx: self.buffer_idx - 1,
            max_count,
            taker: self,
            phantom: Default::default(),
        })
    }
}

pub struct DataSourceTakerIter<'a, 'b, T: Exhaustive> {
    max_count_idx: usize,
    max_count: usize,
    taker: &'b mut DataSourceTaker<'a>,
    phantom: PhantomData<T>,
}

impl<T: Exhaustive> Iterator for DataSourceTakerIter<'_, '_, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.max_count == 0 {
            return None;
        }
        self.max_count -= 1;
        match T::generate(self.taker) {
            Ok(v) => Some(Ok(v)),
            Err(e) => {
                // Record max length reached, higher values of `max_count` won't have any effect
                let max_count_choice = &mut self.taker.buffer[self.max_count_idx];
                max_count_choice.max_choice = max_count_choice.choice;
                Some(Err(e))
            }
        }
    }
}

pub struct DataSource {
    max_choices: usize,
    buffer: Vec<Choice>,
    first_run: bool,
}

impl DataSource {
    pub fn new(max_choices: usize) -> Self {
        Self {
            max_choices,
            buffer: vec![],
            first_run: true,
        }
    }

    pub fn next_run(&mut self) -> Option<DataSourceTaker<'_>> {
        if !self.first_run {
            for i in (0..self.buffer.len()).rev() {
                let entry = &mut self.buffer[i];
                if entry.choice == entry.max_choice {
                    self.buffer.pop();
                } else {
                    entry.choice += 1;
                    break;
                }
            }
            if self.buffer.is_empty() {
                return None;
            }
        }
        self.first_run = false;
        Some(DataSourceTaker {
            choices_left: self.max_choices,
            buffer: &mut self.buffer,
            buffer_idx: 0,
        })
    }
}
