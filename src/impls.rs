use std::cell::{Cell, RefCell, UnsafeCell};
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use crate::{ChoiceError, DataSourceTaker, Exhaustive};

impl Exhaustive for bool {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.choice(2)? != 0)
    }
}

impl<T: Exhaustive> Exhaustive for Box<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::generate(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for Rc<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::generate(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for Arc<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::generate(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for Cell<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::generate(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for RefCell<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::generate(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for UnsafeCell<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::generate(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for Mutex<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::generate(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for RwLock<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::generate(u)?))
    }
}

impl Exhaustive for () {
    fn generate(_: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(())
    }
}

impl<T1: Exhaustive> Exhaustive for (T1,) {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok((T1::generate(u)?,))
    }
}

impl<T1: Exhaustive, T2: Exhaustive> Exhaustive for (T1, T2) {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok((T1::generate(u)?,T2::generate(u)?,))
    }
}

impl<T1: Exhaustive, T2: Exhaustive, T3: Exhaustive> Exhaustive for (T1, T2, T3) {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok((T1::generate(u)?,T2::generate(u)?,T3::generate(u)?,))
    }
}

impl<T1: Exhaustive, T2: Exhaustive, T3: Exhaustive, T4: Exhaustive> Exhaustive for (T1, T2, T3, T4) {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok((T1::generate(u)?,T2::generate(u)?,T3::generate(u)?,T4::generate(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for Option<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        if bool::generate(u)? {
            Ok(None)
        } else {
            Ok(Some(T::generate(u)?))
        }
    }
}

impl<T: Exhaustive, E: Exhaustive> Exhaustive for Result<T, E> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        if bool::generate(u)? {
            Ok(Ok(T::generate(u)?))
        } else {
            Ok(Err(E::generate(u)?))
        }
    }
}

impl<T: Exhaustive> Exhaustive for Vec<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}

impl<T: Exhaustive> Exhaustive for LinkedList<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}

impl<T: Exhaustive> Exhaustive for VecDeque<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}


impl<T: Exhaustive + Ord> Exhaustive for BTreeSet<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}

impl<K: Exhaustive + Ord, V: Exhaustive> Exhaustive for BTreeMap<K, V> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<(K, V)>()?.collect::<Result<_, _>>()?)
    }
}

impl<T: Exhaustive + Hash + Eq> Exhaustive for HashSet<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}

impl<K: Exhaustive + Hash + Eq, V: Exhaustive> Exhaustive for HashMap<K, V> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<(K, V)>()?.collect::<Result<_, _>>()?)
    }
}

impl<T: Exhaustive + Ord> Exhaustive for BinaryHeap<T> {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}

impl<const N: usize, T: Exhaustive> Exhaustive for [T; N] {
    fn generate(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok((0..N).map(|_| T::generate(u)).collect::<Result<Vec<_>, _>>()?.try_into().unwrap_or_else(|_| unreachable!()))
    }
}


