/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_http::query_writer::QueryWriter;
use aws_smithy_runtime_api::client::auth::http::{
    HTTP_API_KEY_AUTH_SCHEME_ID, HTTP_BASIC_AUTH_SCHEME_ID, HTTP_BEARER_AUTH_SCHEME_ID,
    HTTP_DIGEST_AUTH_SCHEME_ID,
};
use aws_smithy_runtime_api::client::auth::{
    AuthSchemeEndpointConfig, AuthSchemeId, HttpAuthScheme, HttpRequestSigner,
};
use aws_smithy_runtime_api::client::identity::http::{Login, Token};
use aws_smithy_runtime_api::client::identity::{Identity, IdentityResolver, IdentityResolvers};
use aws_smithy_runtime_api::client::orchestrator::{BoxError, HttpRequest};
use aws_smithy_runtime_api::config_bag::ConfigBag;
use aws_smithy_types::base64::encode;
use http::header::HeaderName;
use http::HeaderValue;

/// Destination for the API key
#[derive(Copy, Clone, Debug)]
pub enum ApiKeyLocation {
    Query,
    Header,
}

/// Auth implementation for Smithy's `@httpApiKey` auth scheme
#[derive(Debug)]
pub struct ApiKeyAuthScheme {
    signer: ApiKeySigner,
}

impl ApiKeyAuthScheme {
    /// Creates a new `ApiKeyAuthScheme`.
    pub fn new(
        scheme: impl Into<String>,
        location: ApiKeyLocation,
        name: impl Into<String>,
    ) -> Self {
        Self {
            signer: ApiKeySigner {
                scheme: scheme.into(),
                location,
                name: name.into(),
            },
        }
    }
}

impl HttpAuthScheme for ApiKeyAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        HTTP_API_KEY_AUTH_SCHEME_ID
    }

    fn identity_resolver<'a>(
        &self,
        identity_resolvers: &'a IdentityResolvers,
    ) -> Option<&'a dyn IdentityResolver> {
        identity_resolvers.identity_resolver(self.scheme_id())
    }

    fn request_signer(&self) -> &dyn HttpRequestSigner {
        &self.signer
    }
}

#[derive(Debug)]
struct ApiKeySigner {
    scheme: String,
    location: ApiKeyLocation,
    name: String,
}

impl HttpRequestSigner for ApiKeySigner {
    fn sign_request(
        &self,
        request: &mut HttpRequest,
        identity: &Identity,
        _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        _config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        let api_key = identity
            .data::<Token>()
            .ok_or("HTTP ApiKey auth requires a `Token` identity")?;
        match self.location {
            ApiKeyLocation::Header => {
                request.headers_mut().append(
                    HeaderName::try_from(&self.name).expect("valid API key header name"),
                    HeaderValue::try_from(format!("{} {}", self.scheme, api_key.token())).map_err(
                        |_| "API key contains characters that can't be included in a HTTP header",
                    )?,
                );
            }
            ApiKeyLocation::Query => {
                let mut query = QueryWriter::new(request.uri());
                query.insert(&self.name, api_key.token());
                *request.uri_mut() = query.build_uri();
            }
        }

        Ok(())
    }
}

/// Auth implementation for Smithy's `@httpBasicAuth` auth scheme
#[derive(Debug, Default)]
pub struct BasicAuthScheme {
    signer: BasicAuthSigner,
}

impl BasicAuthScheme {
    /// Creates a new `BasicAuthScheme`.
    pub fn new() -> Self {
        Self {
            signer: BasicAuthSigner,
        }
    }
}

impl HttpAuthScheme for BasicAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        HTTP_BASIC_AUTH_SCHEME_ID
    }

    fn identity_resolver<'a>(
        &self,
        identity_resolvers: &'a IdentityResolvers,
    ) -> Option<&'a dyn IdentityResolver> {
        identity_resolvers.identity_resolver(self.scheme_id())
    }

    fn request_signer(&self) -> &dyn HttpRequestSigner {
        &self.signer
    }
}

#[derive(Debug, Default)]
struct BasicAuthSigner;

impl HttpRequestSigner for BasicAuthSigner {
    fn sign_request(
        &self,
        request: &mut HttpRequest,
        identity: &Identity,
        _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        _config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        let login = identity
            .data::<Login>()
            .ok_or("HTTP basic auth requires a `Login` identity")?;
        request.headers_mut().insert(
            http::header::AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Basic {}",
                encode(format!("{}:{}", login.user(), login.password()))
            ))
            .expect("valid header value"),
        );
        Ok(())
    }
}

/// Auth implementation for Smithy's `@httpBearerAuth` auth scheme
#[derive(Debug, Default)]
pub struct BearerAuthScheme {
    signer: BearerAuthSigner,
}

impl BearerAuthScheme {
    /// Creates a new `BearerAuthScheme`.
    pub fn new() -> Self {
        Self {
            signer: BearerAuthSigner,
        }
    }
}

