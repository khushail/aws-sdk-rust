// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Quantifies the anomaly. The higher score means that it's more anomalous. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AnomalyScore {
    /// <p>The maximum score that's observed during the <code>AnomalyDateInterval</code>. </p>
    #[doc(hidden)]
    pub max_score: f64,
    /// <p>The last observed score. </p>
    #[doc(hidden)]
    pub current_score: f64,
}
impl AnomalyScore {
    /// <p>The maximum score that's observed during the <code>AnomalyDateInterval</code>. </p>
    pub fn max_score(&self) -> f64 {
        self.max_score
    }
    /// <p>The last observed score. </p>
    pub fn current_score(&self) -> f64 {
        self.current_score
    }
}
impl AnomalyScore {
    /// Creates a new builder-style object to manufacture [`AnomalyScore`](crate::types::AnomalyScore).
    pub fn builder() -> crate::types::builders::AnomalyScoreBuilder {
        crate::types::builders::AnomalyScoreBuilder::default()
    }
}

/// A builder for [`AnomalyScore`](crate::types::AnomalyScore).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AnomalyScoreBuilder {
    pub(crate) max_score: ::std::option::Option<f64>,
    pub(crate) current_score: ::std::option::Option<f64>,
}
impl AnomalyScoreBuilder {
    /// <p>The maximum score that's observed during the <code>AnomalyDateInterval</code>. </p>
    pub fn max_score(mut self, input: f64) -> Self {
        self.max_score = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum score that's observed during the <code>AnomalyDateInterval</code>. </p>
    pub fn set_max_score(mut self, input: ::std::option::Option<f64>) -> Self {
        self.max_score = input;
        self
    }
    /// <p>The last observed score. </p>
    pub fn current_score(mut self, input: f64) -> Self {
        self.current_score = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last observed score. </p>
    pub fn set_current_score(mut self, input: ::std::option::Option<f64>) -> Self {
        self.current_score = input;
        self
    }
    /// Consumes the builder and constructs a [`AnomalyScore`](crate::types::AnomalyScore).
    pub fn build(self) -> crate::types::AnomalyScore {
        crate::types::AnomalyScore {
            max_score: self.max_score.unwrap_or_default(),
            current_score: self.current_score.unwrap_or_default(),
        }
    }
}
