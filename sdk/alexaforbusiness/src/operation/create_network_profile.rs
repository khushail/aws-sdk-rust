// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl CreateNetworkProfileInput {
    /// Consumes the builder and constructs an Operation<[`CreateNetworkProfile`](crate::operation::create_network_profile::CreateNetworkProfile)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::create_network_profile::CreateNetworkProfile,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(_config.use_dual_stack)
            .set_use_fips(_config.use_fips)
            .set_endpoint(_config.endpoint_url.clone())
            .build()
            .map_err(|err| {
                ::aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            ::std::result::Result::Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                ::std::option::Option::Some(params),
            ),
            ::std::result::Result::Err(e) => {
                (::std::result::Result::Err(e), ::std::option::Option::None)
            }
        };
        if self.client_request_token.is_none() {
            self.client_request_token =
                ::std::option::Option::Some(_config.make_token.make_idempotency_token());
        }
        let mut request = {
            fn uri_base(
                _input: &crate::operation::create_network_profile::CreateNetworkProfileInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_network_profile::CreateNetworkProfileInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<
                ::http::request::Builder,
                ::aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, ::http::request::Builder::new())?;
            builder = ::aws_smithy_http::header::set_request_header_if_absent(
                builder,
                ::http::header::CONTENT_TYPE,
                "application/x-amz-json-1.1",
            );
            builder = ::aws_smithy_http::header::set_request_header_if_absent(
                builder,
                ::http::header::HeaderName::from_static("x-amz-target"),
                "AlexaForBusiness.CreateNetworkProfile",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_create_network_profile::ser_create_network_profile_input(
                &self,
            )?,
        );
        if let ::std::option::Option::Some(content_length) = body.content_length() {
            request = ::aws_smithy_http::header::set_request_header_if_absent(
                request,
                ::http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = ::aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let ::std::option::Option::Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(::aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = ::aws_http::user_agent::AwsUserAgent::new_from_environment(
            ::aws_types::os_shim_internal::Env::real(),
            crate::meta::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = ::aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(::aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(::aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        ::aws_http::auth::set_credentials_cache(
            &mut request.properties_mut(),
            _config.credentials_cache.clone(),
        );
        let op = ::aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::create_network_profile::CreateNetworkProfile::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "CreateNetworkProfile",
            "alexaforbusiness",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `CreateNetworkProfile`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct CreateNetworkProfile;
impl CreateNetworkProfile {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for CreateNetworkProfile {
    type Output = ::std::result::Result<
        crate::operation::create_network_profile::CreateNetworkProfileOutput,
        crate::operation::create_network_profile::CreateNetworkProfileError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_create_network_profile::de_create_network_profile_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_create_network_profile::de_create_network_profile_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type CreateNetworkProfileErrorKind = CreateNetworkProfileError;
/// Error type for the `CreateNetworkProfileError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum CreateNetworkProfileError {
    /// <p>The resource being created already exists.</p>
    AlreadyExistsException(crate::types::error::AlreadyExistsException),
    /// <p>There is a concurrent modification of resources.</p>
    ConcurrentModificationException(crate::types::error::ConcurrentModificationException),
    /// <p>The Certificate Authority can't issue or revoke a certificate.</p>
    InvalidCertificateAuthorityException(crate::types::error::InvalidCertificateAuthorityException),
    /// <p>The service linked role is locked for deletion. </p>
    InvalidServiceLinkedRoleStateException(
        crate::types::error::InvalidServiceLinkedRoleStateException,
    ),
    /// <p>You are performing an action that would put you beyond your account's limits.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for CreateNetworkProfileError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<
            dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
        >,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = ::aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl ::std::fmt::Display for CreateNetworkProfileError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::AlreadyExistsException(_inner) => _inner.fmt(f),
            Self::ConcurrentModificationException(_inner) => _inner.fmt(f),
            Self::InvalidCertificateAuthorityException(_inner) => _inner.fmt(f),
            Self::InvalidServiceLinkedRoleStateException(_inner) => _inner.fmt(f),
            Self::LimitExceededException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateNetworkProfileError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AlreadyExistsException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ConcurrentModificationException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidCertificateAuthorityException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidServiceLinkedRoleStateException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::LimitExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::create_network_profile::CreateNetworkProfileError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for CreateNetworkProfileError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl CreateNetworkProfileError {
    /// Creates the `CreateNetworkProfileError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<
            ::std::boxed::Box<
                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
            >,
        >,
    ) -> Self {
        Self::Unhandled(
            ::aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `CreateNetworkProfileError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(
            ::aws_smithy_types::error::Unhandled::builder()
                .source(err.clone())
                .meta(err)
                .build(),
        )
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::AlreadyExistsException(e) => e.meta(),
            Self::ConcurrentModificationException(e) => e.meta(),
            Self::InvalidCertificateAuthorityException(e) => e.meta(),
            Self::InvalidServiceLinkedRoleStateException(e) => e.meta(),
            Self::LimitExceededException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `CreateNetworkProfileError::AlreadyExistsException`.
    pub fn is_already_exists_exception(&self) -> bool {
        matches!(self, Self::AlreadyExistsException(_))
    }
    /// Returns `true` if the error kind is `CreateNetworkProfileError::ConcurrentModificationException`.
    pub fn is_concurrent_modification_exception(&self) -> bool {
        matches!(self, Self::ConcurrentModificationException(_))
    }
    /// Returns `true` if the error kind is `CreateNetworkProfileError::InvalidCertificateAuthorityException`.
    pub fn is_invalid_certificate_authority_exception(&self) -> bool {
        matches!(self, Self::InvalidCertificateAuthorityException(_))
    }
    /// Returns `true` if the error kind is `CreateNetworkProfileError::InvalidServiceLinkedRoleStateException`.
    pub fn is_invalid_service_linked_role_state_exception(&self) -> bool {
        matches!(self, Self::InvalidServiceLinkedRoleStateException(_))
    }
    /// Returns `true` if the error kind is `CreateNetworkProfileError::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::LimitExceededException(_))
    }
}
impl ::std::error::Error for CreateNetworkProfileError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::AlreadyExistsException(_inner) => ::std::option::Option::Some(_inner),
            Self::ConcurrentModificationException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidCertificateAuthorityException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::InvalidServiceLinkedRoleStateException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::LimitExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::create_network_profile::_create_network_profile_output::CreateNetworkProfileOutput;

pub use crate::operation::create_network_profile::_create_network_profile_input::CreateNetworkProfileInput;

mod _create_network_profile_input;

mod _create_network_profile_output;

/// Builders
pub mod builders;
