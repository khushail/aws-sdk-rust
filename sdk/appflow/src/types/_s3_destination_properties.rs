// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The properties that are applied when Amazon S3 is used as a destination. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3DestinationProperties {
    /// <p> The Amazon S3 bucket name in which Amazon AppFlow places the transferred data. </p>
    #[doc(hidden)]
    pub bucket_name: ::std::option::Option<::std::string::String>,
    /// <p> The object key for the destination bucket in which Amazon AppFlow places the files. </p>
    #[doc(hidden)]
    pub bucket_prefix: ::std::option::Option<::std::string::String>,
    /// <p> The configuration that determines how Amazon AppFlow should format the flow output data when Amazon S3 is used as the destination. </p>
    #[doc(hidden)]
    pub s3_output_format_config: ::std::option::Option<crate::types::S3OutputFormatConfig>,
}
impl S3DestinationProperties {
    /// <p> The Amazon S3 bucket name in which Amazon AppFlow places the transferred data. </p>
    pub fn bucket_name(&self) -> ::std::option::Option<&str> {
        self.bucket_name.as_deref()
    }
    /// <p> The object key for the destination bucket in which Amazon AppFlow places the files. </p>
    pub fn bucket_prefix(&self) -> ::std::option::Option<&str> {
        self.bucket_prefix.as_deref()
    }
    /// <p> The configuration that determines how Amazon AppFlow should format the flow output data when Amazon S3 is used as the destination. </p>
    pub fn s3_output_format_config(
        &self,
    ) -> ::std::option::Option<&crate::types::S3OutputFormatConfig> {
        self.s3_output_format_config.as_ref()
    }
}
impl S3DestinationProperties {
    /// Creates a new builder-style object to manufacture [`S3DestinationProperties`](crate::types::S3DestinationProperties).
    pub fn builder() -> crate::types::builders::S3DestinationPropertiesBuilder {
        crate::types::builders::S3DestinationPropertiesBuilder::default()
    }
}

/// A builder for [`S3DestinationProperties`](crate::types::S3DestinationProperties).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct S3DestinationPropertiesBuilder {
    pub(crate) bucket_name: ::std::option::Option<::std::string::String>,
    pub(crate) bucket_prefix: ::std::option::Option<::std::string::String>,
    pub(crate) s3_output_format_config: ::std::option::Option<crate::types::S3OutputFormatConfig>,
}
impl S3DestinationPropertiesBuilder {
    /// <p> The Amazon S3 bucket name in which Amazon AppFlow places the transferred data. </p>
    pub fn bucket_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The Amazon S3 bucket name in which Amazon AppFlow places the transferred data. </p>
    pub fn set_bucket_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket_name = input;
        self
    }
    /// <p> The object key for the destination bucket in which Amazon AppFlow places the files. </p>
    pub fn bucket_prefix(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.bucket_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The object key for the destination bucket in which Amazon AppFlow places the files. </p>
    pub fn set_bucket_prefix(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.bucket_prefix = input;
        self
    }
    /// <p> The configuration that determines how Amazon AppFlow should format the flow output data when Amazon S3 is used as the destination. </p>
    pub fn s3_output_format_config(mut self, input: crate::types::S3OutputFormatConfig) -> Self {
        self.s3_output_format_config = ::std::option::Option::Some(input);
        self
    }
    /// <p> The configuration that determines how Amazon AppFlow should format the flow output data when Amazon S3 is used as the destination. </p>
    pub fn set_s3_output_format_config(
        mut self,
        input: ::std::option::Option<crate::types::S3OutputFormatConfig>,
    ) -> Self {
        self.s3_output_format_config = input;
        self
    }
    /// Consumes the builder and constructs a [`S3DestinationProperties`](crate::types::S3DestinationProperties).
    pub fn build(self) -> crate::types::S3DestinationProperties {
        crate::types::S3DestinationProperties {
            bucket_name: self.bucket_name,
            bucket_prefix: self.bucket_prefix,
            s3_output_format_config: self.s3_output_format_config,
        }
    }
}
