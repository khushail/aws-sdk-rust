// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListStreamingSessionBackupsInput {
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The user ID of the user that owns the streaming session.</p>
    #[doc(hidden)]
    pub owned_by: ::std::option::Option<::std::string::String>,
    /// <p>The studio ID. </p>
    #[doc(hidden)]
    pub studio_id: ::std::option::Option<::std::string::String>,
}
impl ListStreamingSessionBackupsInput {
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The user ID of the user that owns the streaming session.</p>
    pub fn owned_by(&self) -> ::std::option::Option<&str> {
        self.owned_by.as_deref()
    }
    /// <p>The studio ID. </p>
    pub fn studio_id(&self) -> ::std::option::Option<&str> {
        self.studio_id.as_deref()
    }
}
impl ListStreamingSessionBackupsInput {
    /// Creates a new builder-style object to manufacture [`ListStreamingSessionBackupsInput`](crate::operation::list_streaming_session_backups::ListStreamingSessionBackupsInput).
    pub fn builder() -> crate::operation::list_streaming_session_backups::builders::ListStreamingSessionBackupsInputBuilder{
        crate::operation::list_streaming_session_backups::builders::ListStreamingSessionBackupsInputBuilder::default()
    }
}

/// A builder for [`ListStreamingSessionBackupsInput`](crate::operation::list_streaming_session_backups::ListStreamingSessionBackupsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListStreamingSessionBackupsInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) owned_by: ::std::option::Option<::std::string::String>,
    pub(crate) studio_id: ::std::option::Option<::std::string::String>,
}
impl ListStreamingSessionBackupsInputBuilder {
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of results, or null if there are no more results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The user ID of the user that owns the streaming session.</p>
    pub fn owned_by(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owned_by = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user ID of the user that owns the streaming session.</p>
    pub fn set_owned_by(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owned_by = input;
        self
    }
    /// <p>The studio ID. </p>
    pub fn studio_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.studio_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The studio ID. </p>
    pub fn set_studio_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.studio_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ListStreamingSessionBackupsInput`](crate::operation::list_streaming_session_backups::ListStreamingSessionBackupsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_streaming_session_backups::ListStreamingSessionBackupsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_streaming_session_backups::ListStreamingSessionBackupsInput {
                next_token: self.next_token,
                owned_by: self.owned_by,
                studio_id: self.studio_id,
            },
        )
    }
}
