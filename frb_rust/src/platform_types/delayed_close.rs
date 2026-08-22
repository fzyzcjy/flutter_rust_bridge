#[derive(Debug)]
pub(super) struct DelayedCloseBatch<T> {
    scheduled: Vec<T>,
    pending: Vec<T>,
}

impl<T> Default for DelayedCloseBatch<T> {
    fn default() -> Self {
        Self {
            scheduled: Default::default(),
            pending: Default::default(),
        }
    }
}

impl<T> DelayedCloseBatch<T> {
    pub(super) fn push(&mut self, value: T) -> bool {
        if self.scheduled.is_empty() {
            self.scheduled.push(value);
            true
        } else {
            self.pending.push(value);
            false
        }
    }

    pub(super) fn complete(&mut self) -> (Vec<T>, bool) {
        let completed = std::mem::take(&mut self.scheduled);
        std::mem::swap(&mut self.scheduled, &mut self.pending);
        let has_next_batch = !self.scheduled.is_empty();

        (completed, has_next_batch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Defers values added during a scheduled delay to the next batch.
    #[test]
    fn complete_keeps_pending_values_for_next_batch() {
        let mut batch = DelayedCloseBatch::default();

        assert!(batch.push(1));
        assert!(!batch.push(2));
        assert_eq!(batch.complete(), (vec![1], true));

        assert!(!batch.push(3));
        assert_eq!(batch.complete(), (vec![2], true));
        assert_eq!(batch.complete(), (vec![3], false));
    }
}
