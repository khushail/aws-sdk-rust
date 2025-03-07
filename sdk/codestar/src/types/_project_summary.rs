// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the metadata for a project.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProjectSummary {
    /// <p>The ID of the project.</p>
    #[doc(hidden)]
    pub project_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    #[doc(hidden)]
    pub project_arn: ::std::option::Option<::std::string::String>,
}
impl ProjectSummary {
    /// <p>The ID of the project.</p>
    pub fn project_id(&self) -> ::std::option::Option<&str> {
        self.project_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn project_arn(&self) -> ::std::option::Option<&str> {
        self.project_arn.as_deref()
    }
}
impl ProjectSummary {
    /// Creates a new builder-style object to manufacture [`ProjectSummary`](crate::types::ProjectSummary).
    pub fn builder() -> crate::types::builders::ProjectSummaryBuilder {
        crate::types::builders::ProjectSummaryBuilder::default()
    }
}

/// A builder for [`ProjectSummary`](crate::types::ProjectSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ProjectSummaryBuilder {
    pub(crate) project_id: ::std::option::Option<::std::string::String>,
    pub(crate) project_arn: ::std::option::Option<::std::string::String>,
}
impl ProjectSummaryBuilder {
    /// <p>The ID of the project.</p>
    pub fn project_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the project.</p>
    pub fn set_project_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn project_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn set_project_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`ProjectSummary`](crate::types::ProjectSummary).
    pub fn build(self) -> crate::types::ProjectSummary {
        crate::types::ProjectSummary {
            project_id: self.project_id,
            project_arn: self.project_arn,
        }
    }
}
