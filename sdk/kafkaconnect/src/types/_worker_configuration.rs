// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration of the workers, which are the processes that run the connector logic.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct WorkerConfiguration {
    /// <p>The revision of the worker configuration.</p>
    #[doc(hidden)]
    pub revision: i64,
    /// <p>The Amazon Resource Name (ARN) of the worker configuration.</p>
    #[doc(hidden)]
    pub worker_configuration_arn: ::std::option::Option<::std::string::String>,
}
impl WorkerConfiguration {
    /// <p>The revision of the worker configuration.</p>
    pub fn revision(&self) -> i64 {
        self.revision
    }
    /// <p>The Amazon Resource Name (ARN) of the worker configuration.</p>
    pub fn worker_configuration_arn(&self) -> ::std::option::Option<&str> {
        self.worker_configuration_arn.as_deref()
    }
}
impl WorkerConfiguration {
    /// Creates a new builder-style object to manufacture [`WorkerConfiguration`](crate::types::WorkerConfiguration).
    pub fn builder() -> crate::types::builders::WorkerConfigurationBuilder {
        crate::types::builders::WorkerConfigurationBuilder::default()
    }
}

/// A builder for [`WorkerConfiguration`](crate::types::WorkerConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct WorkerConfigurationBuilder {
    pub(crate) revision: ::std::option::Option<i64>,
    pub(crate) worker_configuration_arn: ::std::option::Option<::std::string::String>,
}
impl WorkerConfigurationBuilder {
    /// <p>The revision of the worker configuration.</p>
    pub fn revision(mut self, input: i64) -> Self {
        self.revision = ::std::option::Option::Some(input);
        self
    }
    /// <p>The revision of the worker configuration.</p>
    pub fn set_revision(mut self, input: ::std::option::Option<i64>) -> Self {
        self.revision = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the worker configuration.</p>
    pub fn worker_configuration_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.worker_configuration_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the worker configuration.</p>
    pub fn set_worker_configuration_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.worker_configuration_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`WorkerConfiguration`](crate::types::WorkerConfiguration).
    pub fn build(self) -> crate::types::WorkerConfiguration {
        crate::types::WorkerConfiguration {
            revision: self.revision.unwrap_or_default(),
            worker_configuration_arn: self.worker_configuration_arn,
        }
    }
}
