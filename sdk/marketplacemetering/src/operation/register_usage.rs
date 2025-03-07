// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl RegisterUsageInput {
    /// Consumes the builder and constructs an Operation<[`RegisterUsage`](crate::operation::register_usage::RegisterUsage)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::register_usage::RegisterUsage,
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
                _input: &crate::operation::register_usage::RegisterUsageInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::register_usage::RegisterUsageInput,
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
                "AWSMPMeteringService.RegisterUsage",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_register_usage::ser_register_usage_input(&self)?,
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
            crate::operation::register_usage::RegisterUsage::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "RegisterUsage",
            "marketplacemetering",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `RegisterUsage`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct RegisterUsage;
impl RegisterUsage {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for RegisterUsage {
    type Output = ::std::result::Result<
        crate::operation::register_usage::RegisterUsageOutput,
        crate::operation::register_usage::RegisterUsageError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_register_usage::de_register_usage_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_register_usage::de_register_usage_http_response_with_props(
                status, headers, body,
            )
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type RegisterUsageErrorKind = RegisterUsageError;
/// Error type for the `RegisterUsageError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum RegisterUsageError {
    /// <p>Exception thrown when the customer does not have a valid subscription for the product.</p>
    CustomerNotEntitledException(crate::types::error::CustomerNotEntitledException),
    /// <p>The API is disabled in the Region.</p>
    DisabledApiException(crate::types::error::DisabledApiException),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceErrorException(crate::types::error::InternalServiceErrorException),
    /// <p>The product code passed does not match the product code used for publishing the product.</p>
    InvalidProductCodeException(crate::types::error::InvalidProductCodeException),
    /// <p>Public Key version is invalid.</p>
    InvalidPublicKeyVersionException(crate::types::error::InvalidPublicKeyVersionException),
    /// <p> <code>RegisterUsage</code> must be called in the same AWS Region the ECS task was launched in. This prevents a container from hardcoding a Region (e.g. withRegion(“us-east-1”) when calling <code>RegisterUsage</code>.</p>
    InvalidRegionException(crate::types::error::InvalidRegionException),
    /// <p>AWS Marketplace does not support metering usage from the underlying platform. Currently, Amazon ECS, Amazon EKS, and AWS Fargate are supported.</p>
    PlatformNotSupportedException(crate::types::error::PlatformNotSupportedException),
    /// <p>The calls to the API are throttled.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for RegisterUsageError {
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
impl ::std::fmt::Display for RegisterUsageError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::CustomerNotEntitledException(_inner) => _inner.fmt(f),
            Self::DisabledApiException(_inner) => _inner.fmt(f),
            Self::InternalServiceErrorException(_inner) => _inner.fmt(f),
            Self::InvalidProductCodeException(_inner) => _inner.fmt(f),
            Self::InvalidPublicKeyVersionException(_inner) => _inner.fmt(f),
            Self::InvalidRegionException(_inner) => _inner.fmt(f),
            Self::PlatformNotSupportedException(_inner) => _inner.fmt(f),
            Self::ThrottlingException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for RegisterUsageError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::CustomerNotEntitledException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DisabledApiException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalServiceErrorException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidProductCodeException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidPublicKeyVersionException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidRegionException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::PlatformNotSupportedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ThrottlingException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::register_usage::RegisterUsageError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for RegisterUsageError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl RegisterUsageError {
    /// Creates the `RegisterUsageError::Unhandled` variant from any error type.
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

    /// Creates the `RegisterUsageError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::CustomerNotEntitledException(e) => e.meta(),
            Self::DisabledApiException(e) => e.meta(),
            Self::InternalServiceErrorException(e) => e.meta(),
            Self::InvalidProductCodeException(e) => e.meta(),
            Self::InvalidPublicKeyVersionException(e) => e.meta(),
            Self::InvalidRegionException(e) => e.meta(),
            Self::PlatformNotSupportedException(e) => e.meta(),
            Self::ThrottlingException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `RegisterUsageError::CustomerNotEntitledException`.
    pub fn is_customer_not_entitled_exception(&self) -> bool {
        matches!(self, Self::CustomerNotEntitledException(_))
    }
    /// Returns `true` if the error kind is `RegisterUsageError::DisabledApiException`.
    pub fn is_disabled_api_exception(&self) -> bool {
        matches!(self, Self::DisabledApiException(_))
    }
    /// Returns `true` if the error kind is `RegisterUsageError::InternalServiceErrorException`.
    pub fn is_internal_service_error_exception(&self) -> bool {
        matches!(self, Self::InternalServiceErrorException(_))
    }
    /// Returns `true` if the error kind is `RegisterUsageError::InvalidProductCodeException`.
    pub fn is_invalid_product_code_exception(&self) -> bool {
        matches!(self, Self::InvalidProductCodeException(_))
    }
    /// Returns `true` if the error kind is `RegisterUsageError::InvalidPublicKeyVersionException`.
    pub fn is_invalid_public_key_version_exception(&self) -> bool {
        matches!(self, Self::InvalidPublicKeyVersionException(_))
    }
    /// Returns `true` if the error kind is `RegisterUsageError::InvalidRegionException`.
    pub fn is_invalid_region_exception(&self) -> bool {
        matches!(self, Self::InvalidRegionException(_))
    }
    /// Returns `true` if the error kind is `RegisterUsageError::PlatformNotSupportedException`.
    pub fn is_platform_not_supported_exception(&self) -> bool {
        matches!(self, Self::PlatformNotSupportedException(_))
    }
    /// Returns `true` if the error kind is `RegisterUsageError::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(self, Self::ThrottlingException(_))
    }
}
impl ::std::error::Error for RegisterUsageError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::CustomerNotEntitledException(_inner) => ::std::option::Option::Some(_inner),
            Self::DisabledApiException(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalServiceErrorException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidProductCodeException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidPublicKeyVersionException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidRegionException(_inner) => ::std::option::Option::Some(_inner),
            Self::PlatformNotSupportedException(_inner) => ::std::option::Option::Some(_inner),
            Self::ThrottlingException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::register_usage::_register_usage_output::RegisterUsageOutput;

pub use crate::operation::register_usage::_register_usage_input::RegisterUsageInput;

mod _register_usage_input;

mod _register_usage_output;

/// Builders
pub mod builders;
