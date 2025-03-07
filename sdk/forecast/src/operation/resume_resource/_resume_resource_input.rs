// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResumeResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the monitor resource to resume.</p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
}
impl ResumeResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the monitor resource to resume.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
}
impl ResumeResourceInput {
    /// Creates a new builder-style object to manufacture [`ResumeResourceInput`](crate::operation::resume_resource::ResumeResourceInput).
    pub fn builder() -> crate::operation::resume_resource::builders::ResumeResourceInputBuilder {
        crate::operation::resume_resource::builders::ResumeResourceInputBuilder::default()
    }
}

/// A builder for [`ResumeResourceInput`](crate::operation::resume_resource::ResumeResourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResumeResourceInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
}
impl ResumeResourceInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the monitor resource to resume.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the monitor resource to resume.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`ResumeResourceInput`](crate::operation::resume_resource::ResumeResourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::resume_resource::ResumeResourceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::resume_resource::ResumeResourceInput {
            resource_arn: self.resource_arn,
        })
    }
}
