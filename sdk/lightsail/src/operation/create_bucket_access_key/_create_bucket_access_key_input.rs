// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateBucketAccessKeyInput {
    /// <p>The name of the bucket that the new access key will belong to, and grant access to.</p>
    #[doc(hidden)]
    pub bucket_name: ::std::option::Option<::std::string::String>,
}
impl CreateBucketAccessKeyInput {
    /// <p>The name of the bucket that the new access key will belong to, and grant access to.</p>
    pub fn bucket_name(&self) -> ::std::option::Option<&str> {
        self.bucket_name.as_deref()
    }
}
impl CreateBucketAccessKeyInput {
    /// Creates a new builder-style object to manufacture [`CreateBucketAccessKeyInput`](crate::operation::create_bucket_access_key::CreateBucketAccessKeyInput).
    pub fn builder(
    ) -> crate::operation::create_bucket_access_key::builders::CreateBucketAccessKeyInputBuilder
    {
        crate::operation::create_bucket_access_key::builders::CreateBucketAccessKeyInputBuilder::default()
    }
}

/// A builder for [`CreateBucketAccessKeyInput`](crate::operation::create_bucket_access_key::CreateBucketAccessKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateBucketAccessKeyInputBuilder {
    pub(crate) bucket_name: ::std::option::Option<::std::string::String>,
}
impl CreateBucketAccessKeyInputBuilder {
    /// <p>The name of the bucket that the new access key will belong to, and grant access to.</p>
    pub fn bucket_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the bucket that the new access key will belong to, and grant access to.</p>
    pub fn set_bucket_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket_name = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateBucketAccessKeyInput`](crate::operation::create_bucket_access_key::CreateBucketAccessKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_bucket_access_key::CreateBucketAccessKeyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_bucket_access_key::CreateBucketAccessKeyInput {
                bucket_name: self.bucket_name,
            },
        )
    }
}
