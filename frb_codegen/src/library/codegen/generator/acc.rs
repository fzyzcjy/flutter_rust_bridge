use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::enum_map;
use std::iter::FromIterator;
use std::ops::AddAssign;

// Generic accumulator over the targets.
//
// [`Acc<Option<String>>`] implements <code>[From]\<impl [ToString]></code>
// for code shared between all platforms.
enum_map!(
    Acc, TargetOrCommon;
    Common, Io, Web;
    common, io, web;
);

impl<T> AddAssign for Acc<Vec<T>> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.common.extend(rhs.common);
        self.io.extend(rhs.io);
        self.web.extend(rhs.web);
    }
}

// TODO rm? codecov says this is unused
// impl<T> Extend<Acc<T>> for Acc<Vec<T>> {
//     fn extend<A: IntoIterator<Item = Acc<T>>>(&mut self, iter: A) {
//         for i in iter {
//             self.push_acc(i)
//         }
//     }
// }

impl<T> FromIterator<Acc<T>> for Acc<Vec<T>> {
    fn from_iter<A: IntoIterator<Item = Acc<T>>>(iter: A) -> Self {
        iter.into_iter()
            .fold(Acc::<Vec<T>>::default(), |mut acc, x| {
                acc.push_acc(x);
                acc
            })
    }
}

impl<T> FromIterator<Acc<Vec<T>>> for Acc<Vec<T>> {
    fn from_iter<A: IntoIterator<Item = Acc<Vec<T>>>>(iter: A) -> Self {
        iter.into_iter()
            .fold(Acc::<Vec<T>>::default(), |mut acc, x| {
                acc += x;
                acc
            })
    }
}

impl<T> Acc<T> {
    pub fn new(mut init: impl FnMut(TargetOrCommon) -> T) -> Acc<T> {
        Acc {
            common: init(TargetOrCommon::Common),
            io: init(TargetOrCommon::Io),
            web: init(TargetOrCommon::Web),
        }
    }

    pub fn new_io(io: T) -> Acc<T>
    where
        T: Default,
    {
        Acc {
            io,
            ..Default::default()
        }
    }

    pub fn new_io_web(value: T) -> Acc<T>
    where
        T: Default + Clone,
    {
        Acc {
            io: value.clone(),
            web: value,
            ..Default::default()
        }
    }

    pub fn new_common(common: T) -> Acc<T>
    where
        T: Default,
    {
        Acc {
            common,
            ..Default::default()
        }
    }

    pub fn map<O>(self, mut mapper: impl FnMut(T, TargetOrCommon) -> O) -> Acc<O> {
        Acc {
            common: mapper(self.common, TargetOrCommon::Common),
            io: mapper(self.io, TargetOrCommon::Io),
            web: mapper(self.web, TargetOrCommon::Web),
        }
    }

    pub fn map_ref<O>(&self, mut mapper: impl FnMut(&T, TargetOrCommon) -> O) -> Acc<O> {
        Acc {
            common: mapper(&self.common, TargetOrCommon::Common),
            io: mapper(&self.io, TargetOrCommon::Io),
            web: mapper(&self.web, TargetOrCommon::Web),
        }
    }

    /// Assign this value to all non-common targets.
    pub fn distribute(value: T) -> Self
    where
        T: Clone + Default,
    {
        Self {
            common: T::default(),
            io: value.clone(),
            web: value,
        }
    }
}

impl<T: ToString> From<T> for Acc<Option<String>> {
    #[inline]
    fn from(common: T) -> Self {
        Acc {
            common: Some(common.to_string()),
            ..Default::default()
        }
    }
}

impl<T> Acc<Vec<T>> {
    #[inline]
    pub fn push_acc(&mut self, acc: Acc<T>) {
        let Acc { common, io, web } = acc;
        self.common.push(common);
        self.io.push(io);
        self.web.push(web);
    }
}