impl HttpAuthScheme for BearerAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        HTTP_BEARER_AUTH_SCHEME_ID
    }

    fn identity_resolver<'a>(
        &self,
        identity_resolvers: &'a IdentityResolvers,
    ) -> Option<&'a dyn IdentityResolver> {
        identity_resolvers.identity_resolver(self.scheme_id())
    }

    fn request_signer(&self) -> &dyn HttpRequestSigner {
        &self.signer
    }
}

#[derive(Debug, Default)]
struct BearerAuthSigner;

impl HttpRequestSigner for BearerAuthSigner {
    fn sign_request(
        &self,
        request: &mut HttpRequest,
        identity: &Identity,
        _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        _config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        let token = identity
            .data::<Token>()
            .ok_or("HTTP bearer auth requires a `Token` identity")?;
        request.headers_mut().insert(
            http::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token.token())).map_err(|_| {
                "Bearer token contains characters that can't be included in a HTTP header"
            })?,
        );
        Ok(())
    }
}

/// Auth implementation for Smithy's `@httpDigestAuth` auth scheme
#[derive(Debug, Default)]
pub struct DigestAuthScheme {
    signer: DigestAuthSigner,
}

impl DigestAuthScheme {
    /// Creates a new `DigestAuthScheme`.
    pub fn new() -> Self {
        Self {
            signer: DigestAuthSigner,
        }
    }
}

impl HttpAuthScheme for DigestAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        HTTP_DIGEST_AUTH_SCHEME_ID
    }

    fn identity_resolver<'a>(
        &self,
        identity_resolvers: &'a IdentityResolvers,
    ) -> Option<&'a dyn IdentityResolver> {
        identity_resolvers.identity_resolver(self.scheme_id())
    }

    fn request_signer(&self) -> &dyn HttpRequestSigner {
        &self.signer
    }
}

#[derive(Debug, Default)]
struct DigestAuthSigner;

impl HttpRequestSigner for DigestAuthSigner {
    fn sign_request(
        &self,
        _request: &mut HttpRequest,
        _identity: &Identity,
        _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        _config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        unimplemented!(
            "support for signing with Smithy's `@httpDigestAuth` auth scheme is not implemented yet"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_smithy_http::body::SdkBody;
    use aws_smithy_runtime_api::client::identity::http::Login;

    #[test]
    fn test_api_key_signing_headers() {
        let signer = ApiKeySigner {
            scheme: "SomeSchemeName".into(),
            location: ApiKeyLocation::Header,
            name: "some-header-name".into(),
        };
        let config_bag = ConfigBag::base();
        let identity = Identity::new(Token::new("some-token", None), None);
        let mut request = http::Request::builder()
            .uri("http://example.com/Foobaz")
            .body(SdkBody::empty())
            .unwrap();
        signer
            .sign_request(
                &mut request,
                &identity,
                AuthSchemeEndpointConfig::empty(),
                &config_bag,
            )
            .expect("success");
        assert_eq!(
            "SomeSchemeName some-token",
            request.headers().get("some-header-name").unwrap()
        );
        assert_eq!("http://example.com/Foobaz", request.uri().to_string());
    }

    #[test]
    fn test_api_key_signing_query() {
        let signer = ApiKeySigner {
            scheme: "".into(),
            location: ApiKeyLocation::Query,
            name: "some-query-name".into(),
        };
        let config_bag = ConfigBag::base();
        let identity = Identity::new(Token::new("some-token", None), None);
        let mut request = http::Request::builder()
            .uri("http://example.com/Foobaz")
            .body(SdkBody::empty())
            .unwrap();
        signer
            .sign_request(
                &mut request,
                &identity,
                AuthSchemeEndpointConfig::empty(),
                &config_bag,
            )
            .expect("success");
        assert!(request.headers().get("some-query-name").is_none());
        assert_eq!(
            "http://example.com/Foobaz?some-query-name=some-token",
            request.uri().to_string()
        );
    }

    #[test]
    fn test_basic_auth() {
        let signer = BasicAuthSigner;
        let config_bag = ConfigBag::base();
        let identity = Identity::new(Login::new("Aladdin", "open sesame", None), None);
        let mut request = http::Request::builder().body(SdkBody::empty()).unwrap();

        signer
            .sign_request(
                &mut request,
                &identity,
                AuthSchemeEndpointConfig::empty(),
                &config_bag,
            )
            .expect("success");
        assert_eq!(
            "Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==",
            request.headers().get("Authorization").unwrap()
        );
    }

    #[test]
    fn test_bearer_auth() {
        let signer = BearerAuthSigner;

        let config_bag = ConfigBag::base();
        let identity = Identity::new(Token::new("some-token", None), None);
        let mut request = http::Request::builder().body(SdkBody::empty()).unwrap();
        signer
            .sign_request(
                &mut request,
                &identity,
                AuthSchemeEndpointConfig::empty(),
                &config_bag,
            )
            .expect("success");
        assert_eq!(
            "Bearer some-token",
            request.headers().get("Authorization").unwrap()
        );
    }
}
