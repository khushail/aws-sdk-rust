// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl CreateLoadBalancerListenersInput {
    /// Consumes the builder and constructs an Operation<[`CreateLoadBalancerListeners`](crate::operation::create_load_balancer_listeners::CreateLoadBalancerListeners)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::create_load_balancer_listeners::CreateLoadBalancerListeners,
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
        let mut request = {
            fn uri_base(
                _input: &crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersInput,
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
                "application/x-www-form-urlencoded",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_create_load_balancer_listeners_input::ser_create_load_balancer_listeners_input_input(&self)?
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
            crate::operation::create_load_balancer_listeners::CreateLoadBalancerListeners::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "CreateLoadBalancerListeners",
            "elasticloadbalancing",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `CreateLoadBalancerListeners`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct CreateLoadBalancerListeners;
impl CreateLoadBalancerListeners {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for CreateLoadBalancerListeners {
    type Output = ::std::result::Result<
        crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersOutput,
        crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_create_load_balancer_listeners::de_create_load_balancer_listeners_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_create_load_balancer_listeners::de_create_load_balancer_listeners_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type CreateLoadBalancerListenersErrorKind = CreateLoadBalancerListenersError;
/// Error type for the `CreateLoadBalancerListenersError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum CreateLoadBalancerListenersError {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFoundException(crate::types::error::AccessPointNotFoundException),
    /// <p>The specified ARN does not refer to a valid SSL certificate in AWS Identity and Access Management (IAM) or AWS Certificate Manager (ACM). Note that if you recently uploaded the certificate to IAM, this error might indicate that the certificate is not fully available yet.</p>
    CertificateNotFoundException(crate::types::error::CertificateNotFoundException),
    /// <p>A listener already exists for the specified load balancer name and port, but with a different instance port, protocol, or SSL certificate.</p>
    DuplicateListenerException(crate::types::error::DuplicateListenerException),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequestException(crate::types::error::InvalidConfigurationRequestException),
    /// <p>The specified protocol or signature version is not supported.</p>
    UnsupportedProtocolException(crate::types::error::UnsupportedProtocolException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for CreateLoadBalancerListenersError {
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
impl ::std::fmt::Display for CreateLoadBalancerListenersError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::AccessPointNotFoundException(_inner) => _inner.fmt(f),
            Self::CertificateNotFoundException(_inner) => _inner.fmt(f),
            Self::DuplicateListenerException(_inner) => _inner.fmt(f),
            Self::InvalidConfigurationRequestException(_inner) => _inner.fmt(f),
            Self::UnsupportedProtocolException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata
    for CreateLoadBalancerListenersError
{
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AccessPointNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CertificateNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DuplicateListenerException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidConfigurationRequestException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnsupportedProtocolException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for CreateLoadBalancerListenersError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl CreateLoadBalancerListenersError {
    /// Creates the `CreateLoadBalancerListenersError::Unhandled` variant from any error type.
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

    /// Creates the `CreateLoadBalancerListenersError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::AccessPointNotFoundException(e) => e.meta(),
            Self::CertificateNotFoundException(e) => e.meta(),
            Self::DuplicateListenerException(e) => e.meta(),
            Self::InvalidConfigurationRequestException(e) => e.meta(),
            Self::UnsupportedProtocolException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `CreateLoadBalancerListenersError::AccessPointNotFoundException`.
    pub fn is_access_point_not_found_exception(&self) -> bool {
        matches!(self, Self::AccessPointNotFoundException(_))
    }
    /// Returns `true` if the error kind is `CreateLoadBalancerListenersError::CertificateNotFoundException`.
    pub fn is_certificate_not_found_exception(&self) -> bool {
        matches!(self, Self::CertificateNotFoundException(_))
    }
    /// Returns `true` if the error kind is `CreateLoadBalancerListenersError::DuplicateListenerException`.
    pub fn is_duplicate_listener_exception(&self) -> bool {
        matches!(self, Self::DuplicateListenerException(_))
    }
    /// Returns `true` if the error kind is `CreateLoadBalancerListenersError::InvalidConfigurationRequestException`.
    pub fn is_invalid_configuration_request_exception(&self) -> bool {
        matches!(self, Self::InvalidConfigurationRequestException(_))
    }
    /// Returns `true` if the error kind is `CreateLoadBalancerListenersError::UnsupportedProtocolException`.
    pub fn is_unsupported_protocol_exception(&self) -> bool {
        matches!(self, Self::UnsupportedProtocolException(_))
    }
}
impl ::std::error::Error for CreateLoadBalancerListenersError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::AccessPointNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::CertificateNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::DuplicateListenerException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidConfigurationRequestException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::UnsupportedProtocolException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::create_load_balancer_listeners::_create_load_balancer_listeners_output::CreateLoadBalancerListenersOutput;

pub use crate::operation::create_load_balancer_listeners::_create_load_balancer_listeners_input::CreateLoadBalancerListenersInput;

mod _create_load_balancer_listeners_input;

mod _create_load_balancer_listeners_output;

/// Builders
pub mod builders;
