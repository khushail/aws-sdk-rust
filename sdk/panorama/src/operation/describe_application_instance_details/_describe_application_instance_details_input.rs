// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeApplicationInstanceDetailsInput {
    /// <p>The application instance's ID.</p>
    #[doc(hidden)]
    pub application_instance_id: ::std::option::Option<::std::string::String>,
}
impl DescribeApplicationInstanceDetailsInput {
    /// <p>The application instance's ID.</p>
    pub fn application_instance_id(&self) -> ::std::option::Option<&str> {
        self.application_instance_id.as_deref()
    }
}
impl DescribeApplicationInstanceDetailsInput {
    /// Creates a new builder-style object to manufacture [`DescribeApplicationInstanceDetailsInput`](crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsInput).
    pub fn builder() -> crate::operation::describe_application_instance_details::builders::DescribeApplicationInstanceDetailsInputBuilder{
        crate::operation::describe_application_instance_details::builders::DescribeApplicationInstanceDetailsInputBuilder::default()
    }
}

/// A builder for [`DescribeApplicationInstanceDetailsInput`](crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeApplicationInstanceDetailsInputBuilder {
    pub(crate) application_instance_id: ::std::option::Option<::std::string::String>,
}
impl DescribeApplicationInstanceDetailsInputBuilder {
    /// <p>The application instance's ID.</p>
    pub fn application_instance_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The application instance's ID.</p>
    pub fn set_application_instance_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_instance_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeApplicationInstanceDetailsInput`](crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsInput {
                application_instance_id: self.application_instance_id
                ,
            }
        )
    }
}
