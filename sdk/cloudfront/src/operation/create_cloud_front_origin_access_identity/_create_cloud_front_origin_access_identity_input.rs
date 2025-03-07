// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request to create a new origin access identity (OAI). An origin access identity is a special CloudFront user that you can associate with Amazon S3 origins, so that you can secure all or just some of your Amazon S3 content. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html"> Restricting Access to Amazon S3 Content by Using an Origin Access Identity</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateCloudFrontOriginAccessIdentityInput {
    /// <p>The current configuration information for the identity.</p>
    #[doc(hidden)]
    pub cloud_front_origin_access_identity_config:
        ::std::option::Option<crate::types::CloudFrontOriginAccessIdentityConfig>,
}
impl CreateCloudFrontOriginAccessIdentityInput {
    /// <p>The current configuration information for the identity.</p>
    pub fn cloud_front_origin_access_identity_config(
        &self,
    ) -> ::std::option::Option<&crate::types::CloudFrontOriginAccessIdentityConfig> {
        self.cloud_front_origin_access_identity_config.as_ref()
    }
}
impl CreateCloudFrontOriginAccessIdentityInput {
    /// Creates a new builder-style object to manufacture [`CreateCloudFrontOriginAccessIdentityInput`](crate::operation::create_cloud_front_origin_access_identity::CreateCloudFrontOriginAccessIdentityInput).
    pub fn builder() -> crate::operation::create_cloud_front_origin_access_identity::builders::CreateCloudFrontOriginAccessIdentityInputBuilder{
        crate::operation::create_cloud_front_origin_access_identity::builders::CreateCloudFrontOriginAccessIdentityInputBuilder::default()
    }
}

/// A builder for [`CreateCloudFrontOriginAccessIdentityInput`](crate::operation::create_cloud_front_origin_access_identity::CreateCloudFrontOriginAccessIdentityInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateCloudFrontOriginAccessIdentityInputBuilder {
    pub(crate) cloud_front_origin_access_identity_config:
        ::std::option::Option<crate::types::CloudFrontOriginAccessIdentityConfig>,
}
impl CreateCloudFrontOriginAccessIdentityInputBuilder {
    /// <p>The current configuration information for the identity.</p>
    pub fn cloud_front_origin_access_identity_config(
        mut self,
        input: crate::types::CloudFrontOriginAccessIdentityConfig,
    ) -> Self {
        self.cloud_front_origin_access_identity_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current configuration information for the identity.</p>
    pub fn set_cloud_front_origin_access_identity_config(
        mut self,
        input: ::std::option::Option<crate::types::CloudFrontOriginAccessIdentityConfig>,
    ) -> Self {
        self.cloud_front_origin_access_identity_config = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateCloudFrontOriginAccessIdentityInput`](crate::operation::create_cloud_front_origin_access_identity::CreateCloudFrontOriginAccessIdentityInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::create_cloud_front_origin_access_identity::CreateCloudFrontOriginAccessIdentityInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::create_cloud_front_origin_access_identity::CreateCloudFrontOriginAccessIdentityInput {
                cloud_front_origin_access_identity_config: self.cloud_front_origin_access_identity_config
                ,
            }
        )
    }
}
