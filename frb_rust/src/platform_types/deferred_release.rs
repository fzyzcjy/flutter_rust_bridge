pub(super) struct DeferredRelease<T> {
    current: Vec<T>,
    next: Vec<T>,
    scheduled: bool,
}

impl<T> Default for DeferredRelease<T> {
    fn default() -> Self {
        Self {
            current: Default::default(),
            next: Default::default(),
            scheduled: false,
        }
    }
}

impl<T> DeferredRelease<T> {
    pub(super) fn push(&mut self, item: T) -> bool {
        if self.scheduled {
            self.next.push(item);
            return false;
        }

        self.current.push(item);
        self.scheduled = true;
        true
    }

    pub(super) fn finish_current(&mut self) -> (Vec<T>, bool) {
        let finished = std::mem::take(&mut self.current);
        if self.next.is_empty() {
            self.scheduled = false;
            return (finished, false);
        }

        self.current = std::mem::take(&mut self.next);
        (finished, true)
    }
}

#[cfg(test)]
mod tests {
    use super::DeferredRelease;

    /// Gives a late final close a full fallback interval in the next batch.
    #[test]
    fn late_final_close_waits_for_next_fallback_callback() {
        let mut deferred = DeferredRelease::default();

        assert!(deferred.push(1));
        assert!(!deferred.push(2));
        assert_eq!(deferred.finish_current(), (vec![1], true));
        assert_eq!(deferred.finish_current(), (vec![2], false));
    }

    /// Schedules a new fallback after the previous batch has finished.
    #[test]
    fn final_close_after_fallback_schedules_again() {
        let mut deferred = DeferredRelease::default();

        assert!(deferred.push(1));
        assert_eq!(deferred.finish_current(), (vec![1], false));
        assert!(deferred.push(2));
    }
}
