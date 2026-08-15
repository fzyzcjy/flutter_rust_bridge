pub(super) struct DeferredRelease<T> {
    pending: Vec<T>,
    scheduled: bool,
}

impl<T> Default for DeferredRelease<T> {
    fn default() -> Self {
        Self {
            pending: Default::default(),
            scheduled: false,
        }
    }
}

impl<T> DeferredRelease<T> {
    pub(super) fn push(&mut self, item: T) -> bool {
        self.pending.push(item);
        if self.scheduled {
            return false;
        }

        self.scheduled = true;
        true
    }

    pub(super) fn finish(&mut self) -> Vec<T> {
        self.scheduled = false;
        std::mem::take(&mut self.pending)
    }
}

#[cfg(test)]
mod tests {
    use super::DeferredRelease;

    /// Batches canceled streams behind one pending fallback callback.
    #[test]
    fn unacknowledged_channels_share_one_fallback_callback() {
        let mut deferred = DeferredRelease::default();

        assert!(deferred.push(1));
        assert!(!deferred.push(2));
        assert_eq!(deferred.finish(), vec![1, 2]);
    }

    /// Schedules a new fallback after the previous batch has finished.
    #[test]
    fn final_close_after_fallback_schedules_again() {
        let mut deferred = DeferredRelease::default();

        assert!(deferred.push(1));
        assert_eq!(deferred.finish(), vec![1]);
        assert!(deferred.push(2));
    }
}
