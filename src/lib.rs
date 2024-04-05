use std::iter;
use std::marker::PhantomData;

pub use exhaustive_macros::Exhaustive;
pub use exhaustive_macros::exhaustive_test;

mod impls;

pub trait Exhaustive: Sized {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self>;

    fn iter_exhaustive(max_length: usize) -> impl Iterator<Item=Self> {
        let mut source = DataSource::new(max_length);
        iter::from_fn(move || source.next_run().map(|mut u| Self::arbitrary(&mut u))).flatten()
    }
}

pub struct DataSourceTaker<'a> {
    choices_left: usize,
    buffer_data: &'a mut Vec<usize>,
    buffer_data_max: &'a mut Vec<usize>,
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
        if range == 1 {
            return Ok(0)
        }

        if self.choices_left == 0 {
            return Err(ChoiceError)
        }
        self.choices_left -= 1;

        if self.buffer_idx < self.buffer_data.len() {
            let data = self.buffer_data[self.buffer_idx];
            self.buffer_idx += 1;
            return Ok(data)
        }

        self.buffer_data.push(0);
        self.buffer_data_max.push(range - 1);
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
            return None
        }
        self.max_count -= 1;
        match T::arbitrary(self.taker) {
            Ok(v) => Some(Ok(v)),
            Err(e) => {
                // Record max length reached
                self.taker.buffer_data_max[self.max_count_idx] = self.taker.buffer_data[self.max_count_idx];
                Some(Err(e))
            }
        }
    }
}

pub struct DataSource {
    max_choices: usize,
    buffer_data: Vec<usize>,
    buffer_data_max: Vec<usize>,
    first_run: bool
}

impl DataSource {
    pub fn new(max_choices: usize) -> Self {
        Self {
            max_choices,
            buffer_data: vec![],
            buffer_data_max: vec![],
            first_run: true
        }
    }

    pub fn next_run(&mut self) -> Option<DataSourceTaker> {
        if !self.first_run {
            for i in (0..self.buffer_data.len()).rev() {
                if self.buffer_data[i] == self.buffer_data_max[i] {
                    self.buffer_data.pop();
                    self.buffer_data_max.pop();
                } else {
                    self.buffer_data[i] += 1;
                    break;
                }
            }
            if self.buffer_data.len() == 0 {
                return None
            }
        }
        self.first_run = false;
        Some(DataSourceTaker {
            choices_left: self.max_choices,
            buffer_data: &mut self.buffer_data,
            buffer_data_max: &mut self.buffer_data_max,
            buffer_idx: 0,
        })
    }
}
