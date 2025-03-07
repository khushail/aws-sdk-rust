// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An access point used to access a bucket.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AccessPoint {
    /// <p>The name of this access point.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether this access point allows access from the public internet. If <code>VpcConfiguration</code> is specified for this access point, then <code>NetworkOrigin</code> is <code>VPC</code>, and the access point doesn't allow access from the public internet. Otherwise, <code>NetworkOrigin</code> is <code>Internet</code>, and the access point allows access from the public internet, subject to the access point and bucket access policies.</p>
    #[doc(hidden)]
    pub network_origin: ::std::option::Option<crate::types::NetworkOrigin>,
    /// <p>The virtual private cloud (VPC) configuration for this access point, if one exists.</p> <note>
    /// <p>This element is empty if this access point is an Amazon S3 on Outposts access point that is used by other Amazon Web Services.</p>
    /// </note>
    #[doc(hidden)]
    pub vpc_configuration: ::std::option::Option<crate::types::VpcConfiguration>,
    /// <p>The name of the bucket associated with this access point.</p>
    #[doc(hidden)]
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The ARN for the access point.</p>
    #[doc(hidden)]
    pub access_point_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name or alias of the access point.</p>
    #[doc(hidden)]
    pub alias: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services account ID associated with the S3 bucket associated with this access point.</p>
    #[doc(hidden)]
    pub bucket_account_id: ::std::option::Option<::std::string::String>,
}
impl AccessPoint {
    /// <p>The name of this access point.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Indicates whether this access point allows access from the public internet. If <code>VpcConfiguration</code> is specified for this access point, then <code>NetworkOrigin</code> is <code>VPC</code>, and the access point doesn't allow access from the public internet. Otherwise, <code>NetworkOrigin</code> is <code>Internet</code>, and the access point allows access from the public internet, subject to the access point and bucket access policies.</p>
    pub fn network_origin(&self) -> ::std::option::Option<&crate::types::NetworkOrigin> {
        self.network_origin.as_ref()
    }
    /// <p>The virtual private cloud (VPC) configuration for this access point, if one exists.</p> <note>
    /// <p>This element is empty if this access point is an Amazon S3 on Outposts access point that is used by other Amazon Web Services.</p>
    /// </note>
    pub fn vpc_configuration(&self) -> ::std::option::Option<&crate::types::VpcConfiguration> {
        self.vpc_configuration.as_ref()
    }
    /// <p>The name of the bucket associated with this access point.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The ARN for the access point.</p>
    pub fn access_point_arn(&self) -> ::std::option::Option<&str> {
        self.access_point_arn.as_deref()
    }
    /// <p>The name or alias of the access point.</p>
    pub fn alias(&self) -> ::std::option::Option<&str> {
        self.alias.as_deref()
    }
    /// <p>The Amazon Web Services account ID associated with the S3 bucket associated with this access point.</p>
    pub fn bucket_account_id(&self) -> ::std::option::Option<&str> {
        self.bucket_account_id.as_deref()
    }
}
impl AccessPoint {
    /// Creates a new builder-style object to manufacture [`AccessPoint`](crate::types::AccessPoint).
    pub fn builder() -> crate::types::builders::AccessPointBuilder {
        crate::types::builders::AccessPointBuilder::default()
    }
}

/// A builder for [`AccessPoint`](crate::types::AccessPoint).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AccessPointBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) network_origin: ::std::option::Option<crate::types::NetworkOrigin>,
    pub(crate) vpc_configuration: ::std::option::Option<crate::types::VpcConfiguration>,
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) access_point_arn: ::std::option::Option<::std::string::String>,
    pub(crate) alias: ::std::option::Option<::std::string::String>,
    pub(crate) bucket_account_id: ::std::option::Option<::std::string::String>,
}
impl AccessPointBuilder {
    /// <p>The name of this access point.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of this access point.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Indicates whether this access point allows access from the public internet. If <code>VpcConfiguration</code> is specified for this access point, then <code>NetworkOrigin</code> is <code>VPC</code>, and the access point doesn't allow access from the public internet. Otherwise, <code>NetworkOrigin</code> is <code>Internet</code>, and the access point allows access from the public internet, subject to the access point and bucket access policies.</p>
    pub fn network_origin(mut self, input: crate::types::NetworkOrigin) -> Self {
        self.network_origin = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether this access point allows access from the public internet. If <code>VpcConfiguration</code> is specified for this access point, then <code>NetworkOrigin</code> is <code>VPC</code>, and the access point doesn't allow access from the public internet. Otherwise, <code>NetworkOrigin</code> is <code>Internet</code>, and the access point allows access from the public internet, subject to the access point and bucket access policies.</p>
    pub fn set_network_origin(
        mut self,
        input: ::std::option::Option<crate::types::NetworkOrigin>,
    ) -> Self {
        self.network_origin = input;
        self
    }
    /// <p>The virtual private cloud (VPC) configuration for this access point, if one exists.</p> <note>
    /// <p>This element is empty if this access point is an Amazon S3 on Outposts access point that is used by other Amazon Web Services.</p>
    /// </note>
    pub fn vpc_configuration(mut self, input: crate::types::VpcConfiguration) -> Self {
        self.vpc_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The virtual private cloud (VPC) configuration for this access point, if one exists.</p> <note>
    /// <p>This element is empty if this access point is an Amazon S3 on Outposts access point that is used by other Amazon Web Services.</p>
    /// </note>
    pub fn set_vpc_configuration(
        mut self,
        input: ::std::option::Option<crate::types::VpcConfiguration>,
    ) -> Self {
        self.vpc_configuration = input;
        self
    }
    /// <p>The name of the bucket associated with this access point.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the bucket associated with this access point.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The ARN for the access point.</p>
    pub fn access_point_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.access_point_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN for the access point.</p>
    pub fn set_access_point_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.access_point_arn = input;
        self
    }
    /// <p>The name or alias of the access point.</p>
    pub fn alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.alias = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name or alias of the access point.</p>
    pub fn set_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.alias = input;
        self
    }
    /// <p>The Amazon Web Services account ID associated with the S3 bucket associated with this access point.</p>
    pub fn bucket_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.bucket_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID associated with the S3 bucket associated with this access point.</p>
    pub fn set_bucket_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.bucket_account_id = input;
        self
    }
    /// Consumes the builder and constructs a [`AccessPoint`](crate::types::AccessPoint).
    pub fn build(self) -> crate::types::AccessPoint {
        crate::types::AccessPoint {
            name: self.name,
            network_origin: self.network_origin,
            vpc_configuration: self.vpc_configuration,
            bucket: self.bucket,
            access_point_arn: self.access_point_arn,
            alias: self.alias,
            bucket_account_id: self.bucket_account_id,
        }
    }
}
