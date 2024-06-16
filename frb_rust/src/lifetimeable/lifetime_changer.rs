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
