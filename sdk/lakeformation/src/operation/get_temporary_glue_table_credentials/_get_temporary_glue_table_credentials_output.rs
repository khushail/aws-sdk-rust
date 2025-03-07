// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTemporaryGlueTableCredentialsOutput {
    /// <p>The access key ID for the temporary credentials.</p>
    #[doc(hidden)]
    pub access_key_id: ::std::option::Option<::std::string::String>,
    /// <p>The secret key for the temporary credentials.</p>
    #[doc(hidden)]
    pub secret_access_key: ::std::option::Option<::std::string::String>,
    /// <p>The session token for the temporary credentials.</p>
    #[doc(hidden)]
    pub session_token: ::std::option::Option<::std::string::String>,
    /// <p>The date and time when the temporary credentials expire.</p>
    #[doc(hidden)]
    pub expiration: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetTemporaryGlueTableCredentialsOutput {
    /// <p>The access key ID for the temporary credentials.</p>
    pub fn access_key_id(&self) -> ::std::option::Option<&str> {
        self.access_key_id.as_deref()
    }
    /// <p>The secret key for the temporary credentials.</p>
    pub fn secret_access_key(&self) -> ::std::option::Option<&str> {
        self.secret_access_key.as_deref()
    }
    /// <p>The session token for the temporary credentials.</p>
    pub fn session_token(&self) -> ::std::option::Option<&str> {
        self.session_token.as_deref()
    }
    /// <p>The date and time when the temporary credentials expire.</p>
    pub fn expiration(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.expiration.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetTemporaryGlueTableCredentialsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetTemporaryGlueTableCredentialsOutput {
    /// Creates a new builder-style object to manufacture [`GetTemporaryGlueTableCredentialsOutput`](crate::operation::get_temporary_glue_table_credentials::GetTemporaryGlueTableCredentialsOutput).
    pub fn builder() -> crate::operation::get_temporary_glue_table_credentials::builders::GetTemporaryGlueTableCredentialsOutputBuilder{
        crate::operation::get_temporary_glue_table_credentials::builders::GetTemporaryGlueTableCredentialsOutputBuilder::default()
    }
}

/// A builder for [`GetTemporaryGlueTableCredentialsOutput`](crate::operation::get_temporary_glue_table_credentials::GetTemporaryGlueTableCredentialsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetTemporaryGlueTableCredentialsOutputBuilder {
    pub(crate) access_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) secret_access_key: ::std::option::Option<::std::string::String>,
    pub(crate) session_token: ::std::option::Option<::std::string::String>,
    pub(crate) expiration: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetTemporaryGlueTableCredentialsOutputBuilder {
    /// <p>The access key ID for the temporary credentials.</p>
    pub fn access_key_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.access_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The access key ID for the temporary credentials.</p>
    pub fn set_access_key_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.access_key_id = input;
        self
    }
    /// <p>The secret key for the temporary credentials.</p>
    pub fn secret_access_key(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.secret_access_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The secret key for the temporary credentials.</p>
    pub fn set_secret_access_key(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.secret_access_key = input;
        self
    }
    /// <p>The session token for the temporary credentials.</p>
    pub fn session_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.session_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The session token for the temporary credentials.</p>
    pub fn set_session_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.session_token = input;
        self
    }
    /// <p>The date and time when the temporary credentials expire.</p>
    pub fn expiration(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.expiration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the temporary credentials expire.</p>
    pub fn set_expiration(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.expiration = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetTemporaryGlueTableCredentialsOutput`](crate::operation::get_temporary_glue_table_credentials::GetTemporaryGlueTableCredentialsOutput).
    pub fn build(self) -> crate::operation::get_temporary_glue_table_credentials::GetTemporaryGlueTableCredentialsOutput{
        crate::operation::get_temporary_glue_table_credentials::GetTemporaryGlueTableCredentialsOutput {
            access_key_id: self.access_key_id
            ,
            secret_access_key: self.secret_access_key
            ,
            session_token: self.session_token
            ,
            expiration: self.expiration
            ,
            _request_id: self._request_id,
        }
    }
}
