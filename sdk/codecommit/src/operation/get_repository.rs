// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl GetRepositoryInput {
    /// Consumes the builder and constructs an Operation<[`GetRepository`](crate::operation::get_repository::GetRepository)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::get_repository::GetRepository,
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
                _input: &crate::operation::get_repository::GetRepositoryInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::get_repository::GetRepositoryInput,
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
                "CodeCommit_20150413.GetRepository",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_get_repository::ser_get_repository_input(&self)?,
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
            crate::operation::get_repository::GetRepository::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "GetRepository",
            "codecommit",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `GetRepository`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct GetRepository;
impl GetRepository {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for GetRepository {
    type Output = ::std::result::Result<
        crate::operation::get_repository::GetRepositoryOutput,
        crate::operation::get_repository::GetRepositoryError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_get_repository::de_get_repository_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_get_repository::de_get_repository_http_response_with_props(
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
pub type GetRepositoryErrorKind = GetRepositoryError;
/// Error type for the `GetRepositoryError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum GetRepositoryError {
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailedException(
        crate::types::error::EncryptionIntegrityChecksFailedException,
    ),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDeniedException(crate::types::error::EncryptionKeyAccessDeniedException),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabledException(crate::types::error::EncryptionKeyDisabledException),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFoundException(crate::types::error::EncryptionKeyNotFoundException),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailableException(crate::types::error::EncryptionKeyUnavailableException),
    /// <p>A specified repository name is not valid.</p> <note>
    /// <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p>
    /// </note>
    InvalidRepositoryNameException(crate::types::error::InvalidRepositoryNameException),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExistException(crate::types::error::RepositoryDoesNotExistException),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequiredException(crate::types::error::RepositoryNameRequiredException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for GetRepositoryError {
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
impl ::std::fmt::Display for GetRepositoryError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::EncryptionIntegrityChecksFailedException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyAccessDeniedException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyDisabledException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyNotFoundException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyUnavailableException(_inner) => _inner.fmt(f),
            Self::InvalidRepositoryNameException(_inner) => _inner.fmt(f),
            Self::RepositoryDoesNotExistException(_inner) => _inner.fmt(f),
            Self::RepositoryNameRequiredException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetRepositoryError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::EncryptionIntegrityChecksFailedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyAccessDeniedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyDisabledException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyUnavailableException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidRepositoryNameException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::RepositoryDoesNotExistException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::RepositoryNameRequiredException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::get_repository::GetRepositoryError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for GetRepositoryError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl GetRepositoryError {
    /// Creates the `GetRepositoryError::Unhandled` variant from any error type.
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

    /// Creates the `GetRepositoryError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::EncryptionIntegrityChecksFailedException(e) => e.meta(),
            Self::EncryptionKeyAccessDeniedException(e) => e.meta(),
            Self::EncryptionKeyDisabledException(e) => e.meta(),
            Self::EncryptionKeyNotFoundException(e) => e.meta(),
            Self::EncryptionKeyUnavailableException(e) => e.meta(),
            Self::InvalidRepositoryNameException(e) => e.meta(),
            Self::RepositoryDoesNotExistException(e) => e.meta(),
            Self::RepositoryNameRequiredException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `GetRepositoryError::EncryptionIntegrityChecksFailedException`.
    pub fn is_encryption_integrity_checks_failed_exception(&self) -> bool {
        matches!(self, Self::EncryptionIntegrityChecksFailedException(_))
    }
    /// Returns `true` if the error kind is `GetRepositoryError::EncryptionKeyAccessDeniedException`.
    pub fn is_encryption_key_access_denied_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyAccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `GetRepositoryError::EncryptionKeyDisabledException`.
    pub fn is_encryption_key_disabled_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyDisabledException(_))
    }
    /// Returns `true` if the error kind is `GetRepositoryError::EncryptionKeyNotFoundException`.
    pub fn is_encryption_key_not_found_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyNotFoundException(_))
    }
    /// Returns `true` if the error kind is `GetRepositoryError::EncryptionKeyUnavailableException`.
    pub fn is_encryption_key_unavailable_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyUnavailableException(_))
    }
    /// Returns `true` if the error kind is `GetRepositoryError::InvalidRepositoryNameException`.
    pub fn is_invalid_repository_name_exception(&self) -> bool {
        matches!(self, Self::InvalidRepositoryNameException(_))
    }
    /// Returns `true` if the error kind is `GetRepositoryError::RepositoryDoesNotExistException`.
    pub fn is_repository_does_not_exist_exception(&self) -> bool {
        matches!(self, Self::RepositoryDoesNotExistException(_))
    }
    /// Returns `true` if the error kind is `GetRepositoryError::RepositoryNameRequiredException`.
    pub fn is_repository_name_required_exception(&self) -> bool {
        matches!(self, Self::RepositoryNameRequiredException(_))
    }
}
impl ::std::error::Error for GetRepositoryError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::EncryptionIntegrityChecksFailedException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::EncryptionKeyAccessDeniedException(_inner) => ::std::option::Option::Some(_inner),
            Self::EncryptionKeyDisabledException(_inner) => ::std::option::Option::Some(_inner),
            Self::EncryptionKeyNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::EncryptionKeyUnavailableException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidRepositoryNameException(_inner) => ::std::option::Option::Some(_inner),
            Self::RepositoryDoesNotExistException(_inner) => ::std::option::Option::Some(_inner),
            Self::RepositoryNameRequiredException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::get_repository::_get_repository_output::GetRepositoryOutput;

pub use crate::operation::get_repository::_get_repository_input::GetRepositoryInput;

mod _get_repository_input;

mod _get_repository_output;

/// Builders
pub mod builders;
