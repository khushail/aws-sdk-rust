// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the configuration of a request to exchange an access code for a token.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ExchangeCodeForTokenRequestBody {
    /// <p>The access code to send in the request.</p>
    #[doc(hidden)]
    pub code: ::std::option::Option<::std::string::String>,
    /// <p>The location of the application that will receive the access code.</p>
    #[doc(hidden)]
    pub redirect_uri: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the client to request the token from.</p>
    #[doc(hidden)]
    pub client_id: ::std::option::Option<::std::string::String>,
}
impl ExchangeCodeForTokenRequestBody {
    /// <p>The access code to send in the request.</p>
    pub fn code(&self) -> ::std::option::Option<&str> {
        self.code.as_deref()
    }
    /// <p>The location of the application that will receive the access code.</p>
    pub fn redirect_uri(&self) -> ::std::option::Option<&str> {
        self.redirect_uri.as_deref()
    }
    /// <p>The ID of the client to request the token from.</p>
    pub fn client_id(&self) -> ::std::option::Option<&str> {
        self.client_id.as_deref()
    }
}
impl ::std::fmt::Debug for ExchangeCodeForTokenRequestBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ExchangeCodeForTokenRequestBody");
        formatter.field("code", &"*** Sensitive Data Redacted ***");
        formatter.field("redirect_uri", &self.redirect_uri);
        formatter.field("client_id", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl ExchangeCodeForTokenRequestBody {
    /// Creates a new builder-style object to manufacture [`ExchangeCodeForTokenRequestBody`](crate::types::ExchangeCodeForTokenRequestBody).
    pub fn builder() -> crate::types::builders::ExchangeCodeForTokenRequestBodyBuilder {
        crate::types::builders::ExchangeCodeForTokenRequestBodyBuilder::default()
    }
}

/// A builder for [`ExchangeCodeForTokenRequestBody`](crate::types::ExchangeCodeForTokenRequestBody).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ExchangeCodeForTokenRequestBodyBuilder {
    pub(crate) code: ::std::option::Option<::std::string::String>,
    pub(crate) redirect_uri: ::std::option::Option<::std::string::String>,
    pub(crate) client_id: ::std::option::Option<::std::string::String>,
}
impl ExchangeCodeForTokenRequestBodyBuilder {
    /// <p>The access code to send in the request.</p>
    pub fn code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The access code to send in the request.</p>
    pub fn set_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code = input;
        self
    }
    /// <p>The location of the application that will receive the access code.</p>
    pub fn redirect_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.redirect_uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The location of the application that will receive the access code.</p>
    pub fn set_redirect_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.redirect_uri = input;
        self
    }
    /// <p>The ID of the client to request the token from.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the client to request the token from.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ExchangeCodeForTokenRequestBody`](crate::types::ExchangeCodeForTokenRequestBody).
    pub fn build(self) -> crate::types::ExchangeCodeForTokenRequestBody {
        crate::types::ExchangeCodeForTokenRequestBody {
            code: self.code,
            redirect_uri: self.redirect_uri,
            client_id: self.client_id,
        }
    }
}
impl ::std::fmt::Debug for ExchangeCodeForTokenRequestBodyBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ExchangeCodeForTokenRequestBodyBuilder");
        formatter.field("code", &"*** Sensitive Data Redacted ***");
        formatter.field("redirect_uri", &self.redirect_uri);
        formatter.field("client_id", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
