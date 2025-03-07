// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The stream and role Amazon Resource Names (ARNs) for a Kinesis data stream used as the source for a delivery stream.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KinesisStreamSourceConfiguration {
    /// <p>The ARN of the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Kinesis Data Streams ARN Format</a>.</p>
    #[doc(hidden)]
    pub kinesis_stream_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the role that provides access to the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">Amazon Web Services Identity and Access Management (IAM) ARN Format</a>.</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
}
impl KinesisStreamSourceConfiguration {
    /// <p>The ARN of the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Kinesis Data Streams ARN Format</a>.</p>
    pub fn kinesis_stream_arn(&self) -> ::std::option::Option<&str> {
        self.kinesis_stream_arn.as_deref()
    }
    /// <p>The ARN of the role that provides access to the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">Amazon Web Services Identity and Access Management (IAM) ARN Format</a>.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
}
impl KinesisStreamSourceConfiguration {
    /// Creates a new builder-style object to manufacture [`KinesisStreamSourceConfiguration`](crate::types::KinesisStreamSourceConfiguration).
    pub fn builder() -> crate::types::builders::KinesisStreamSourceConfigurationBuilder {
        crate::types::builders::KinesisStreamSourceConfigurationBuilder::default()
    }
}

/// A builder for [`KinesisStreamSourceConfiguration`](crate::types::KinesisStreamSourceConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KinesisStreamSourceConfigurationBuilder {
    pub(crate) kinesis_stream_arn: ::std::option::Option<::std::string::String>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
}
impl KinesisStreamSourceConfigurationBuilder {
    /// <p>The ARN of the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Kinesis Data Streams ARN Format</a>.</p>
    pub fn kinesis_stream_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.kinesis_stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Kinesis Data Streams ARN Format</a>.</p>
    pub fn set_kinesis_stream_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.kinesis_stream_arn = input;
        self
    }
    /// <p>The ARN of the role that provides access to the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">Amazon Web Services Identity and Access Management (IAM) ARN Format</a>.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the role that provides access to the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">Amazon Web Services Identity and Access Management (IAM) ARN Format</a>.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`KinesisStreamSourceConfiguration`](crate::types::KinesisStreamSourceConfiguration).
    pub fn build(self) -> crate::types::KinesisStreamSourceConfiguration {
        crate::types::KinesisStreamSourceConfiguration {
            kinesis_stream_arn: self.kinesis_stream_arn,
            role_arn: self.role_arn,
        }
    }
}
