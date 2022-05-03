#![macro_use]

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use anyhow::{anyhow, Result};
use log::*;
use parking_lot::{Mutex, RwLock};

pub type SubPoolId = i64;
pub type HandleId = i64;

pub trait PoolObjectHandle: Debug + Eq + Hash + Sized + Clone {
    type Object: Clone;

    fn get_pool() -> &'static Pool<Self>;

    fn sub_pool_id(&self) -> SubPoolId;
    fn handle_id(&self) -> HandleId;

    fn generate_handle(sub_pool_id: SubPoolId) -> Self;
}

#[macro_export]
macro_rules! impl_pool_object_handle {
    ($O:ty, $H:ident) => {
        impl PoolObjectHandle for $H {
            type Object = $O;

            fn get_pool() -> &'static crate::utils::pool::Pool<Self> {
                lazy_static! {
                    static ref POOL: crate::utils::pool::Pool<$H> =
                        crate::utils::pool::Pool::new(stringify!($H).to_string());
                }
                &POOL
            }

            fn generate_handle(sub_pool_id: crate::utils::pool::SubPoolId) -> $H {
                Self(sub_pool_id, rand::thread_rng().gen::<i64>())
            }

            fn sub_pool_id(&self) -> crate::utils::pool::SubPoolId {
                self.0
            }
            fn handle_id(&self) -> crate::utils::pool::HandleId {
                self.1
            }
        }
    };
}

pub struct Pool<H: PoolObjectHandle> {
    name: String,
    // use [RwLock] for sub pools, since most of the times, we will not create/delete a sub-pool
    sub_pool_map: RwLock<HashMap<SubPoolId, SubPool<H::Object>>>,
}

pub trait PoolTrait: Sync {
    fn name(&self) -> &str;

    fn destroy_sub_pool(&self, sub_pool_id: SubPoolId) -> Vec<SubPoolId>;
}

impl<H: PoolObjectHandle + Sync> PoolTrait for Pool<H>
where
    H::Object: Send,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn destroy_sub_pool(&self, sub_pool_id: SubPoolId) -> Vec<SubPoolId> {
        self.sub_pool_map
            .write()
            .remove(&sub_pool_id)
            .map_or(vec![], |sub_pool| sub_pool.keys())
    }
}

impl<H: PoolObjectHandle> Pool<H> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            sub_pool_map: RwLock::new(HashMap::new()),
        }
    }

    pub fn remove_object(&self, handle: H) -> Option<H::Object> {
        self.access_sub_pool(handle.sub_pool_id(), |sub_pool| {
            sub_pool.remove(handle.handle_id())
        })
    }

    pub fn get_cloned_object(&self, handle: H) -> Result<H::Object> {
        self.access_sub_pool(handle.sub_pool_id(), |sub_pool| {
            sub_pool.get_cloned(handle.handle_id())
        })
    }

    pub fn put_object(&self, sub_pool_id: SubPoolId, object: H::Object) -> H {
        let handle = H::generate_handle(sub_pool_id);
        self.access_sub_pool(handle.sub_pool_id(), |sub_pool| {
            sub_pool.put(handle.handle_id(), object)
        });
        handle
    }

    fn access_sub_pool<F, Ret>(&self, sub_pool_id: SubPoolId, f: F) -> Ret
    where
        F: FnOnce(&SubPool<H::Object>) -> Ret,
    {
        let read_guard = self.sub_pool_map.read();
        if let Some(sub_pool) = read_guard.get(&sub_pool_id) {
            return f(sub_pool);
        }
        drop(read_guard);

        // have to create it
        self.sub_pool_map
            .write()
            .insert(sub_pool_id, SubPool::new());

        // and then read again
        f(&self.sub_pool_map.read()[&sub_pool_id])
    }
}

struct SubPool<Obj> {
    object_map: Mutex<HashMap<HandleId, Obj>>,
}

impl<Obj: Clone> SubPool<Obj> {
    fn new() -> Self {
        Self {
            object_map: Mutex::new(HashMap::new()),
        }
    }

    fn remove(&self, handle_id: HandleId) -> Option<Obj> {
        let result = self.object_map.lock().remove(&handle_id);
        if let None = result {
            warn!(
                "Pool cannot `remove` for handle_id={:?} since item does not exist, ignore",
                handle_id
            )
        }
        result
    }

    fn get_cloned(&self, handle_id: HandleId) -> Result<Obj> {
        Ok(self
            .object_map
            .lock()
            .get(&handle_id)
            .ok_or_else(|| {
                anyhow!(
                    "Pool cannot `get` for handle_id={:?} since item does not exist",
                    handle_id
                )
            })?
            .clone())
    }

    fn put(&self, handle_id: HandleId, object: Obj) {
        self.object_map.lock().insert(handle_id, object);
    }

    fn keys(&self) -> Vec<HandleId> {
        self.object_map.lock().keys().copied().collect()
    }
}

pub fn remove<H: PoolObjectHandle + 'static>(handle: H) -> Option<H::Object> {
    H::get_pool().remove_object(handle)
}

pub fn get_cloned<H: PoolObjectHandle + 'static>(handle: H) -> Result<H::Object> {
    H::get_pool().get_cloned_object(handle)
}

pub fn remove_or_get_cloned<H: PoolObjectHandle + 'static>(
    handle: H,
    do_remove: bool,
) -> Result<H::Object> {
    Ok(if do_remove {
        remove(handle).ok_or_else(|| anyhow!("remove() returns null"))?
    } else {
        get_cloned(handle)?
    })
}

pub fn put<H: PoolObjectHandle + 'static>(sub_pool_id: SubPoolId, object: H::Object) -> H {
    H::get_pool().put_object(sub_pool_id, object)
}

pub fn destory_sub_pools(
    pools: &[&'static dyn PoolTrait],
    sub_pool_id: SubPoolId,
) -> Result<String> {
    let mut warnings = vec![];

    for pool in pools.iter() {
        let remain_handle_ids = pool.destroy_sub_pool(sub_pool_id);
        if !remain_handle_ids.is_empty() {
            warnings.push(format!(
                "SubPool(pool_name={}, sub_pool_id={}) has remain_handle_ids={:?} when destroying",
                pool.name(),
                sub_pool_id,
                remain_handle_ids
            ));
        }
    }

    Ok(warnings.join("; "))
}
