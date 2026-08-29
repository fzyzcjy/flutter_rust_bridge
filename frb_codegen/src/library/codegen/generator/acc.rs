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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Keeps target slots distinct while constructing and mapping values.
    fn constructs_and_maps_each_target_slot() {
        let acc = Acc::new(|target| target.to_string());
        let mapped = acc.map(|value, target| format!("{target}:{value}"));

        assert_eq!(mapped.common, "Common:Common");
        assert_eq!(mapped.io, "Io:Io");
        assert_eq!(mapped.web, "Web:Web");
    }

    #[test]
    /// Maps borrowed values without consuming their original target slots.
    fn maps_references_without_consuming_values() {
        let acc = Acc::new(|target| target.to_string());
        let mapped = acc.map_ref(|value, target| format!("{target}:{value}"));

        assert_eq!(mapped.common, "Common:Common");
        assert_eq!(acc.io, "Io");
    }

    #[test]
    /// Initializes common and platform-specific accumulators in their intended slots.
    fn initializes_common_and_platform_specific_values() {
        assert_eq!(Acc::new_common("common").common, "common");
        assert_eq!(Acc::new_io("io").io, "io");

        let distributed = Acc::new_io_web("platform");
        assert_eq!(distributed.common, "");
        assert_eq!(distributed.io, "platform");
        assert_eq!(distributed.web, "platform");

        let distribute = Acc::distribute("distributed");
        assert_eq!(distribute.common, "");
        assert_eq!(distribute.io, "distributed");
        assert_eq!(distribute.web, "distributed");
    }

    #[test]
    /// Converts a shared string into only the common optional code slot.
    fn converts_common_values_to_optional_code() {
        let acc: Acc<Option<String>> = "shared".into();

        assert_eq!(acc.common.as_deref(), Some("shared"));
        assert_eq!(acc.io, None);
        assert_eq!(acc.web, None);
    }

    #[test]
    /// Appends a scalar accumulator to each matching target vector.
    fn pushes_each_accumulator_slot_explicitly() {
        let mut values = Acc::<Vec<_>>::default();
        values.push_acc(Acc {
            common: 1,
            io: 2,
            web: 3,
        });

        assert_eq!(values.common, vec![1]);
        assert_eq!(values.io, vec![2]);
        assert_eq!(values.web, vec![3]);
    }

    #[test]
    /// Merges vector accumulators into their matching target slots.
    fn add_assign_merges_matching_target_vectors() {
        let mut left = Acc {
            common: vec![1],
            io: vec![2],
            web: vec![3],
        };
        left += Acc {
            common: vec![4],
            io: vec![5],
            web: vec![6],
        };

        assert_eq!(left.common, vec![1, 4]);
        assert_eq!(left.io, vec![2, 5]);
        assert_eq!(left.web, vec![3, 6]);
    }

    #[test]
    /// Preserves target order when collecting scalar and vector accumulators.
    fn collects_and_merges_accumulators_in_target_order() {
        let scalar: Acc<Vec<_>> = [Acc::new_common(1), Acc::new_io_web(2)]
            .into_iter()
            .collect();
        assert_eq!(scalar.common, vec![1, 0]);
        assert_eq!(scalar.io, vec![0, 2]);
        assert_eq!(scalar.web, vec![0, 2]);

        let vectors: Acc<Vec<i32>> = [
            Acc {
                common: vec![1],
                io: vec![2],
                web: vec![3],
            },
            Acc {
                common: vec![4],
                io: vec![5],
                web: vec![6],
            },
        ]
        .into_iter()
        .collect();
        assert_eq!(vectors.common, vec![1, 4]);
        assert_eq!(vectors.io, vec![2, 5]);
        assert_eq!(vectors.web, vec![3, 6]);
    }
}
