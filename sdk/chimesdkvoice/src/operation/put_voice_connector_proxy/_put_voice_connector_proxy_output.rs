// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutVoiceConnectorProxyOutput {
    /// <p>The proxy configuration details.</p>
    #[doc(hidden)]
    pub proxy: ::std::option::Option<crate::types::Proxy>,
    _request_id: Option<String>,
}
impl PutVoiceConnectorProxyOutput {
    /// <p>The proxy configuration details.</p>
    pub fn proxy(&self) -> ::std::option::Option<&crate::types::Proxy> {
        self.proxy.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for PutVoiceConnectorProxyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutVoiceConnectorProxyOutput {
    /// Creates a new builder-style object to manufacture [`PutVoiceConnectorProxyOutput`](crate::operation::put_voice_connector_proxy::PutVoiceConnectorProxyOutput).
    pub fn builder(
    ) -> crate::operation::put_voice_connector_proxy::builders::PutVoiceConnectorProxyOutputBuilder
    {
        crate::operation::put_voice_connector_proxy::builders::PutVoiceConnectorProxyOutputBuilder::default()
    }
}

/// A builder for [`PutVoiceConnectorProxyOutput`](crate::operation::put_voice_connector_proxy::PutVoiceConnectorProxyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutVoiceConnectorProxyOutputBuilder {
    pub(crate) proxy: ::std::option::Option<crate::types::Proxy>,
    _request_id: Option<String>,
}
impl PutVoiceConnectorProxyOutputBuilder {
    /// <p>The proxy configuration details.</p>
    pub fn proxy(mut self, input: crate::types::Proxy) -> Self {
        self.proxy = ::std::option::Option::Some(input);
        self
    }
    /// <p>The proxy configuration details.</p>
    pub fn set_proxy(mut self, input: ::std::option::Option<crate::types::Proxy>) -> Self {
        self.proxy = input;
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
    /// Consumes the builder and constructs a [`PutVoiceConnectorProxyOutput`](crate::operation::put_voice_connector_proxy::PutVoiceConnectorProxyOutput).
    pub fn build(
        self,
    ) -> crate::operation::put_voice_connector_proxy::PutVoiceConnectorProxyOutput {
        crate::operation::put_voice_connector_proxy::PutVoiceConnectorProxyOutput {
            proxy: self.proxy,
            _request_id: self._request_id,
        }
    }
}
