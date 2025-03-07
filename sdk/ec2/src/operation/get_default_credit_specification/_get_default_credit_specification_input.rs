// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDefaultCreditSpecificationInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The instance family.</p>
    #[doc(hidden)]
    pub instance_family: ::std::option::Option<crate::types::UnlimitedSupportedInstanceFamily>,
}
impl GetDefaultCreditSpecificationInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The instance family.</p>
    pub fn instance_family(
        &self,
    ) -> ::std::option::Option<&crate::types::UnlimitedSupportedInstanceFamily> {
        self.instance_family.as_ref()
    }
}
impl GetDefaultCreditSpecificationInput {
    /// Creates a new builder-style object to manufacture [`GetDefaultCreditSpecificationInput`](crate::operation::get_default_credit_specification::GetDefaultCreditSpecificationInput).
    pub fn builder() -> crate::operation::get_default_credit_specification::builders::GetDefaultCreditSpecificationInputBuilder{
        crate::operation::get_default_credit_specification::builders::GetDefaultCreditSpecificationInputBuilder::default()
    }
}

/// A builder for [`GetDefaultCreditSpecificationInput`](crate::operation::get_default_credit_specification::GetDefaultCreditSpecificationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetDefaultCreditSpecificationInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) instance_family:
        ::std::option::Option<crate::types::UnlimitedSupportedInstanceFamily>,
}
impl GetDefaultCreditSpecificationInputBuilder {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The instance family.</p>
    pub fn instance_family(
        mut self,
        input: crate::types::UnlimitedSupportedInstanceFamily,
    ) -> Self {
        self.instance_family = ::std::option::Option::Some(input);
        self
    }
    /// <p>The instance family.</p>
    pub fn set_instance_family(
        mut self,
        input: ::std::option::Option<crate::types::UnlimitedSupportedInstanceFamily>,
    ) -> Self {
        self.instance_family = input;
        self
    }
    /// Consumes the builder and constructs a [`GetDefaultCreditSpecificationInput`](crate::operation::get_default_credit_specification::GetDefaultCreditSpecificationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_default_credit_specification::GetDefaultCreditSpecificationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_default_credit_specification::GetDefaultCreditSpecificationInput {
                dry_run: self.dry_run
                ,
                instance_family: self.instance_family
                ,
            }
        )
    }
}
