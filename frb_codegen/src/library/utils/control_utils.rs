// https://stackoverflow.com/questions/63644071/idiomatic-way-to-filter-values-matching-a-pattern-in-an-iterator
#[doc(hidden)] // only to be used within crate, not for end users
#[macro_export]
macro_rules! if_then_some {
    ($cond: expr, $val: expr) => {
        if $cond {
            Some($val)
        } else {
            None
        }
    };
    (let $pattern:pat = $expr: expr, $val: expr) => {
        if let $pattern = $expr {
            Some($val)
        } else {
            None
        }
    };
}

#[cfg(test)]
mod tests {
    /// Evaluates the value expression only when the condition is true.
    #[test]
    fn test_if_then_some_expression_arm_returns_some_or_none_lazily() {
        let mut condition_evaluations = 0;
        let mut value_evaluations = 0;

        let present = crate::if_then_some!(
            {
                condition_evaluations += 1;
                true
            },
            {
                value_evaluations += 1;
                "present"
            }
        );
        let absent = crate::if_then_some!(false, {
            value_evaluations += 1;
            "absent"
        });

        assert_eq!(present, Some("present"));
        assert_eq!(absent, None);
        assert_eq!(condition_evaluations, 1);
        assert_eq!(value_evaluations, 1);
    }

    /// Evaluates a false condition once without evaluating its value expression.
    #[test]
    fn test_if_then_some_expression_arm_evaluates_false_condition_once() {
        let mut condition_evaluations = 0;
        let mut value_evaluations = 0;

        let result = crate::if_then_some!(
            {
                condition_evaluations += 1;
                false
            },
            {
                value_evaluations += 1;
                "unreachable"
            }
        );

        assert_eq!(result, None);
        assert_eq!(condition_evaluations, 1);
        assert_eq!(value_evaluations, 0);
    }

    /// Returns Some only for matching patterns without evaluating absent values.
    #[test]
    fn test_if_then_some_pattern_arm_matches_and_is_lazy() {
        let mut expression_evaluations = 0;
        let mut value_evaluations = 0;

        let matched = crate::if_then_some!(
            let Some(value) = {
                expression_evaluations += 1;
                Some(3)
            },
            {
                value_evaluations += 1;
                value * 2
            }
        );
        let unmatched = crate::if_then_some!(let Some(value) = None::<u8>, {
            value_evaluations += 1;
            value * 2
        });

        assert_eq!(matched, Some(6));
        assert_eq!(unmatched, None);
        assert_eq!(expression_evaluations, 1);
        assert_eq!(value_evaluations, 1);
    }

    /// Evaluates an unmatched expression once without evaluating its value expression.
    #[test]
    fn test_if_then_some_pattern_arm_evaluates_unmatched_expression_once() {
        let mut expression_evaluations = 0;
        let mut value_evaluations = 0;

        let result = crate::if_then_some!(
            let Some(value) = {
                expression_evaluations += 1;
                None::<u8>
            },
            {
                value_evaluations += 1;
                value * 2
            }
        );

        assert_eq!(result, None);
        assert_eq!(expression_evaluations, 1);
        assert_eq!(value_evaluations, 0);
    }
}
