// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeDomainOutput {
    /// <p>The name of the domain.</p>
    #[doc(hidden)]
    pub domain_name: ::std::option::Option<::std::string::String>,
    /// <p>The name to display.</p>
    #[doc(hidden)]
    pub display_name: ::std::option::Option<::std::string::String>,
    /// <p>The time that the domain was added.</p>
    #[doc(hidden)]
    pub created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The current state for the domain.</p>
    #[doc(hidden)]
    pub domain_status: ::std::option::Option<crate::types::DomainStatus>,
    /// <p>The ARN of an issued ACM certificate that is valid for the domain being associated.</p>
    #[doc(hidden)]
    pub acm_certificate_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeDomainOutput {
    /// <p>The name of the domain.</p>
    pub fn domain_name(&self) -> ::std::option::Option<&str> {
        self.domain_name.as_deref()
    }
    /// <p>The name to display.</p>
    pub fn display_name(&self) -> ::std::option::Option<&str> {
        self.display_name.as_deref()
    }
    /// <p>The time that the domain was added.</p>
    pub fn created_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_time.as_ref()
    }
    /// <p>The current state for the domain.</p>
    pub fn domain_status(&self) -> ::std::option::Option<&crate::types::DomainStatus> {
        self.domain_status.as_ref()
    }
    /// <p>The ARN of an issued ACM certificate that is valid for the domain being associated.</p>
    pub fn acm_certificate_arn(&self) -> ::std::option::Option<&str> {
        self.acm_certificate_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeDomainOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeDomainOutput {
    /// Creates a new builder-style object to manufacture [`DescribeDomainOutput`](crate::operation::describe_domain::DescribeDomainOutput).
    pub fn builder() -> crate::operation::describe_domain::builders::DescribeDomainOutputBuilder {
        crate::operation::describe_domain::builders::DescribeDomainOutputBuilder::default()
    }
}

/// A builder for [`DescribeDomainOutput`](crate::operation::describe_domain::DescribeDomainOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeDomainOutputBuilder {
    pub(crate) domain_name: ::std::option::Option<::std::string::String>,
    pub(crate) display_name: ::std::option::Option<::std::string::String>,
    pub(crate) created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) domain_status: ::std::option::Option<crate::types::DomainStatus>,
    pub(crate) acm_certificate_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeDomainOutputBuilder {
    /// <p>The name of the domain.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the domain.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_name = input;
        self
    }
    /// <p>The name to display.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name to display.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_name = input;
        self
    }
    /// <p>The time that the domain was added.</p>
    pub fn created_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time that the domain was added.</p>
    pub fn set_created_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_time = input;
        self
    }
    /// <p>The current state for the domain.</p>
    pub fn domain_status(mut self, input: crate::types::DomainStatus) -> Self {
        self.domain_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current state for the domain.</p>
    pub fn set_domain_status(
        mut self,
        input: ::std::option::Option<crate::types::DomainStatus>,
    ) -> Self {
        self.domain_status = input;
        self
    }
    /// <p>The ARN of an issued ACM certificate that is valid for the domain being associated.</p>
    pub fn acm_certificate_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.acm_certificate_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of an issued ACM certificate that is valid for the domain being associated.</p>
    pub fn set_acm_certificate_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.acm_certificate_arn = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeDomainOutput`](crate::operation::describe_domain::DescribeDomainOutput).
    pub fn build(self) -> crate::operation::describe_domain::DescribeDomainOutput {
        crate::operation::describe_domain::DescribeDomainOutput {
            domain_name: self.domain_name,
            display_name: self.display_name,
            created_time: self.created_time,
            domain_status: self.domain_status,
            acm_certificate_arn: self.acm_certificate_arn,
            _request_id: self._request_id,
        }
    }
}
