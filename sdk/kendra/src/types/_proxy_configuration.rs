// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the configuration information for a web proxy to connect to website hosts.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProxyConfiguration {
    /// <p>The name of the website host you want to connect to via a web proxy server.</p>
    /// <p>For example, the host name of https://a.example.com/page1.html is "a.example.com".</p>
    #[doc(hidden)]
    pub host: ::std::option::Option<::std::string::String>,
    /// <p>The port number of the website host you want to connect to via a web proxy server. </p>
    /// <p>For example, the port for https://a.example.com/page1.html is 443, the standard port for HTTPS.</p>
    #[doc(hidden)]
    pub port: ::std::option::Option<i32>,
    /// <p>Your secret ARN, which you can create in <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html">Secrets Manager</a> </p>
    /// <p>The credentials are optional. You use a secret if web proxy credentials are required to connect to a website host. Amazon Kendra currently support basic authentication to connect to a web proxy server. The secret stores your credentials.</p>
    #[doc(hidden)]
    pub credentials: ::std::option::Option<::std::string::String>,
}
impl ProxyConfiguration {
    /// <p>The name of the website host you want to connect to via a web proxy server.</p>
    /// <p>For example, the host name of https://a.example.com/page1.html is "a.example.com".</p>
    pub fn host(&self) -> ::std::option::Option<&str> {
        self.host.as_deref()
    }
    /// <p>The port number of the website host you want to connect to via a web proxy server. </p>
    /// <p>For example, the port for https://a.example.com/page1.html is 443, the standard port for HTTPS.</p>
    pub fn port(&self) -> ::std::option::Option<i32> {
        self.port
    }
    /// <p>Your secret ARN, which you can create in <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html">Secrets Manager</a> </p>
    /// <p>The credentials are optional. You use a secret if web proxy credentials are required to connect to a website host. Amazon Kendra currently support basic authentication to connect to a web proxy server. The secret stores your credentials.</p>
    pub fn credentials(&self) -> ::std::option::Option<&str> {
        self.credentials.as_deref()
    }
}
impl ProxyConfiguration {
    /// Creates a new builder-style object to manufacture [`ProxyConfiguration`](crate::types::ProxyConfiguration).
    pub fn builder() -> crate::types::builders::ProxyConfigurationBuilder {
        crate::types::builders::ProxyConfigurationBuilder::default()
    }
}

/// A builder for [`ProxyConfiguration`](crate::types::ProxyConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ProxyConfigurationBuilder {
    pub(crate) host: ::std::option::Option<::std::string::String>,
    pub(crate) port: ::std::option::Option<i32>,
    pub(crate) credentials: ::std::option::Option<::std::string::String>,
}
impl ProxyConfigurationBuilder {
    /// <p>The name of the website host you want to connect to via a web proxy server.</p>
    /// <p>For example, the host name of https://a.example.com/page1.html is "a.example.com".</p>
    pub fn host(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.host = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the website host you want to connect to via a web proxy server.</p>
    /// <p>For example, the host name of https://a.example.com/page1.html is "a.example.com".</p>
    pub fn set_host(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.host = input;
        self
    }
    /// <p>The port number of the website host you want to connect to via a web proxy server. </p>
    /// <p>For example, the port for https://a.example.com/page1.html is 443, the standard port for HTTPS.</p>
    pub fn port(mut self, input: i32) -> Self {
        self.port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The port number of the website host you want to connect to via a web proxy server. </p>
    /// <p>For example, the port for https://a.example.com/page1.html is 443, the standard port for HTTPS.</p>
    pub fn set_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.port = input;
        self
    }
    /// <p>Your secret ARN, which you can create in <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html">Secrets Manager</a> </p>
    /// <p>The credentials are optional. You use a secret if web proxy credentials are required to connect to a website host. Amazon Kendra currently support basic authentication to connect to a web proxy server. The secret stores your credentials.</p>
    pub fn credentials(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.credentials = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Your secret ARN, which you can create in <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html">Secrets Manager</a> </p>
    /// <p>The credentials are optional. You use a secret if web proxy credentials are required to connect to a website host. Amazon Kendra currently support basic authentication to connect to a web proxy server. The secret stores your credentials.</p>
    pub fn set_credentials(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.credentials = input;
        self
    }
    /// Consumes the builder and constructs a [`ProxyConfiguration`](crate::types::ProxyConfiguration).
    pub fn build(self) -> crate::types::ProxyConfiguration {
        crate::types::ProxyConfiguration {
            host: self.host,
            port: self.port,
            credentials: self.credentials,
        }
    }
}
