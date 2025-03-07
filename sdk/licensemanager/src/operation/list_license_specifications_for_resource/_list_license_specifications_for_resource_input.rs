// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListLicenseSpecificationsForResourceInput {
    /// <p>Amazon Resource Name (ARN) of a resource that has an associated license configuration.</p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>Maximum number of results to return in a single call.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>Token for the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListLicenseSpecificationsForResourceInput {
    /// <p>Amazon Resource Name (ARN) of a resource that has an associated license configuration.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>Maximum number of results to return in a single call.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>Token for the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListLicenseSpecificationsForResourceInput {
    /// Creates a new builder-style object to manufacture [`ListLicenseSpecificationsForResourceInput`](crate::operation::list_license_specifications_for_resource::ListLicenseSpecificationsForResourceInput).
    pub fn builder() -> crate::operation::list_license_specifications_for_resource::builders::ListLicenseSpecificationsForResourceInputBuilder{
        crate::operation::list_license_specifications_for_resource::builders::ListLicenseSpecificationsForResourceInputBuilder::default()
    }
}

/// A builder for [`ListLicenseSpecificationsForResourceInput`](crate::operation::list_license_specifications_for_resource::ListLicenseSpecificationsForResourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListLicenseSpecificationsForResourceInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListLicenseSpecificationsForResourceInputBuilder {
    /// <p>Amazon Resource Name (ARN) of a resource that has an associated license configuration.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of a resource that has an associated license configuration.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>Maximum number of results to return in a single call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>Maximum number of results to return in a single call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>Token for the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Token for the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListLicenseSpecificationsForResourceInput`](crate::operation::list_license_specifications_for_resource::ListLicenseSpecificationsForResourceInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::list_license_specifications_for_resource::ListLicenseSpecificationsForResourceInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::list_license_specifications_for_resource::ListLicenseSpecificationsForResourceInput {
                resource_arn: self.resource_arn
                ,
                max_results: self.max_results
                ,
                next_token: self.next_token
                ,
            }
        )
    }
}
