pub enum BuildRunnerRegression {
    Value { value: String },
}

pub fn build_runner_regression(value: String) -> BuildRunnerRegression {
    BuildRunnerRegression::Value { value }
}
