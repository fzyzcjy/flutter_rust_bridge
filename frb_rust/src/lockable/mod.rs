// Only for generated code, not for normal users
pub trait Lockable {
    type RwLockReadGuard;
    type RwLockWriteGuard;

    fn lockable_decode_sync_ref(&self) -> Self::RwLockReadGuard;

    fn lockable_decode_sync_ref_mut(&self) -> Self::RwLockWriteGuard;
}
