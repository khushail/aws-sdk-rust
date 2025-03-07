// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeJobInput {
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[doc(hidden)]
    pub job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeJobInput {
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
}
impl DescribeJobInput {
    /// Creates a new builder-style object to manufacture [`DescribeJobInput`](crate::operation::describe_job::DescribeJobInput).
    pub fn builder() -> crate::operation::describe_job::builders::DescribeJobInputBuilder {
        crate::operation::describe_job::builders::DescribeJobInputBuilder::default()
    }
}

/// A builder for [`DescribeJobInput`](crate::operation::describe_job::DescribeJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeJobInputBuilder {
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeJobInputBuilder {
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeJobInput`](crate::operation::describe_job::DescribeJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_job::DescribeJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_job::DescribeJobInput {
            job_id: self.job_id,
        })
    }
}
