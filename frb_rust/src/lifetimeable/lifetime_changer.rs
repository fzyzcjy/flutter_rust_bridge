/// Copied from `ouroboros`: https://github.com/someguynamedjosh/ouroboros/blob/7316c75b988ce97140c824837253161bc411eb22/ouroboros/src/lib.rs#L391
///
/// # Safety
///
/// Please refer to their doc
pub unsafe fn ouroboros_change_lifetime<'old, 'new: 'old, T: 'new>(data: &'old T) -> &'new T {
    &*(data as *const _)
}

/// # Safety
///
/// Please refer to `change_lifetime`
pub unsafe fn ouroboros_change_lifetime_mut<'old, 'new: 'old, T: 'new>(
    data: &'old mut T,
) -> &'new mut T {
    &mut *(data as *mut _)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Preserves the referenced value when changing an immutable lifetime.
    fn changes_immutable_lifetime() {
        let value = 42;
        let reference = unsafe { ouroboros_change_lifetime(&value) };

        assert_eq!(*reference, 42);
    }

    #[test]
    /// Preserves mutable access when changing a lifetime.
    fn changes_mutable_lifetime() {
        let mut value = 42;
        let reference = unsafe { ouroboros_change_lifetime_mut(&mut value) };
        *reference = 100;

        assert_eq!(value, 100);
    }
}
