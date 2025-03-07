// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeMailboxExportJobInput {
    /// <p>The mailbox export job ID.</p>
    #[doc(hidden)]
    pub job_id: ::std::option::Option<::std::string::String>,
    /// <p>The organization ID.</p>
    #[doc(hidden)]
    pub organization_id: ::std::option::Option<::std::string::String>,
}
impl DescribeMailboxExportJobInput {
    /// <p>The mailbox export job ID.</p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
    /// <p>The organization ID.</p>
    pub fn organization_id(&self) -> ::std::option::Option<&str> {
        self.organization_id.as_deref()
    }
}
impl DescribeMailboxExportJobInput {
    /// Creates a new builder-style object to manufacture [`DescribeMailboxExportJobInput`](crate::operation::describe_mailbox_export_job::DescribeMailboxExportJobInput).
    pub fn builder(
    ) -> crate::operation::describe_mailbox_export_job::builders::DescribeMailboxExportJobInputBuilder
    {
        crate::operation::describe_mailbox_export_job::builders::DescribeMailboxExportJobInputBuilder::default()
    }
}

/// A builder for [`DescribeMailboxExportJobInput`](crate::operation::describe_mailbox_export_job::DescribeMailboxExportJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeMailboxExportJobInputBuilder {
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
    pub(crate) organization_id: ::std::option::Option<::std::string::String>,
}
impl DescribeMailboxExportJobInputBuilder {
    /// <p>The mailbox export job ID.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The mailbox export job ID.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// <p>The organization ID.</p>
    pub fn organization_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.organization_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The organization ID.</p>
    pub fn set_organization_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.organization_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeMailboxExportJobInput`](crate::operation::describe_mailbox_export_job::DescribeMailboxExportJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_mailbox_export_job::DescribeMailboxExportJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_mailbox_export_job::DescribeMailboxExportJobInput {
                job_id: self.job_id,
                organization_id: self.organization_id,
            },
        )
    }
}
