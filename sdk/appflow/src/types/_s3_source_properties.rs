// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The properties that are applied when Amazon S3 is being used as the flow source. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3SourceProperties {
    /// <p> The Amazon S3 bucket name where the source files are stored. </p>
    #[doc(hidden)]
    pub bucket_name: ::std::option::Option<::std::string::String>,
    /// <p> The object key for the Amazon S3 bucket in which the source files are stored. </p>
    #[doc(hidden)]
    pub bucket_prefix: ::std::option::Option<::std::string::String>,
    /// <p> When you use Amazon S3 as the source, the configuration format that you provide the flow input data. </p>
    #[doc(hidden)]
    pub s3_input_format_config: ::std::option::Option<crate::types::S3InputFormatConfig>,
}
impl S3SourceProperties {
    /// <p> The Amazon S3 bucket name where the source files are stored. </p>
    pub fn bucket_name(&self) -> ::std::option::Option<&str> {
        self.bucket_name.as_deref()
    }
    /// <p> The object key for the Amazon S3 bucket in which the source files are stored. </p>
    pub fn bucket_prefix(&self) -> ::std::option::Option<&str> {
        self.bucket_prefix.as_deref()
    }
    /// <p> When you use Amazon S3 as the source, the configuration format that you provide the flow input data. </p>
    pub fn s3_input_format_config(
        &self,
    ) -> ::std::option::Option<&crate::types::S3InputFormatConfig> {
        self.s3_input_format_config.as_ref()
    }
}
impl S3SourceProperties {
    /// Creates a new builder-style object to manufacture [`S3SourceProperties`](crate::types::S3SourceProperties).
    pub fn builder() -> crate::types::builders::S3SourcePropertiesBuilder {
        crate::types::builders::S3SourcePropertiesBuilder::default()
    }
}

/// A builder for [`S3SourceProperties`](crate::types::S3SourceProperties).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct S3SourcePropertiesBuilder {
    pub(crate) bucket_name: ::std::option::Option<::std::string::String>,
    pub(crate) bucket_prefix: ::std::option::Option<::std::string::String>,
    pub(crate) s3_input_format_config: ::std::option::Option<crate::types::S3InputFormatConfig>,
}
impl S3SourcePropertiesBuilder {
    /// <p> The Amazon S3 bucket name where the source files are stored. </p>
    pub fn bucket_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The Amazon S3 bucket name where the source files are stored. </p>
    pub fn set_bucket_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket_name = input;
        self
    }
    /// <p> The object key for the Amazon S3 bucket in which the source files are stored. </p>
    pub fn bucket_prefix(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.bucket_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The object key for the Amazon S3 bucket in which the source files are stored. </p>
    pub fn set_bucket_prefix(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.bucket_prefix = input;
        self
    }
    /// <p> When you use Amazon S3 as the source, the configuration format that you provide the flow input data. </p>
    pub fn s3_input_format_config(mut self, input: crate::types::S3InputFormatConfig) -> Self {
        self.s3_input_format_config = ::std::option::Option::Some(input);
        self
    }
    /// <p> When you use Amazon S3 as the source, the configuration format that you provide the flow input data. </p>
    pub fn set_s3_input_format_config(
        mut self,
        input: ::std::option::Option<crate::types::S3InputFormatConfig>,
    ) -> Self {
        self.s3_input_format_config = input;
        self
    }
    /// Consumes the builder and constructs a [`S3SourceProperties`](crate::types::S3SourceProperties).
    pub fn build(self) -> crate::types::S3SourceProperties {
        crate::types::S3SourceProperties {
            bucket_name: self.bucket_name,
            bucket_prefix: self.bucket_prefix,
            s3_input_format_config: self.s3_input_format_config,
        }
    }
}
