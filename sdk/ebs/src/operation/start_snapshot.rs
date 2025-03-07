// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl StartSnapshotInput {
    /// Consumes the builder and constructs an Operation<[`StartSnapshot`](crate::operation::start_snapshot::StartSnapshot)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::start_snapshot::StartSnapshot,
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
        if self.client_token.is_none() {
            self.client_token =
                ::std::option::Option::Some(_config.make_token.make_idempotency_token());
        }
        let mut request = {
            fn uri_base(
                _input: &crate::operation::start_snapshot::StartSnapshotInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/snapshots").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::start_snapshot::StartSnapshotInput,
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
            crate::protocol_serde::shape_start_snapshot::ser_start_snapshot_input(&self)?,
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
            crate::operation::start_snapshot::StartSnapshot::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "StartSnapshot",
            "ebs",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `StartSnapshot`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct StartSnapshot;
impl StartSnapshot {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for StartSnapshot {
    type Output = ::std::result::Result<
        crate::operation::start_snapshot::StartSnapshotOutput,
        crate::operation::start_snapshot::StartSnapshotError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 201 {
            crate::protocol_serde::shape_start_snapshot::de_start_snapshot_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_start_snapshot::de_start_snapshot_http_response_with_props(
                status, headers, body,
            )
        }
    }
    fn sensitive(&self) -> bool {
        true
    }
}
#[allow(unreachable_code, unused_variables)]
#[cfg(test)]
mod start_snapshot_request_test {
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: LowercaseMessage
    #[::tokio::test]
    #[allow(unused_mut)]
    async fn lowercase_message_response() {
        let expected_output = crate::types::error::ValidationException::builder()
            .set_message(::std::option::Option::Some(
                "1 validation error detected".to_owned(),
            ))
            .build();
        let http_response = ::http::response::Builder::new()
            .header("content-length", "77")
            .header("content-type", "application/json")
            .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
            .header(
                "x-amzn-errortype",
                "ValidationException:http://internal.amazon.com/coral/com.amazon.coral.validate/",
            )
            .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
            .status(400)
            .body(::aws_smithy_http::body::SdkBody::from(
                "{\n  \"message\": \"1 validation error detected\"\n}\n",
            ))
            .unwrap();
        let mut op_response = ::aws_smithy_http::operation::Response::new(http_response);
        use ::aws_smithy_http::response::ParseHttpResponse;
        let parser = crate::operation::start_snapshot::StartSnapshot::new();
        let parsed = parser.parse_unloaded(&mut op_response);
        let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|::bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::start_snapshot::StartSnapshot as ::aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::operation::start_snapshot::StartSnapshotError::ValidationException(parsed) =
            parsed
        {
            ::pretty_assertions::assert_eq!(
                parsed.message,
                expected_output.message,
                "Unexpected value for `message`"
            );
            ::pretty_assertions::assert_eq!(
                parsed.reason,
                expected_output.reason,
                "Unexpected value for `reason`"
            );
        } else {
            panic!(
                "wrong variant: Got: {:?}. Expected: {:?}",
                parsed, expected_output
            );
        }
    }
    /// This test case validates case insensitive parsing of `message`
    /// Test ID: UppercaseMessage
    #[::tokio::test]
    #[allow(unused_mut)]
    async fn uppercase_message_response() {
        let expected_output = crate::types::error::ValidationException::builder()
            .set_message(::std::option::Option::Some(
                "Invalid volume size: 99999999999".to_owned(),
            ))
            .set_reason(::std::option::Option::Some(
                crate::types::ValidationExceptionReason::from("INVALID_VOLUME_SIZE"),
            ))
            .build();
        let http_response = ::http::response::Builder::new()
        .header("content-length", "77")
        .header("content-type", "application/json")
        .header("date", "Wed, 30 Jun 2021 23:42:27 GMT")
        .header("x-amzn-errortype", "ValidationException:http://internal.amazon.com/coral/com.amazon.zeppelindataservice/")
        .header("x-amzn-requestid", "2af8f013-250a-4f6e-88ae-6dd7f6e12807")
        .status(400)
                    .body(::aws_smithy_http::body::SdkBody::from("{\"Message\":\"Invalid volume size: 99999999999\",\"Reason\":\"INVALID_VOLUME_SIZE\"}\n"))
                    .unwrap();
        let mut op_response = ::aws_smithy_http::operation::Response::new(http_response);
        use ::aws_smithy_http::response::ParseHttpResponse;
        let parser = crate::operation::start_snapshot::StartSnapshot::new();
        let parsed = parser.parse_unloaded(&mut op_response);
        let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|::bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::start_snapshot::StartSnapshot as ::aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.expect_err("should be error response");
        if let crate::operation::start_snapshot::StartSnapshotError::ValidationException(parsed) =
            parsed
        {
            ::pretty_assertions::assert_eq!(
                parsed.message,
                expected_output.message,
                "Unexpected value for `message`"
            );
            ::pretty_assertions::assert_eq!(
                parsed.reason,
                expected_output.reason,
                "Unexpected value for `reason`"
            );
        } else {
            panic!(
                "wrong variant: Got: {:?}. Expected: {:?}",
                parsed, expected_output
            );
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type StartSnapshotErrorKind = StartSnapshotError;
/// Error type for the `StartSnapshotError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum StartSnapshotError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>You have reached the limit for concurrent API requests. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-accessing-snapshot.html#ebsapi-performance">Optimizing performance of the EBS direct APIs</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    ConcurrentLimitExceededException(crate::types::error::ConcurrentLimitExceededException),
    /// <p>The request uses the same client token as a previous, but non-identical request.</p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>An internal error has occurred.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>The number of API requests has exceed the maximum allowed API request throttling limit.</p>
    RequestThrottledException(crate::types::error::RequestThrottledException),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>Your current service quotas do not allow you to perform this action.</p>
    ServiceQuotaExceededException(crate::types::error::ServiceQuotaExceededException),
    /// <p>The input fails to satisfy the constraints of the EBS direct APIs.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for StartSnapshotError {
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
impl ::std::fmt::Display for StartSnapshotError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::AccessDeniedException(_inner) => _inner.fmt(f),
            Self::ConcurrentLimitExceededException(_inner) => _inner.fmt(f),
            Self::ConflictException(_inner) => _inner.fmt(f),
            Self::InternalServerException(_inner) => _inner.fmt(f),
            Self::RequestThrottledException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::ServiceQuotaExceededException(_inner) => _inner.fmt(f),
            Self::ValidationException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for StartSnapshotError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AccessDeniedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ConcurrentLimitExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ConflictException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalServerException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::RequestThrottledException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ServiceQuotaExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ValidationException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::start_snapshot::StartSnapshotError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for StartSnapshotError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl StartSnapshotError {
    /// Creates the `StartSnapshotError::Unhandled` variant from any error type.
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

    /// Creates the `StartSnapshotError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::AccessDeniedException(e) => e.meta(),
            Self::ConcurrentLimitExceededException(e) => e.meta(),
            Self::ConflictException(e) => e.meta(),
            Self::InternalServerException(e) => e.meta(),
            Self::RequestThrottledException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::ServiceQuotaExceededException(e) => e.meta(),
            Self::ValidationException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `StartSnapshotError::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(self, Self::AccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `StartSnapshotError::ConcurrentLimitExceededException`.
    pub fn is_concurrent_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::ConcurrentLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `StartSnapshotError::ConflictException`.
    pub fn is_conflict_exception(&self) -> bool {
        matches!(self, Self::ConflictException(_))
    }
    /// Returns `true` if the error kind is `StartSnapshotError::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(self, Self::InternalServerException(_))
    }
    /// Returns `true` if the error kind is `StartSnapshotError::RequestThrottledException`.
    pub fn is_request_throttled_exception(&self) -> bool {
        matches!(self, Self::RequestThrottledException(_))
    }
    /// Returns `true` if the error kind is `StartSnapshotError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `StartSnapshotError::ServiceQuotaExceededException`.
    pub fn is_service_quota_exceeded_exception(&self) -> bool {
        matches!(self, Self::ServiceQuotaExceededException(_))
    }
    /// Returns `true` if the error kind is `StartSnapshotError::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(self, Self::ValidationException(_))
    }
}
impl ::std::error::Error for StartSnapshotError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::AccessDeniedException(_inner) => ::std::option::Option::Some(_inner),
            Self::ConcurrentLimitExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::ConflictException(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalServerException(_inner) => ::std::option::Option::Some(_inner),
            Self::RequestThrottledException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::ServiceQuotaExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::ValidationException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::start_snapshot::_start_snapshot_output::StartSnapshotOutput;

pub use crate::operation::start_snapshot::_start_snapshot_input::StartSnapshotInput;

mod _start_snapshot_input;

mod _start_snapshot_output;

/// Builders
pub mod builders;
