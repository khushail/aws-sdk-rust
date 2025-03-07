// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Predictive Dialer config
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PredictiveDialerConfig {
    /// The bandwidth allocation of a queue resource.
    #[doc(hidden)]
    pub bandwidth_allocation: ::std::option::Option<f64>,
}
impl PredictiveDialerConfig {
    /// The bandwidth allocation of a queue resource.
    pub fn bandwidth_allocation(&self) -> ::std::option::Option<f64> {
        self.bandwidth_allocation
    }
}
impl PredictiveDialerConfig {
    /// Creates a new builder-style object to manufacture [`PredictiveDialerConfig`](crate::types::PredictiveDialerConfig).
    pub fn builder() -> crate::types::builders::PredictiveDialerConfigBuilder {
        crate::types::builders::PredictiveDialerConfigBuilder::default()
    }
}

/// A builder for [`PredictiveDialerConfig`](crate::types::PredictiveDialerConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PredictiveDialerConfigBuilder {
    pub(crate) bandwidth_allocation: ::std::option::Option<f64>,
}
impl PredictiveDialerConfigBuilder {
    /// The bandwidth allocation of a queue resource.
    pub fn bandwidth_allocation(mut self, input: f64) -> Self {
        self.bandwidth_allocation = ::std::option::Option::Some(input);
        self
    }
    /// The bandwidth allocation of a queue resource.
    pub fn set_bandwidth_allocation(mut self, input: ::std::option::Option<f64>) -> Self {
        self.bandwidth_allocation = input;
        self
    }
    /// Consumes the builder and constructs a [`PredictiveDialerConfig`](crate::types::PredictiveDialerConfig).
    pub fn build(self) -> crate::types::PredictiveDialerConfig {
        crate::types::PredictiveDialerConfig {
            bandwidth_allocation: self.bandwidth_allocation,
        }
    }
}
