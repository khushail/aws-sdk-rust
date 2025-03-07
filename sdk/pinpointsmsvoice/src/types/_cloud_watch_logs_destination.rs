// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// An object that contains information about an event destination that sends data to Amazon CloudWatch Logs.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CloudWatchLogsDestination {
    /// The Amazon Resource Name (ARN) of an Amazon Identity and Access Management (IAM) role that is able to write event data to an Amazon CloudWatch destination.
    #[doc(hidden)]
    pub iam_role_arn: ::std::option::Option<::std::string::String>,
    /// The name of the Amazon CloudWatch Log Group that you want to record events in.
    #[doc(hidden)]
    pub log_group_arn: ::std::option::Option<::std::string::String>,
}
impl CloudWatchLogsDestination {
    /// The Amazon Resource Name (ARN) of an Amazon Identity and Access Management (IAM) role that is able to write event data to an Amazon CloudWatch destination.
    pub fn iam_role_arn(&self) -> ::std::option::Option<&str> {
        self.iam_role_arn.as_deref()
    }
    /// The name of the Amazon CloudWatch Log Group that you want to record events in.
    pub fn log_group_arn(&self) -> ::std::option::Option<&str> {
        self.log_group_arn.as_deref()
    }
}
impl CloudWatchLogsDestination {
    /// Creates a new builder-style object to manufacture [`CloudWatchLogsDestination`](crate::types::CloudWatchLogsDestination).
    pub fn builder() -> crate::types::builders::CloudWatchLogsDestinationBuilder {
        crate::types::builders::CloudWatchLogsDestinationBuilder::default()
    }
}

/// A builder for [`CloudWatchLogsDestination`](crate::types::CloudWatchLogsDestination).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CloudWatchLogsDestinationBuilder {
    pub(crate) iam_role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) log_group_arn: ::std::option::Option<::std::string::String>,
}
impl CloudWatchLogsDestinationBuilder {
    /// The Amazon Resource Name (ARN) of an Amazon Identity and Access Management (IAM) role that is able to write event data to an Amazon CloudWatch destination.
    pub fn iam_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.iam_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The Amazon Resource Name (ARN) of an Amazon Identity and Access Management (IAM) role that is able to write event data to an Amazon CloudWatch destination.
    pub fn set_iam_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.iam_role_arn = input;
        self
    }
    /// The name of the Amazon CloudWatch Log Group that you want to record events in.
    pub fn log_group_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.log_group_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the Amazon CloudWatch Log Group that you want to record events in.
    pub fn set_log_group_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.log_group_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`CloudWatchLogsDestination`](crate::types::CloudWatchLogsDestination).
    pub fn build(self) -> crate::types::CloudWatchLogsDestination {
        crate::types::CloudWatchLogsDestination {
            iam_role_arn: self.iam_role_arn,
            log_group_arn: self.log_group_arn,
        }
    }
}
