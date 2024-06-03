// use std::collections::hash_map::Entry::{Occupied, Vacant};
// use std::collections::HashMap;
// use std::hash::Hash;
//
// pub(crate) struct SimpleCache<K, V>(HashMap<K, V>);
//
// impl<K, V> Default for SimpleCache<K, V> {
//     fn default() -> Self {
//         Self(HashMap::new())
//     }
// }
//
// impl<K: Eq + Hash, V> SimpleCache<K, V> {
//     pub(crate) fn get_or_insert(
//         &mut self,
//         key: K,
//         inserter: impl FnOnce() -> anyhow::Result<V>,
//     ) -> anyhow::Result<&mut V> {
//         Ok(match self.0.entry(key) {
//             Occupied(entry) => entry.into_mut(),
//             Vacant(entry) => entry.insert(inserter()?),
//         })
//     }
// }
