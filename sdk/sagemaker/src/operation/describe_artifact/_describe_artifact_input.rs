// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeArtifactInput {
    /// <p>The Amazon Resource Name (ARN) of the artifact to describe.</p>
    #[doc(hidden)]
    pub artifact_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeArtifactInput {
    /// <p>The Amazon Resource Name (ARN) of the artifact to describe.</p>
    pub fn artifact_arn(&self) -> ::std::option::Option<&str> {
        self.artifact_arn.as_deref()
    }
}
impl DescribeArtifactInput {
    /// Creates a new builder-style object to manufacture [`DescribeArtifactInput`](crate::operation::describe_artifact::DescribeArtifactInput).
    pub fn builder() -> crate::operation::describe_artifact::builders::DescribeArtifactInputBuilder
    {
        crate::operation::describe_artifact::builders::DescribeArtifactInputBuilder::default()
    }
}

/// A builder for [`DescribeArtifactInput`](crate::operation::describe_artifact::DescribeArtifactInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeArtifactInputBuilder {
    pub(crate) artifact_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeArtifactInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the artifact to describe.</p>
    pub fn artifact_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.artifact_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the artifact to describe.</p>
    pub fn set_artifact_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.artifact_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeArtifactInput`](crate::operation::describe_artifact::DescribeArtifactInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_artifact::DescribeArtifactInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_artifact::DescribeArtifactInput {
            artifact_arn: self.artifact_arn,
        })
    }
}
