// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutServiceQuotaIncreaseRequestIntoTemplateInput {
    /// <p>The quota identifier.</p>
    #[doc(hidden)]
    pub quota_code: ::std::option::Option<::std::string::String>,
    /// <p>The service identifier.</p>
    #[doc(hidden)]
    pub service_code: ::std::option::Option<::std::string::String>,
    /// <p>The AWS Region.</p>
    #[doc(hidden)]
    pub aws_region: ::std::option::Option<::std::string::String>,
    /// <p>The new, increased value for the quota.</p>
    #[doc(hidden)]
    pub desired_value: ::std::option::Option<f64>,
}
impl PutServiceQuotaIncreaseRequestIntoTemplateInput {
    /// <p>The quota identifier.</p>
    pub fn quota_code(&self) -> ::std::option::Option<&str> {
        self.quota_code.as_deref()
    }
    /// <p>The service identifier.</p>
    pub fn service_code(&self) -> ::std::option::Option<&str> {
        self.service_code.as_deref()
    }
    /// <p>The AWS Region.</p>
    pub fn aws_region(&self) -> ::std::option::Option<&str> {
        self.aws_region.as_deref()
    }
    /// <p>The new, increased value for the quota.</p>
    pub fn desired_value(&self) -> ::std::option::Option<f64> {
        self.desired_value
    }
}
impl PutServiceQuotaIncreaseRequestIntoTemplateInput {
    /// Creates a new builder-style object to manufacture [`PutServiceQuotaIncreaseRequestIntoTemplateInput`](crate::operation::put_service_quota_increase_request_into_template::PutServiceQuotaIncreaseRequestIntoTemplateInput).
    pub fn builder() -> crate::operation::put_service_quota_increase_request_into_template::builders::PutServiceQuotaIncreaseRequestIntoTemplateInputBuilder{
        crate::operation::put_service_quota_increase_request_into_template::builders::PutServiceQuotaIncreaseRequestIntoTemplateInputBuilder::default()
    }
}

/// A builder for [`PutServiceQuotaIncreaseRequestIntoTemplateInput`](crate::operation::put_service_quota_increase_request_into_template::PutServiceQuotaIncreaseRequestIntoTemplateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutServiceQuotaIncreaseRequestIntoTemplateInputBuilder {
    pub(crate) quota_code: ::std::option::Option<::std::string::String>,
    pub(crate) service_code: ::std::option::Option<::std::string::String>,
    pub(crate) aws_region: ::std::option::Option<::std::string::String>,
    pub(crate) desired_value: ::std::option::Option<f64>,
}
impl PutServiceQuotaIncreaseRequestIntoTemplateInputBuilder {
    /// <p>The quota identifier.</p>
    pub fn quota_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.quota_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The quota identifier.</p>
    pub fn set_quota_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.quota_code = input;
        self
    }
    /// <p>The service identifier.</p>
    pub fn service_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The service identifier.</p>
    pub fn set_service_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_code = input;
        self
    }
    /// <p>The AWS Region.</p>
    pub fn aws_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.aws_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The AWS Region.</p>
    pub fn set_aws_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.aws_region = input;
        self
    }
    /// <p>The new, increased value for the quota.</p>
    pub fn desired_value(mut self, input: f64) -> Self {
        self.desired_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The new, increased value for the quota.</p>
    pub fn set_desired_value(mut self, input: ::std::option::Option<f64>) -> Self {
        self.desired_value = input;
        self
    }
    /// Consumes the builder and constructs a [`PutServiceQuotaIncreaseRequestIntoTemplateInput`](crate::operation::put_service_quota_increase_request_into_template::PutServiceQuotaIncreaseRequestIntoTemplateInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::put_service_quota_increase_request_into_template::PutServiceQuotaIncreaseRequestIntoTemplateInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::put_service_quota_increase_request_into_template::PutServiceQuotaIncreaseRequestIntoTemplateInput {
                quota_code: self.quota_code
                ,
                service_code: self.service_code
                ,
                aws_region: self.aws_region
                ,
                desired_value: self.desired_value
                ,
            }
        )
    }
}
