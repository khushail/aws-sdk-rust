// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSettingsInput {
    /// <p>The identifier of the directory for which to retrieve information.</p>
    #[doc(hidden)]
    pub directory_id: ::std::option::Option<::std::string::String>,
    /// <p>The status of the directory settings for which to retrieve information.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::DirectoryConfigurationStatus>,
    /// <p>The <code>DescribeSettingsResult.NextToken</code> value from a previous call to <code>DescribeSettings</code>. Pass null if this is the first call.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeSettingsInput {
    /// <p>The identifier of the directory for which to retrieve information.</p>
    pub fn directory_id(&self) -> ::std::option::Option<&str> {
        self.directory_id.as_deref()
    }
    /// <p>The status of the directory settings for which to retrieve information.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::DirectoryConfigurationStatus> {
        self.status.as_ref()
    }
    /// <p>The <code>DescribeSettingsResult.NextToken</code> value from a previous call to <code>DescribeSettings</code>. Pass null if this is the first call.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeSettingsInput {
    /// Creates a new builder-style object to manufacture [`DescribeSettingsInput`](crate::operation::describe_settings::DescribeSettingsInput).
    pub fn builder() -> crate::operation::describe_settings::builders::DescribeSettingsInputBuilder
    {
        crate::operation::describe_settings::builders::DescribeSettingsInputBuilder::default()
    }
}

/// A builder for [`DescribeSettingsInput`](crate::operation::describe_settings::DescribeSettingsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeSettingsInputBuilder {
    pub(crate) directory_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::DirectoryConfigurationStatus>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeSettingsInputBuilder {
    /// <p>The identifier of the directory for which to retrieve information.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.directory_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the directory for which to retrieve information.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.directory_id = input;
        self
    }
    /// <p>The status of the directory settings for which to retrieve information.</p>
    pub fn status(mut self, input: crate::types::DirectoryConfigurationStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the directory settings for which to retrieve information.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::DirectoryConfigurationStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The <code>DescribeSettingsResult.NextToken</code> value from a previous call to <code>DescribeSettings</code>. Pass null if this is the first call.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>DescribeSettingsResult.NextToken</code> value from a previous call to <code>DescribeSettings</code>. Pass null if this is the first call.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeSettingsInput`](crate::operation::describe_settings::DescribeSettingsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_settings::DescribeSettingsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_settings::DescribeSettingsInput {
            directory_id: self.directory_id,
            status: self.status,
            next_token: self.next_token,
        })
    }
}
