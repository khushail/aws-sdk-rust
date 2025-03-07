// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Metadata about the pull request that is used when comparing the pull request source with its destination.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PullRequestCreatedEventMetadata {
    /// <p>The name of the repository where the pull request was created.</p>
    #[doc(hidden)]
    pub repository_name: ::std::option::Option<::std::string::String>,
    /// <p>The commit ID on the source branch used when the pull request was created.</p>
    #[doc(hidden)]
    pub source_commit_id: ::std::option::Option<::std::string::String>,
    /// <p>The commit ID of the tip of the branch specified as the destination branch when the pull request was created.</p>
    #[doc(hidden)]
    pub destination_commit_id: ::std::option::Option<::std::string::String>,
    /// <p>The commit ID of the most recent commit that the source branch and the destination branch have in common.</p>
    #[doc(hidden)]
    pub merge_base: ::std::option::Option<::std::string::String>,
}
impl PullRequestCreatedEventMetadata {
    /// <p>The name of the repository where the pull request was created.</p>
    pub fn repository_name(&self) -> ::std::option::Option<&str> {
        self.repository_name.as_deref()
    }
    /// <p>The commit ID on the source branch used when the pull request was created.</p>
    pub fn source_commit_id(&self) -> ::std::option::Option<&str> {
        self.source_commit_id.as_deref()
    }
    /// <p>The commit ID of the tip of the branch specified as the destination branch when the pull request was created.</p>
    pub fn destination_commit_id(&self) -> ::std::option::Option<&str> {
        self.destination_commit_id.as_deref()
    }
    /// <p>The commit ID of the most recent commit that the source branch and the destination branch have in common.</p>
    pub fn merge_base(&self) -> ::std::option::Option<&str> {
        self.merge_base.as_deref()
    }
}
impl PullRequestCreatedEventMetadata {
    /// Creates a new builder-style object to manufacture [`PullRequestCreatedEventMetadata`](crate::types::PullRequestCreatedEventMetadata).
    pub fn builder() -> crate::types::builders::PullRequestCreatedEventMetadataBuilder {
        crate::types::builders::PullRequestCreatedEventMetadataBuilder::default()
    }
}

/// A builder for [`PullRequestCreatedEventMetadata`](crate::types::PullRequestCreatedEventMetadata).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PullRequestCreatedEventMetadataBuilder {
    pub(crate) repository_name: ::std::option::Option<::std::string::String>,
    pub(crate) source_commit_id: ::std::option::Option<::std::string::String>,
    pub(crate) destination_commit_id: ::std::option::Option<::std::string::String>,
    pub(crate) merge_base: ::std::option::Option<::std::string::String>,
}
impl PullRequestCreatedEventMetadataBuilder {
    /// <p>The name of the repository where the pull request was created.</p>
    pub fn repository_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.repository_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the repository where the pull request was created.</p>
    pub fn set_repository_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.repository_name = input;
        self
    }
    /// <p>The commit ID on the source branch used when the pull request was created.</p>
    pub fn source_commit_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_commit_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The commit ID on the source branch used when the pull request was created.</p>
    pub fn set_source_commit_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_commit_id = input;
        self
    }
    /// <p>The commit ID of the tip of the branch specified as the destination branch when the pull request was created.</p>
    pub fn destination_commit_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.destination_commit_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The commit ID of the tip of the branch specified as the destination branch when the pull request was created.</p>
    pub fn set_destination_commit_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.destination_commit_id = input;
        self
    }
    /// <p>The commit ID of the most recent commit that the source branch and the destination branch have in common.</p>
    pub fn merge_base(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.merge_base = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The commit ID of the most recent commit that the source branch and the destination branch have in common.</p>
    pub fn set_merge_base(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.merge_base = input;
        self
    }
    /// Consumes the builder and constructs a [`PullRequestCreatedEventMetadata`](crate::types::PullRequestCreatedEventMetadata).
    pub fn build(self) -> crate::types::PullRequestCreatedEventMetadata {
        crate::types::PullRequestCreatedEventMetadata {
            repository_name: self.repository_name,
            source_commit_id: self.source_commit_id,
            destination_commit_id: self.destination_commit_id,
            merge_base: self.merge_base,
        }
    }
}
