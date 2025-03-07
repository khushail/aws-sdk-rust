// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a single value in the forecast data used for predictive scaling.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Datapoint {
    /// <p>The time stamp for the data point in UTC format.</p>
    #[doc(hidden)]
    pub timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The value of the data point.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<f64>,
}
impl Datapoint {
    /// <p>The time stamp for the data point in UTC format.</p>
    pub fn timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
    /// <p>The value of the data point.</p>
    pub fn value(&self) -> ::std::option::Option<f64> {
        self.value
    }
}
impl Datapoint {
    /// Creates a new builder-style object to manufacture [`Datapoint`](crate::types::Datapoint).
    pub fn builder() -> crate::types::builders::DatapointBuilder {
        crate::types::builders::DatapointBuilder::default()
    }
}

/// A builder for [`Datapoint`](crate::types::Datapoint).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DatapointBuilder {
    pub(crate) timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) value: ::std::option::Option<f64>,
}
impl DatapointBuilder {
    /// <p>The time stamp for the data point in UTC format.</p>
    pub fn timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time stamp for the data point in UTC format.</p>
    pub fn set_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.timestamp = input;
        self
    }
    /// <p>The value of the data point.</p>
    pub fn value(mut self, input: f64) -> Self {
        self.value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The value of the data point.</p>
    pub fn set_value(mut self, input: ::std::option::Option<f64>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`Datapoint`](crate::types::Datapoint).
    pub fn build(self) -> crate::types::Datapoint {
        crate::types::Datapoint {
            timestamp: self.timestamp,
            value: self.value,
        }
    }
}
