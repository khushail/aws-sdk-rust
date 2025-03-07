// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The information retrieved from the Amazon EC2 instances.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnvironmentInfoDescription {
    /// <p>The type of information retrieved.</p>
    #[doc(hidden)]
    pub info_type: ::std::option::Option<crate::types::EnvironmentInfoType>,
    /// <p>The Amazon EC2 Instance ID for this information.</p>
    #[doc(hidden)]
    pub ec2_instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The time stamp when this information was retrieved.</p>
    #[doc(hidden)]
    pub sample_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The retrieved information. Currently contains a presigned Amazon S3 URL. The files are deleted after 15 minutes.</p>
    /// <p>Anyone in possession of this URL can access the files before they are deleted. Make the URL available only to trusted parties.</p>
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
}
impl EnvironmentInfoDescription {
    /// <p>The type of information retrieved.</p>
    pub fn info_type(&self) -> ::std::option::Option<&crate::types::EnvironmentInfoType> {
        self.info_type.as_ref()
    }
    /// <p>The Amazon EC2 Instance ID for this information.</p>
    pub fn ec2_instance_id(&self) -> ::std::option::Option<&str> {
        self.ec2_instance_id.as_deref()
    }
    /// <p>The time stamp when this information was retrieved.</p>
    pub fn sample_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.sample_timestamp.as_ref()
    }
    /// <p>The retrieved information. Currently contains a presigned Amazon S3 URL. The files are deleted after 15 minutes.</p>
    /// <p>Anyone in possession of this URL can access the files before they are deleted. Make the URL available only to trusted parties.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl EnvironmentInfoDescription {
    /// Creates a new builder-style object to manufacture [`EnvironmentInfoDescription`](crate::types::EnvironmentInfoDescription).
    pub fn builder() -> crate::types::builders::EnvironmentInfoDescriptionBuilder {
        crate::types::builders::EnvironmentInfoDescriptionBuilder::default()
    }
}

/// A builder for [`EnvironmentInfoDescription`](crate::types::EnvironmentInfoDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EnvironmentInfoDescriptionBuilder {
    pub(crate) info_type: ::std::option::Option<crate::types::EnvironmentInfoType>,
    pub(crate) ec2_instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) sample_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl EnvironmentInfoDescriptionBuilder {
    /// <p>The type of information retrieved.</p>
    pub fn info_type(mut self, input: crate::types::EnvironmentInfoType) -> Self {
        self.info_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of information retrieved.</p>
    pub fn set_info_type(
        mut self,
        input: ::std::option::Option<crate::types::EnvironmentInfoType>,
    ) -> Self {
        self.info_type = input;
        self
    }
    /// <p>The Amazon EC2 Instance ID for this information.</p>
    pub fn ec2_instance_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.ec2_instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon EC2 Instance ID for this information.</p>
    pub fn set_ec2_instance_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.ec2_instance_id = input;
        self
    }
    /// <p>The time stamp when this information was retrieved.</p>
    pub fn sample_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.sample_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time stamp when this information was retrieved.</p>
    pub fn set_sample_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.sample_timestamp = input;
        self
    }
    /// <p>The retrieved information. Currently contains a presigned Amazon S3 URL. The files are deleted after 15 minutes.</p>
    /// <p>Anyone in possession of this URL can access the files before they are deleted. Make the URL available only to trusted parties.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The retrieved information. Currently contains a presigned Amazon S3 URL. The files are deleted after 15 minutes.</p>
    /// <p>Anyone in possession of this URL can access the files before they are deleted. Make the URL available only to trusted parties.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Consumes the builder and constructs a [`EnvironmentInfoDescription`](crate::types::EnvironmentInfoDescription).
    pub fn build(self) -> crate::types::EnvironmentInfoDescription {
        crate::types::EnvironmentInfoDescription {
            info_type: self.info_type,
            ec2_instance_id: self.ec2_instance_id,
            sample_timestamp: self.sample_timestamp,
            message: self.message,
        }
    }
}
