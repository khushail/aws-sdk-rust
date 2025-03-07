// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a key pair.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateKeyPairOutput {
    /// <ul>
    /// <li> <p>For RSA key pairs, the key fingerprint is the SHA-1 digest of the DER encoded private key.</p> </li>
    /// <li> <p>For ED25519 key pairs, the key fingerprint is the base64-encoded SHA-256 digest, which is the default for OpenSSH, starting with OpenSSH 6.8.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub key_fingerprint: ::std::option::Option<::std::string::String>,
    /// <p>An unencrypted PEM encoded RSA or ED25519 private key.</p>
    #[doc(hidden)]
    pub key_material: ::std::option::Option<::std::string::String>,
    /// <p>The name of the key pair.</p>
    #[doc(hidden)]
    pub key_name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the key pair.</p>
    #[doc(hidden)]
    pub key_pair_id: ::std::option::Option<::std::string::String>,
    /// <p>Any tags applied to the key pair.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl CreateKeyPairOutput {
    /// <ul>
    /// <li> <p>For RSA key pairs, the key fingerprint is the SHA-1 digest of the DER encoded private key.</p> </li>
    /// <li> <p>For ED25519 key pairs, the key fingerprint is the base64-encoded SHA-256 digest, which is the default for OpenSSH, starting with OpenSSH 6.8.</p> </li>
    /// </ul>
    pub fn key_fingerprint(&self) -> ::std::option::Option<&str> {
        self.key_fingerprint.as_deref()
    }
    /// <p>An unencrypted PEM encoded RSA or ED25519 private key.</p>
    pub fn key_material(&self) -> ::std::option::Option<&str> {
        self.key_material.as_deref()
    }
    /// <p>The name of the key pair.</p>
    pub fn key_name(&self) -> ::std::option::Option<&str> {
        self.key_name.as_deref()
    }
    /// <p>The ID of the key pair.</p>
    pub fn key_pair_id(&self) -> ::std::option::Option<&str> {
        self.key_pair_id.as_deref()
    }
    /// <p>Any tags applied to the key pair.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl ::std::fmt::Debug for CreateKeyPairOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateKeyPairOutput");
        formatter.field("key_fingerprint", &self.key_fingerprint);
        formatter.field("key_material", &"*** Sensitive Data Redacted ***");
        formatter.field("key_name", &self.key_name);
        formatter.field("key_pair_id", &self.key_pair_id);
        formatter.field("tags", &self.tags);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_http::request_id::RequestId for CreateKeyPairOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateKeyPairOutput {
    /// Creates a new builder-style object to manufacture [`CreateKeyPairOutput`](crate::operation::create_key_pair::CreateKeyPairOutput).
    pub fn builder() -> crate::operation::create_key_pair::builders::CreateKeyPairOutputBuilder {
        crate::operation::create_key_pair::builders::CreateKeyPairOutputBuilder::default()
    }
}

/// A builder for [`CreateKeyPairOutput`](crate::operation::create_key_pair::CreateKeyPairOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct CreateKeyPairOutputBuilder {
    pub(crate) key_fingerprint: ::std::option::Option<::std::string::String>,
    pub(crate) key_material: ::std::option::Option<::std::string::String>,
    pub(crate) key_name: ::std::option::Option<::std::string::String>,
    pub(crate) key_pair_id: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl CreateKeyPairOutputBuilder {
    /// <ul>
    /// <li> <p>For RSA key pairs, the key fingerprint is the SHA-1 digest of the DER encoded private key.</p> </li>
    /// <li> <p>For ED25519 key pairs, the key fingerprint is the base64-encoded SHA-256 digest, which is the default for OpenSSH, starting with OpenSSH 6.8.</p> </li>
    /// </ul>
    pub fn key_fingerprint(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.key_fingerprint = ::std::option::Option::Some(input.into());
        self
    }
    /// <ul>
    /// <li> <p>For RSA key pairs, the key fingerprint is the SHA-1 digest of the DER encoded private key.</p> </li>
    /// <li> <p>For ED25519 key pairs, the key fingerprint is the base64-encoded SHA-256 digest, which is the default for OpenSSH, starting with OpenSSH 6.8.</p> </li>
    /// </ul>
    pub fn set_key_fingerprint(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.key_fingerprint = input;
        self
    }
    /// <p>An unencrypted PEM encoded RSA or ED25519 private key.</p>
    pub fn key_material(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_material = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An unencrypted PEM encoded RSA or ED25519 private key.</p>
    pub fn set_key_material(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_material = input;
        self
    }
    /// <p>The name of the key pair.</p>
    pub fn key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the key pair.</p>
    pub fn set_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_name = input;
        self
    }
    /// <p>The ID of the key pair.</p>
    pub fn key_pair_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_pair_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the key pair.</p>
    pub fn set_key_pair_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_pair_id = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags applied to the key pair.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Any tags applied to the key pair.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
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
    /// Consumes the builder and constructs a [`CreateKeyPairOutput`](crate::operation::create_key_pair::CreateKeyPairOutput).
    pub fn build(self) -> crate::operation::create_key_pair::CreateKeyPairOutput {
        crate::operation::create_key_pair::CreateKeyPairOutput {
            key_fingerprint: self.key_fingerprint,
            key_material: self.key_material,
            key_name: self.key_name,
            key_pair_id: self.key_pair_id,
            tags: self.tags,
            _request_id: self._request_id,
        }
    }
}
impl ::std::fmt::Debug for CreateKeyPairOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateKeyPairOutputBuilder");
        formatter.field("key_fingerprint", &self.key_fingerprint);
        formatter.field("key_material", &"*** Sensitive Data Redacted ***");
        formatter.field("key_name", &self.key_name);
        formatter.field("key_pair_id", &self.key_pair_id);
        formatter.field("tags", &self.tags);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
