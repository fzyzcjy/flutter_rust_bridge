pub(super) struct DeferredCloseBatches<T> {
    current: Vec<T>,
    next: Vec<T>,
    scheduled: bool,
}

impl<T> Default for DeferredCloseBatches<T> {
    fn default() -> Self {
        Self {
            current: Default::default(),
            next: Default::default(),
            scheduled: false,
        }
    }
}

impl<T> DeferredCloseBatches<T> {
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
        let completed = std::mem::take(&mut self.current);
        if self.next.is_empty() {
            self.scheduled = false;
            return (completed, false);
        }

        self.current = std::mem::take(&mut self.next);
        (completed, true)
    }
}

#[cfg(test)]
mod tests {
    use super::DeferredCloseBatches;

    #[test]
    fn item_added_after_scheduling_waits_for_next_batch() {
        let mut batches = DeferredCloseBatches::default();

        assert!(batches.push(1));
        assert!(!batches.push(2));

        let (first_batch, has_next_batch) = batches.finish_current();
        assert_eq!(first_batch, vec![1]);
        assert!(has_next_batch);

        let (second_batch, has_next_batch) = batches.finish_current();
        assert_eq!(second_batch, vec![2]);
        assert!(!has_next_batch);
    }
}
