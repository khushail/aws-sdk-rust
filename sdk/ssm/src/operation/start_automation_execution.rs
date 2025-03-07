// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl StartAutomationExecutionInput {
    /// Consumes the builder and constructs an Operation<[`StartAutomationExecution`](crate::operation::start_automation_execution::StartAutomationExecution)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::start_automation_execution::StartAutomationExecution,
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
                _input: &crate::operation::start_automation_execution::StartAutomationExecutionInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::start_automation_execution::StartAutomationExecutionInput,
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
                "AmazonSSM.StartAutomationExecution",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_start_automation_execution::ser_start_automation_execution_input(&self)?
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
            crate::operation::start_automation_execution::StartAutomationExecution::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "StartAutomationExecution",
            "ssm",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `StartAutomationExecution`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct StartAutomationExecution;
impl StartAutomationExecution {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for StartAutomationExecution {
    type Output = ::std::result::Result<
        crate::operation::start_automation_execution::StartAutomationExecutionOutput,
        crate::operation::start_automation_execution::StartAutomationExecutionError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_start_automation_execution::de_start_automation_execution_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_start_automation_execution::de_start_automation_execution_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type StartAutomationExecutionErrorKind = StartAutomationExecutionError;
/// Error type for the `StartAutomationExecutionError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum StartAutomationExecutionError {
    /// <p>An Automation runbook with the specified name couldn't be found.</p>
    AutomationDefinitionNotFoundException(
        crate::types::error::AutomationDefinitionNotFoundException,
    ),
    /// <p>An Automation runbook with the specified name and version couldn't be found.</p>
    AutomationDefinitionVersionNotFoundException(
        crate::types::error::AutomationDefinitionVersionNotFoundException,
    ),
    /// <p>The number of simultaneously running Automation executions exceeded the allowable limit.</p>
    AutomationExecutionLimitExceededException(
        crate::types::error::AutomationExecutionLimitExceededException,
    ),
    /// <p>Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. </p>
    IdempotentParameterMismatch(crate::types::error::IdempotentParameterMismatch),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::types::error::InternalServerError),
    /// <p>The supplied parameters for invoking the specified Automation runbook are incorrect. For example, they may not match the set of parameters permitted for the specified Automation document.</p>
    InvalidAutomationExecutionParametersException(
        crate::types::error::InvalidAutomationExecutionParametersException,
    ),
    /// <p>The target isn't valid or doesn't exist. It might not be configured for Systems Manager or you might not have permission to perform the operation.</p>
    InvalidTarget(crate::types::error::InvalidTarget),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for StartAutomationExecutionError {
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
impl ::std::fmt::Display for StartAutomationExecutionError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::AutomationDefinitionNotFoundException(_inner) => _inner.fmt(f),
            Self::AutomationDefinitionVersionNotFoundException(_inner) => _inner.fmt(f),
            Self::AutomationExecutionLimitExceededException(_inner) => _inner.fmt(f),
            Self::IdempotentParameterMismatch(_inner) => _inner.fmt(f),
            Self::InternalServerError(_inner) => _inner.fmt(f),
            Self::InvalidAutomationExecutionParametersException(_inner) => _inner.fmt(f),
            Self::InvalidTarget(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for StartAutomationExecutionError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AutomationDefinitionNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::AutomationDefinitionVersionNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::AutomationExecutionLimitExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::IdempotentParameterMismatch(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalServerError(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidAutomationExecutionParametersException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidTarget(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::start_automation_execution::StartAutomationExecutionError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for StartAutomationExecutionError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl StartAutomationExecutionError {
    /// Creates the `StartAutomationExecutionError::Unhandled` variant from any error type.
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

    /// Creates the `StartAutomationExecutionError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::AutomationDefinitionNotFoundException(e) => e.meta(),
            Self::AutomationDefinitionVersionNotFoundException(e) => e.meta(),
            Self::AutomationExecutionLimitExceededException(e) => e.meta(),
            Self::IdempotentParameterMismatch(e) => e.meta(),
            Self::InternalServerError(e) => e.meta(),
            Self::InvalidAutomationExecutionParametersException(e) => e.meta(),
            Self::InvalidTarget(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `StartAutomationExecutionError::AutomationDefinitionNotFoundException`.
    pub fn is_automation_definition_not_found_exception(&self) -> bool {
        matches!(self, Self::AutomationDefinitionNotFoundException(_))
    }
    /// Returns `true` if the error kind is `StartAutomationExecutionError::AutomationDefinitionVersionNotFoundException`.
    pub fn is_automation_definition_version_not_found_exception(&self) -> bool {
        matches!(self, Self::AutomationDefinitionVersionNotFoundException(_))
    }
    /// Returns `true` if the error kind is `StartAutomationExecutionError::AutomationExecutionLimitExceededException`.
    pub fn is_automation_execution_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::AutomationExecutionLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `StartAutomationExecutionError::IdempotentParameterMismatch`.
    pub fn is_idempotent_parameter_mismatch(&self) -> bool {
        matches!(self, Self::IdempotentParameterMismatch(_))
    }
    /// Returns `true` if the error kind is `StartAutomationExecutionError::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(self, Self::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `StartAutomationExecutionError::InvalidAutomationExecutionParametersException`.
    pub fn is_invalid_automation_execution_parameters_exception(&self) -> bool {
        matches!(self, Self::InvalidAutomationExecutionParametersException(_))
    }
    /// Returns `true` if the error kind is `StartAutomationExecutionError::InvalidTarget`.
    pub fn is_invalid_target(&self) -> bool {
        matches!(self, Self::InvalidTarget(_))
    }
}
impl ::std::error::Error for StartAutomationExecutionError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::AutomationDefinitionNotFoundException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::AutomationDefinitionVersionNotFoundException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::AutomationExecutionLimitExceededException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::IdempotentParameterMismatch(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalServerError(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidAutomationExecutionParametersException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::InvalidTarget(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::start_automation_execution::_start_automation_execution_output::StartAutomationExecutionOutput;

pub use crate::operation::start_automation_execution::_start_automation_execution_input::StartAutomationExecutionInput;

mod _start_automation_execution_input;

mod _start_automation_execution_output;

/// Builders
pub mod builders;
