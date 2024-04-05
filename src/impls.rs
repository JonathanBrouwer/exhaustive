use std::cell::{Cell, RefCell, UnsafeCell};
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use crate::{ChoiceError, DataSourceTaker, Exhaustive};

impl Exhaustive for bool {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.choice(2)? != 0)
    }
}

impl<T: Exhaustive> Exhaustive for Box<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::arbitrary(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for Rc<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::arbitrary(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for Arc<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::arbitrary(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for Cell<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::arbitrary(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for RefCell<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::arbitrary(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for UnsafeCell<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::arbitrary(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for Mutex<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::arbitrary(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for RwLock<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(Self::new(T::arbitrary(u)?))
    }
}

impl Exhaustive for () {
    fn arbitrary(_: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(())
    }
}

impl<T1: Exhaustive> Exhaustive for (T1,) {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok((T1::arbitrary(u)?,))
    }
}

impl<T1: Exhaustive, T2: Exhaustive> Exhaustive for (T1, T2) {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok((T1::arbitrary(u)?,T2::arbitrary(u)?,))
    }
}

impl<T1: Exhaustive, T2: Exhaustive, T3: Exhaustive> Exhaustive for (T1, T2, T3) {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok((T1::arbitrary(u)?,T2::arbitrary(u)?,T3::arbitrary(u)?,))
    }
}

impl<T1: Exhaustive, T2: Exhaustive, T3: Exhaustive, T4: Exhaustive> Exhaustive for (T1, T2, T3, T4) {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok((T1::arbitrary(u)?,T2::arbitrary(u)?,T3::arbitrary(u)?,T4::arbitrary(u)?))
    }
}

impl<T: Exhaustive> Exhaustive for Option<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        if bool::arbitrary(u)? {
            Ok(None)
        } else {
            Ok(Some(T::arbitrary(u)?))
        }
    }
}

impl<T: Exhaustive, E: Exhaustive> Exhaustive for Result<T, E> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        if bool::arbitrary(u)? {
            Ok(Ok(T::arbitrary(u)?))
        } else {
            Ok(Err(E::arbitrary(u)?))
        }
    }
}

impl<T: Exhaustive> Exhaustive for Vec<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}

impl<T: Exhaustive> Exhaustive for LinkedList<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}

impl<T: Exhaustive> Exhaustive for VecDeque<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}


impl<T: Exhaustive + Ord> Exhaustive for BTreeSet<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}

impl<K: Exhaustive + Ord, V: Exhaustive> Exhaustive for BTreeMap<K, V> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<(K, V)>()?.collect::<Result<_, _>>()?)
    }
}

impl<T: Exhaustive + Hash + Eq> Exhaustive for HashSet<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}

impl<K: Exhaustive + Hash + Eq, V: Exhaustive> Exhaustive for HashMap<K, V> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<(K, V)>()?.collect::<Result<_, _>>()?)
    }
}

impl<T: Exhaustive + Ord> Exhaustive for BinaryHeap<T> {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok(u.iter_of::<T>()?.collect::<Result<_, _>>()?)
    }
}

impl<const N: usize, T: Exhaustive> Exhaustive for [T; N] {
    fn arbitrary(u: &mut DataSourceTaker) -> Result<Self, ChoiceError> {
        Ok((0..N).map(|_| T::arbitrary(u)).collect::<Result<Vec<_>, _>>()?.try_into().unwrap_or_else(|_| unreachable!()))
    }
}


