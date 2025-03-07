// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCertificatesOutput {
    /// <p>Returns the next token, which you can use to list the next certificate.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Returns an array of the certificates that are specified in the <code>ListCertificates</code> call.</p>
    #[doc(hidden)]
    pub certificates: ::std::option::Option<::std::vec::Vec<crate::types::ListedCertificate>>,
    _request_id: Option<String>,
}
impl ListCertificatesOutput {
    /// <p>Returns the next token, which you can use to list the next certificate.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Returns an array of the certificates that are specified in the <code>ListCertificates</code> call.</p>
    pub fn certificates(&self) -> ::std::option::Option<&[crate::types::ListedCertificate]> {
        self.certificates.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListCertificatesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListCertificatesOutput {
    /// Creates a new builder-style object to manufacture [`ListCertificatesOutput`](crate::operation::list_certificates::ListCertificatesOutput).
    pub fn builder() -> crate::operation::list_certificates::builders::ListCertificatesOutputBuilder
    {
        crate::operation::list_certificates::builders::ListCertificatesOutputBuilder::default()
    }
}

/// A builder for [`ListCertificatesOutput`](crate::operation::list_certificates::ListCertificatesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListCertificatesOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) certificates:
        ::std::option::Option<::std::vec::Vec<crate::types::ListedCertificate>>,
    _request_id: Option<String>,
}
impl ListCertificatesOutputBuilder {
    /// <p>Returns the next token, which you can use to list the next certificate.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Returns the next token, which you can use to list the next certificate.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `certificates`.
    ///
    /// To override the contents of this collection use [`set_certificates`](Self::set_certificates).
    ///
    /// <p>Returns an array of the certificates that are specified in the <code>ListCertificates</code> call.</p>
    pub fn certificates(mut self, input: crate::types::ListedCertificate) -> Self {
        let mut v = self.certificates.unwrap_or_default();
        v.push(input);
        self.certificates = ::std::option::Option::Some(v);
        self
    }
    /// <p>Returns an array of the certificates that are specified in the <code>ListCertificates</code> call.</p>
    pub fn set_certificates(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ListedCertificate>>,
    ) -> Self {
        self.certificates = input;
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
    /// Consumes the builder and constructs a [`ListCertificatesOutput`](crate::operation::list_certificates::ListCertificatesOutput).
    pub fn build(self) -> crate::operation::list_certificates::ListCertificatesOutput {
        crate::operation::list_certificates::ListCertificatesOutput {
            next_token: self.next_token,
            certificates: self.certificates,
            _request_id: self._request_id,
        }
    }
}
