// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateResourceProfileDetectionsInput {
    /// <p>The Amazon Resource Name (ARN) of the S3 bucket that the request applies to.</p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>An array of objects, one for each custom data identifier or managed data identifier that detected the type of sensitive data to start excluding or including in the bucket's score. To start including all sensitive data types in the score, don't specify any values for this array.</p>
    #[doc(hidden)]
    pub suppress_data_identifiers:
        ::std::option::Option<::std::vec::Vec<crate::types::SuppressDataIdentifier>>,
}
impl UpdateResourceProfileDetectionsInput {
    /// <p>The Amazon Resource Name (ARN) of the S3 bucket that the request applies to.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>An array of objects, one for each custom data identifier or managed data identifier that detected the type of sensitive data to start excluding or including in the bucket's score. To start including all sensitive data types in the score, don't specify any values for this array.</p>
    pub fn suppress_data_identifiers(
        &self,
    ) -> ::std::option::Option<&[crate::types::SuppressDataIdentifier]> {
        self.suppress_data_identifiers.as_deref()
    }
}
impl UpdateResourceProfileDetectionsInput {
    /// Creates a new builder-style object to manufacture [`UpdateResourceProfileDetectionsInput`](crate::operation::update_resource_profile_detections::UpdateResourceProfileDetectionsInput).
    pub fn builder() -> crate::operation::update_resource_profile_detections::builders::UpdateResourceProfileDetectionsInputBuilder{
        crate::operation::update_resource_profile_detections::builders::UpdateResourceProfileDetectionsInputBuilder::default()
    }
}

/// A builder for [`UpdateResourceProfileDetectionsInput`](crate::operation::update_resource_profile_detections::UpdateResourceProfileDetectionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateResourceProfileDetectionsInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) suppress_data_identifiers:
        ::std::option::Option<::std::vec::Vec<crate::types::SuppressDataIdentifier>>,
}
impl UpdateResourceProfileDetectionsInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the S3 bucket that the request applies to.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the S3 bucket that the request applies to.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// Appends an item to `suppress_data_identifiers`.
    ///
    /// To override the contents of this collection use [`set_suppress_data_identifiers`](Self::set_suppress_data_identifiers).
    ///
    /// <p>An array of objects, one for each custom data identifier or managed data identifier that detected the type of sensitive data to start excluding or including in the bucket's score. To start including all sensitive data types in the score, don't specify any values for this array.</p>
    pub fn suppress_data_identifiers(
        mut self,
        input: crate::types::SuppressDataIdentifier,
    ) -> Self {
        let mut v = self.suppress_data_identifiers.unwrap_or_default();
        v.push(input);
        self.suppress_data_identifiers = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of objects, one for each custom data identifier or managed data identifier that detected the type of sensitive data to start excluding or including in the bucket's score. To start including all sensitive data types in the score, don't specify any values for this array.</p>
    pub fn set_suppress_data_identifiers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SuppressDataIdentifier>>,
    ) -> Self {
        self.suppress_data_identifiers = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateResourceProfileDetectionsInput`](crate::operation::update_resource_profile_detections::UpdateResourceProfileDetectionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_resource_profile_detections::UpdateResourceProfileDetectionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_resource_profile_detections::UpdateResourceProfileDetectionsInput {
                resource_arn: self.resource_arn
                ,
                suppress_data_identifiers: self.suppress_data_identifiers
                ,
            }
        )
    }
}
