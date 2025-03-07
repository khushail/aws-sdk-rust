// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The origin access identity's configuration information. For more information, see <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_CloudFrontOriginAccessIdentityConfig.html">CloudFrontOriginAccessIdentityConfig</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetCloudFrontOriginAccessIdentityConfigInput {
    /// <p>The identity's ID.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl GetCloudFrontOriginAccessIdentityConfigInput {
    /// <p>The identity's ID.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl GetCloudFrontOriginAccessIdentityConfigInput {
    /// Creates a new builder-style object to manufacture [`GetCloudFrontOriginAccessIdentityConfigInput`](crate::operation::get_cloud_front_origin_access_identity_config::GetCloudFrontOriginAccessIdentityConfigInput).
    pub fn builder() -> crate::operation::get_cloud_front_origin_access_identity_config::builders::GetCloudFrontOriginAccessIdentityConfigInputBuilder{
        crate::operation::get_cloud_front_origin_access_identity_config::builders::GetCloudFrontOriginAccessIdentityConfigInputBuilder::default()
    }
}

/// A builder for [`GetCloudFrontOriginAccessIdentityConfigInput`](crate::operation::get_cloud_front_origin_access_identity_config::GetCloudFrontOriginAccessIdentityConfigInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetCloudFrontOriginAccessIdentityConfigInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl GetCloudFrontOriginAccessIdentityConfigInputBuilder {
    /// <p>The identity's ID.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identity's ID.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetCloudFrontOriginAccessIdentityConfigInput`](crate::operation::get_cloud_front_origin_access_identity_config::GetCloudFrontOriginAccessIdentityConfigInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_cloud_front_origin_access_identity_config::GetCloudFrontOriginAccessIdentityConfigInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::get_cloud_front_origin_access_identity_config::GetCloudFrontOriginAccessIdentityConfigInput {
                id: self.id
                ,
            }
        )
    }
}
