// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about an Amazon Web Services CodeCommit repository. The CodeCommit repository must be in the same Amazon Web Services Region and Amazon Web Services account where its CodeGuru Reviewer code reviews are configured.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CodeCommitRepository {
    /// <p>The name of the Amazon Web Services CodeCommit repository. For more information, see <a href="https://docs.aws.amazon.com/codecommit/latest/APIReference/API_GetRepository.html#CodeCommit-GetRepository-request-repositoryName">repositoryName</a> in the <i>Amazon Web Services CodeCommit API Reference</i>.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl CodeCommitRepository {
    /// <p>The name of the Amazon Web Services CodeCommit repository. For more information, see <a href="https://docs.aws.amazon.com/codecommit/latest/APIReference/API_GetRepository.html#CodeCommit-GetRepository-request-repositoryName">repositoryName</a> in the <i>Amazon Web Services CodeCommit API Reference</i>.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl CodeCommitRepository {
    /// Creates a new builder-style object to manufacture [`CodeCommitRepository`](crate::types::CodeCommitRepository).
    pub fn builder() -> crate::types::builders::CodeCommitRepositoryBuilder {
        crate::types::builders::CodeCommitRepositoryBuilder::default()
    }
}

/// A builder for [`CodeCommitRepository`](crate::types::CodeCommitRepository).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CodeCommitRepositoryBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl CodeCommitRepositoryBuilder {
    /// <p>The name of the Amazon Web Services CodeCommit repository. For more information, see <a href="https://docs.aws.amazon.com/codecommit/latest/APIReference/API_GetRepository.html#CodeCommit-GetRepository-request-repositoryName">repositoryName</a> in the <i>Amazon Web Services CodeCommit API Reference</i>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon Web Services CodeCommit repository. For more information, see <a href="https://docs.aws.amazon.com/codecommit/latest/APIReference/API_GetRepository.html#CodeCommit-GetRepository-request-repositoryName">repositoryName</a> in the <i>Amazon Web Services CodeCommit API Reference</i>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`CodeCommitRepository`](crate::types::CodeCommitRepository).
    pub fn build(self) -> crate::types::CodeCommitRepository {
        crate::types::CodeCommitRepository { name: self.name }
    }
}
