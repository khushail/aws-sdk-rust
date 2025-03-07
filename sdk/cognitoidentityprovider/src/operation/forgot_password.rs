// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl ForgotPasswordInput {
    /// Consumes the builder and constructs an Operation<[`ForgotPassword`](crate::operation::forgot_password::ForgotPassword)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::forgot_password::ForgotPassword,
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
                _input: &crate::operation::forgot_password::ForgotPasswordInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::forgot_password::ForgotPasswordInput,
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
                "AWSCognitoIdentityProviderService.ForgotPassword",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_forgot_password::ser_forgot_password_input(&self)?,
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
        signing_config.signing_requirements = ::aws_sig_auth::signer::SigningRequirements::Disabled;
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
            crate::operation::forgot_password::ForgotPassword::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "ForgotPassword",
            "cognitoidentityprovider",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `ForgotPassword`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct ForgotPassword;
impl ForgotPassword {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for ForgotPassword {
    type Output = ::std::result::Result<
        crate::operation::forgot_password::ForgotPasswordOutput,
        crate::operation::forgot_password::ForgotPasswordError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_forgot_password::de_forgot_password_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_forgot_password::de_forgot_password_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type ForgotPasswordErrorKind = ForgotPasswordError;
/// Error type for the `ForgotPasswordError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum ForgotPasswordError {
    /// <p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailureException(crate::types::error::CodeDeliveryFailureException),
    /// <p>This exception is thrown when WAF doesn't allow your request based on a web ACL that's associated with your user pool.</p>
    ForbiddenException(crate::types::error::ForbiddenException),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalErrorException(crate::types::error::InternalErrorException),
    /// <p>This exception is thrown when Amazon Cognito isn't allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicyException(
        crate::types::error::InvalidEmailRoleAccessPolicyException,
    ),
    /// <p>This exception is thrown when Amazon Cognito encounters an invalid Lambda response.</p>
    InvalidLambdaResponseException(crate::types::error::InvalidLambdaResponseException),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameterException(crate::types::error::InvalidParameterException),
    /// <p>This exception is returned when the role provided for SMS configuration doesn't have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicyException(crate::types::error::InvalidSmsRoleAccessPolicyException),
    /// <p>This exception is thrown when the trust relationship is not valid for the role provided for SMS configuration. This can happen if you don't trust <code>cognito-idp.amazonaws.com</code> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationshipException(
        crate::types::error::InvalidSmsRoleTrustRelationshipException,
    ),
    /// <p>This exception is thrown when a user exceeds the limit for a requested Amazon Web Services resource.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// <p>This exception is thrown when a user isn't authorized.</p>
    NotAuthorizedException(crate::types::error::NotAuthorizedException),
    /// <p>This exception is thrown when the Amazon Cognito service can't find the requested resource.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequestsException(crate::types::error::TooManyRequestsException),
    /// <p>This exception is thrown when Amazon Cognito encounters an unexpected exception with Lambda.</p>
    UnexpectedLambdaException(crate::types::error::UnexpectedLambdaException),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the Lambda service.</p>
    UserLambdaValidationException(crate::types::error::UserLambdaValidationException),
    /// <p>This exception is thrown when a user isn't found.</p>
    UserNotFoundException(crate::types::error::UserNotFoundException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for ForgotPasswordError {
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
impl ::std::fmt::Display for ForgotPasswordError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::CodeDeliveryFailureException(_inner) => _inner.fmt(f),
            Self::ForbiddenException(_inner) => _inner.fmt(f),
            Self::InternalErrorException(_inner) => _inner.fmt(f),
            Self::InvalidEmailRoleAccessPolicyException(_inner) => _inner.fmt(f),
            Self::InvalidLambdaResponseException(_inner) => _inner.fmt(f),
            Self::InvalidParameterException(_inner) => _inner.fmt(f),
            Self::InvalidSmsRoleAccessPolicyException(_inner) => _inner.fmt(f),
            Self::InvalidSmsRoleTrustRelationshipException(_inner) => _inner.fmt(f),
            Self::LimitExceededException(_inner) => _inner.fmt(f),
            Self::NotAuthorizedException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::TooManyRequestsException(_inner) => _inner.fmt(f),
            Self::UnexpectedLambdaException(_inner) => _inner.fmt(f),
            Self::UserLambdaValidationException(_inner) => _inner.fmt(f),
            Self::UserNotFoundException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ForgotPasswordError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::CodeDeliveryFailureException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ForbiddenException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalErrorException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidEmailRoleAccessPolicyException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidLambdaResponseException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidParameterException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidSmsRoleAccessPolicyException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidSmsRoleTrustRelationshipException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::LimitExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NotAuthorizedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TooManyRequestsException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnexpectedLambdaException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UserLambdaValidationException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UserNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::forgot_password::ForgotPasswordError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for ForgotPasswordError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ForgotPasswordError {
    /// Creates the `ForgotPasswordError::Unhandled` variant from any error type.
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

    /// Creates the `ForgotPasswordError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::CodeDeliveryFailureException(e) => e.meta(),
            Self::ForbiddenException(e) => e.meta(),
            Self::InternalErrorException(e) => e.meta(),
            Self::InvalidEmailRoleAccessPolicyException(e) => e.meta(),
            Self::InvalidLambdaResponseException(e) => e.meta(),
            Self::InvalidParameterException(e) => e.meta(),
            Self::InvalidSmsRoleAccessPolicyException(e) => e.meta(),
            Self::InvalidSmsRoleTrustRelationshipException(e) => e.meta(),
            Self::LimitExceededException(e) => e.meta(),
            Self::NotAuthorizedException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::TooManyRequestsException(e) => e.meta(),
            Self::UnexpectedLambdaException(e) => e.meta(),
            Self::UserLambdaValidationException(e) => e.meta(),
            Self::UserNotFoundException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::CodeDeliveryFailureException`.
    pub fn is_code_delivery_failure_exception(&self) -> bool {
        matches!(self, Self::CodeDeliveryFailureException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::ForbiddenException`.
    pub fn is_forbidden_exception(&self) -> bool {
        matches!(self, Self::ForbiddenException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::InternalErrorException`.
    pub fn is_internal_error_exception(&self) -> bool {
        matches!(self, Self::InternalErrorException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::InvalidEmailRoleAccessPolicyException`.
    pub fn is_invalid_email_role_access_policy_exception(&self) -> bool {
        matches!(self, Self::InvalidEmailRoleAccessPolicyException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::InvalidLambdaResponseException`.
    pub fn is_invalid_lambda_response_exception(&self) -> bool {
        matches!(self, Self::InvalidLambdaResponseException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::InvalidParameterException`.
    pub fn is_invalid_parameter_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::InvalidSmsRoleAccessPolicyException`.
    pub fn is_invalid_sms_role_access_policy_exception(&self) -> bool {
        matches!(self, Self::InvalidSmsRoleAccessPolicyException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::InvalidSmsRoleTrustRelationshipException`.
    pub fn is_invalid_sms_role_trust_relationship_exception(&self) -> bool {
        matches!(self, Self::InvalidSmsRoleTrustRelationshipException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::LimitExceededException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::NotAuthorizedException`.
    pub fn is_not_authorized_exception(&self) -> bool {
        matches!(self, Self::NotAuthorizedException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::TooManyRequestsException`.
    pub fn is_too_many_requests_exception(&self) -> bool {
        matches!(self, Self::TooManyRequestsException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::UnexpectedLambdaException`.
    pub fn is_unexpected_lambda_exception(&self) -> bool {
        matches!(self, Self::UnexpectedLambdaException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::UserLambdaValidationException`.
    pub fn is_user_lambda_validation_exception(&self) -> bool {
        matches!(self, Self::UserLambdaValidationException(_))
    }
    /// Returns `true` if the error kind is `ForgotPasswordError::UserNotFoundException`.
    pub fn is_user_not_found_exception(&self) -> bool {
        matches!(self, Self::UserNotFoundException(_))
    }
}
impl ::std::error::Error for ForgotPasswordError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::CodeDeliveryFailureException(_inner) => ::std::option::Option::Some(_inner),
            Self::ForbiddenException(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalErrorException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidEmailRoleAccessPolicyException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::InvalidLambdaResponseException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidParameterException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidSmsRoleAccessPolicyException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::InvalidSmsRoleTrustRelationshipException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::LimitExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::NotAuthorizedException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::TooManyRequestsException(_inner) => ::std::option::Option::Some(_inner),
            Self::UnexpectedLambdaException(_inner) => ::std::option::Option::Some(_inner),
            Self::UserLambdaValidationException(_inner) => ::std::option::Option::Some(_inner),
            Self::UserNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::forgot_password::_forgot_password_output::ForgotPasswordOutput;

pub use crate::operation::forgot_password::_forgot_password_input::ForgotPasswordInput;

mod _forgot_password_input;

mod _forgot_password_output;

/// Builders
pub mod builders;
