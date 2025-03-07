// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration details of the CloudWatch Logs destination.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CloudWatchLogsDestinationDetails {
    /// <p>The name of the CloudWatch Logs log group.</p>
    #[doc(hidden)]
    pub log_group: ::std::option::Option<::std::string::String>,
}
impl CloudWatchLogsDestinationDetails {
    /// <p>The name of the CloudWatch Logs log group.</p>
    pub fn log_group(&self) -> ::std::option::Option<&str> {
        self.log_group.as_deref()
    }
}
impl CloudWatchLogsDestinationDetails {
    /// Creates a new builder-style object to manufacture [`CloudWatchLogsDestinationDetails`](crate::types::CloudWatchLogsDestinationDetails).
    pub fn builder() -> crate::types::builders::CloudWatchLogsDestinationDetailsBuilder {
        crate::types::builders::CloudWatchLogsDestinationDetailsBuilder::default()
    }
}

/// A builder for [`CloudWatchLogsDestinationDetails`](crate::types::CloudWatchLogsDestinationDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CloudWatchLogsDestinationDetailsBuilder {
    pub(crate) log_group: ::std::option::Option<::std::string::String>,
}
impl CloudWatchLogsDestinationDetailsBuilder {
    /// <p>The name of the CloudWatch Logs log group.</p>
    pub fn log_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_group = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the CloudWatch Logs log group.</p>
    pub fn set_log_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_group = input;
        self
    }
    /// Consumes the builder and constructs a [`CloudWatchLogsDestinationDetails`](crate::types::CloudWatchLogsDestinationDetails).
    pub fn build(self) -> crate::types::CloudWatchLogsDestinationDetails {
        crate::types::CloudWatchLogsDestinationDetails {
            log_group: self.log_group,
        }
    }
}
