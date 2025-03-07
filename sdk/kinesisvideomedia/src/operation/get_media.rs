// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl GetMediaInput {
    /// Consumes the builder and constructs an Operation<[`GetMedia`](crate::operation::get_media::GetMedia)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::get_media::GetMedia,
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
                _input: &crate::operation::get_media::GetMediaInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/getMedia").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::get_media::GetMediaInput,
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
                "application/json",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_get_media::ser_get_media_input(&self)?,
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
            crate::operation::get_media::GetMedia::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "GetMedia",
            "kinesisvideomedia",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `GetMedia`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct GetMedia;
impl GetMedia {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseHttpResponse for GetMedia {
    type Output = ::std::result::Result<
        crate::operation::get_media::GetMediaOutput,
        crate::operation::get_media::GetMediaError,
    >;
    fn parse_unloaded(
        &self,
        response: &mut ::aws_smithy_http::operation::Response,
    ) -> ::std::option::Option<Self::Output> {
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return ::std::option::Option::None;
        }
        ::std::option::Option::Some(
            crate::protocol_serde::shape_get_media::de_get_media_op_response(response),
        )
    }
    fn parse_loaded(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::protocol_serde::shape_get_media::de_get_media_http_error(
            response.status().as_u16(),
            response.headers(),
            response.body().as_ref(),
        )
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type GetMediaErrorKind = GetMediaError;
/// Error type for the `GetMediaError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum GetMediaError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceededException(crate::types::error::ClientLimitExceededException),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client connections.</p>
    ConnectionLimitExceededException(crate::types::error::ConnectionLimitExceededException),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgumentException(crate::types::error::InvalidArgumentException),
    /// <p> Status Code: 400, Caller used wrong endpoint to write data to a stream. On receiving such an exception, the user must call <code>GetDataEndpoint</code> with <code>AccessMode</code> set to "READ" and use the endpoint Kinesis Video returns in the next <code>GetMedia</code> call. </p>
    InvalidEndpointException(crate::types::error::InvalidEndpointException),
    /// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
    NotAuthorizedException(crate::types::error::NotAuthorizedException),
    /// <p>Status Code: 404, The stream with the given name does not exist.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for GetMediaError {
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
impl ::std::fmt::Display for GetMediaError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::ClientLimitExceededException(_inner) => _inner.fmt(f),
            Self::ConnectionLimitExceededException(_inner) => _inner.fmt(f),
            Self::InvalidArgumentException(_inner) => _inner.fmt(f),
            Self::InvalidEndpointException(_inner) => _inner.fmt(f),
            Self::NotAuthorizedException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetMediaError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ClientLimitExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ConnectionLimitExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidArgumentException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidEndpointException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NotAuthorizedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::get_media::GetMediaError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for GetMediaError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl GetMediaError {
    /// Creates the `GetMediaError::Unhandled` variant from any error type.
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

    /// Creates the `GetMediaError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::ClientLimitExceededException(e) => e.meta(),
            Self::ConnectionLimitExceededException(e) => e.meta(),
            Self::InvalidArgumentException(e) => e.meta(),
            Self::InvalidEndpointException(e) => e.meta(),
            Self::NotAuthorizedException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `GetMediaError::ClientLimitExceededException`.
    pub fn is_client_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::ClientLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `GetMediaError::ConnectionLimitExceededException`.
    pub fn is_connection_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::ConnectionLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `GetMediaError::InvalidArgumentException`.
    pub fn is_invalid_argument_exception(&self) -> bool {
        matches!(self, Self::InvalidArgumentException(_))
    }
    /// Returns `true` if the error kind is `GetMediaError::InvalidEndpointException`.
    pub fn is_invalid_endpoint_exception(&self) -> bool {
        matches!(self, Self::InvalidEndpointException(_))
    }
    /// Returns `true` if the error kind is `GetMediaError::NotAuthorizedException`.
    pub fn is_not_authorized_exception(&self) -> bool {
        matches!(self, Self::NotAuthorizedException(_))
    }
    /// Returns `true` if the error kind is `GetMediaError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
}
impl ::std::error::Error for GetMediaError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::ClientLimitExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::ConnectionLimitExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidArgumentException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidEndpointException(_inner) => ::std::option::Option::Some(_inner),
            Self::NotAuthorizedException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::get_media::_get_media_output::GetMediaOutput;

pub use crate::operation::get_media::_get_media_input::GetMediaInput;

mod _get_media_input;

mod _get_media_output;

/// Builders
pub mod builders;
