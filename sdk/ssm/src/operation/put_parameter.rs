// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl PutParameterInput {
    /// Consumes the builder and constructs an Operation<[`PutParameter`](crate::operation::put_parameter::PutParameter)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::put_parameter::PutParameter,
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
                _input: &crate::operation::put_parameter::PutParameterInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::put_parameter::PutParameterInput,
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
                "AmazonSSM.PutParameter",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_put_parameter::ser_put_parameter_input(&self)?,
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
            crate::operation::put_parameter::PutParameter::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "PutParameter",
            "ssm",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `PutParameter`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct PutParameter;
impl PutParameter {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for PutParameter {
    type Output = ::std::result::Result<
        crate::operation::put_parameter::PutParameterOutput,
        crate::operation::put_parameter::PutParameterError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_put_parameter::de_put_parameter_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_put_parameter::de_put_parameter_http_response_with_props(
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
pub type PutParameterErrorKind = PutParameterError;
/// Error type for the `PutParameterError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum PutParameterError {
    /// <p>A hierarchy can have a maximum of 15 levels. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-parameter-name-constraints.html">Requirements and constraints for parameter names</a> in the <i>Amazon Web Services Systems Manager User Guide</i>. </p>
    HierarchyLevelLimitExceededException(crate::types::error::HierarchyLevelLimitExceededException),
    /// <p>Parameter Store doesn't support changing a parameter type in a hierarchy. For example, you can't change a parameter from a <code>String</code> type to a <code>SecureString</code> type. You must create a new, unique parameter.</p>
    HierarchyTypeMismatchException(crate::types::error::HierarchyTypeMismatchException),
    /// <p>There is a conflict in the policies specified for this parameter. You can't, for example, specify two Expiration policies for a parameter. Review your policies, and try again.</p>
    IncompatiblePolicyException(crate::types::error::IncompatiblePolicyException),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::types::error::InternalServerError),
    /// <p>The request doesn't meet the regular expression requirement.</p>
    InvalidAllowedPatternException(crate::types::error::InvalidAllowedPatternException),
    /// <p>The query key ID isn't valid.</p>
    InvalidKeyId(crate::types::error::InvalidKeyId),
    /// <p>A policy attribute or its value is invalid. </p>
    InvalidPolicyAttributeException(crate::types::error::InvalidPolicyAttributeException),
    /// <p>The policy type isn't supported. Parameter Store supports the following policy types: Expiration, ExpirationNotification, and NoChangeNotification.</p>
    InvalidPolicyTypeException(crate::types::error::InvalidPolicyTypeException),
    /// <p>The parameter already exists. You can't create duplicate parameters.</p>
    ParameterAlreadyExists(crate::types::error::ParameterAlreadyExists),
    /// <p>You have exceeded the number of parameters for this Amazon Web Services account. Delete one or more parameters and try again.</p>
    ParameterLimitExceeded(crate::types::error::ParameterLimitExceeded),
    /// <p>Parameter Store retains the 100 most recently created versions of a parameter. After this number of versions has been created, Parameter Store deletes the oldest version when a new one is created. However, if the oldest version has a <i>label</i> attached to it, Parameter Store won't delete the version and instead presents this error message:</p>
    /// <p> <code>An error occurred (ParameterMaxVersionLimitExceeded) when calling the PutParameter operation: You attempted to create a new version of <i>parameter-name</i> by calling the PutParameter API with the overwrite flag. Version <i>version-number</i>, the oldest version, can't be deleted because it has a label associated with it. Move the label to another version of the parameter, and try again.</code> </p>
    /// <p>This safeguard is to prevent parameter versions with mission critical labels assigned to them from being deleted. To continue creating new parameters, first move the label from the oldest version of the parameter to a newer one for use in your operations. For information about moving parameter labels, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-labels.html#sysman-paramstore-labels-console-move">Move a parameter label (console)</a> or <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-labels.html#sysman-paramstore-labels-cli-move">Move a parameter label (CLI)</a> in the <i>Amazon Web Services Systems Manager User Guide</i>. </p>
    ParameterMaxVersionLimitExceeded(crate::types::error::ParameterMaxVersionLimitExceeded),
    /// <p>The parameter name isn't valid.</p>
    ParameterPatternMismatchException(crate::types::error::ParameterPatternMismatchException),
    /// <p>You specified more than the maximum number of allowed policies for the parameter. The maximum is 10.</p>
    PoliciesLimitExceededException(crate::types::error::PoliciesLimitExceededException),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(crate::types::error::TooManyUpdates),
    /// <p>The parameter type isn't supported.</p>
    UnsupportedParameterType(crate::types::error::UnsupportedParameterType),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for PutParameterError {
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
impl ::std::fmt::Display for PutParameterError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::HierarchyLevelLimitExceededException(_inner) => _inner.fmt(f),
            Self::HierarchyTypeMismatchException(_inner) => _inner.fmt(f),
            Self::IncompatiblePolicyException(_inner) => _inner.fmt(f),
            Self::InternalServerError(_inner) => _inner.fmt(f),
            Self::InvalidAllowedPatternException(_inner) => _inner.fmt(f),
            Self::InvalidKeyId(_inner) => _inner.fmt(f),
            Self::InvalidPolicyAttributeException(_inner) => _inner.fmt(f),
            Self::InvalidPolicyTypeException(_inner) => _inner.fmt(f),
            Self::ParameterAlreadyExists(_inner) => _inner.fmt(f),
            Self::ParameterLimitExceeded(_inner) => _inner.fmt(f),
            Self::ParameterMaxVersionLimitExceeded(_inner) => _inner.fmt(f),
            Self::ParameterPatternMismatchException(_inner) => _inner.fmt(f),
            Self::PoliciesLimitExceededException(_inner) => _inner.fmt(f),
            Self::TooManyUpdates(_inner) => _inner.fmt(f),
            Self::UnsupportedParameterType(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for PutParameterError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::HierarchyLevelLimitExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::HierarchyTypeMismatchException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::IncompatiblePolicyException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalServerError(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidAllowedPatternException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidKeyId(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidPolicyAttributeException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidPolicyTypeException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ParameterAlreadyExists(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ParameterLimitExceeded(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ParameterMaxVersionLimitExceeded(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ParameterPatternMismatchException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::PoliciesLimitExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TooManyUpdates(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnsupportedParameterType(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::put_parameter::PutParameterError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for PutParameterError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl PutParameterError {
    /// Creates the `PutParameterError::Unhandled` variant from any error type.
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

    /// Creates the `PutParameterError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::HierarchyLevelLimitExceededException(e) => e.meta(),
            Self::HierarchyTypeMismatchException(e) => e.meta(),
            Self::IncompatiblePolicyException(e) => e.meta(),
            Self::InternalServerError(e) => e.meta(),
            Self::InvalidAllowedPatternException(e) => e.meta(),
            Self::InvalidKeyId(e) => e.meta(),
            Self::InvalidPolicyAttributeException(e) => e.meta(),
            Self::InvalidPolicyTypeException(e) => e.meta(),
            Self::ParameterAlreadyExists(e) => e.meta(),
            Self::ParameterLimitExceeded(e) => e.meta(),
            Self::ParameterMaxVersionLimitExceeded(e) => e.meta(),
            Self::ParameterPatternMismatchException(e) => e.meta(),
            Self::PoliciesLimitExceededException(e) => e.meta(),
            Self::TooManyUpdates(e) => e.meta(),
            Self::UnsupportedParameterType(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `PutParameterError::HierarchyLevelLimitExceededException`.
    pub fn is_hierarchy_level_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::HierarchyLevelLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::HierarchyTypeMismatchException`.
    pub fn is_hierarchy_type_mismatch_exception(&self) -> bool {
        matches!(self, Self::HierarchyTypeMismatchException(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::IncompatiblePolicyException`.
    pub fn is_incompatible_policy_exception(&self) -> bool {
        matches!(self, Self::IncompatiblePolicyException(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(self, Self::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::InvalidAllowedPatternException`.
    pub fn is_invalid_allowed_pattern_exception(&self) -> bool {
        matches!(self, Self::InvalidAllowedPatternException(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::InvalidKeyId`.
    pub fn is_invalid_key_id(&self) -> bool {
        matches!(self, Self::InvalidKeyId(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::InvalidPolicyAttributeException`.
    pub fn is_invalid_policy_attribute_exception(&self) -> bool {
        matches!(self, Self::InvalidPolicyAttributeException(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::InvalidPolicyTypeException`.
    pub fn is_invalid_policy_type_exception(&self) -> bool {
        matches!(self, Self::InvalidPolicyTypeException(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::ParameterAlreadyExists`.
    pub fn is_parameter_already_exists(&self) -> bool {
        matches!(self, Self::ParameterAlreadyExists(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::ParameterLimitExceeded`.
    pub fn is_parameter_limit_exceeded(&self) -> bool {
        matches!(self, Self::ParameterLimitExceeded(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::ParameterMaxVersionLimitExceeded`.
    pub fn is_parameter_max_version_limit_exceeded(&self) -> bool {
        matches!(self, Self::ParameterMaxVersionLimitExceeded(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::ParameterPatternMismatchException`.
    pub fn is_parameter_pattern_mismatch_exception(&self) -> bool {
        matches!(self, Self::ParameterPatternMismatchException(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::PoliciesLimitExceededException`.
    pub fn is_policies_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::PoliciesLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::TooManyUpdates`.
    pub fn is_too_many_updates(&self) -> bool {
        matches!(self, Self::TooManyUpdates(_))
    }
    /// Returns `true` if the error kind is `PutParameterError::UnsupportedParameterType`.
    pub fn is_unsupported_parameter_type(&self) -> bool {
        matches!(self, Self::UnsupportedParameterType(_))
    }
}
impl ::std::error::Error for PutParameterError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::HierarchyLevelLimitExceededException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::HierarchyTypeMismatchException(_inner) => ::std::option::Option::Some(_inner),
            Self::IncompatiblePolicyException(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalServerError(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidAllowedPatternException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidKeyId(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidPolicyAttributeException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidPolicyTypeException(_inner) => ::std::option::Option::Some(_inner),
            Self::ParameterAlreadyExists(_inner) => ::std::option::Option::Some(_inner),
            Self::ParameterLimitExceeded(_inner) => ::std::option::Option::Some(_inner),
            Self::ParameterMaxVersionLimitExceeded(_inner) => ::std::option::Option::Some(_inner),
            Self::ParameterPatternMismatchException(_inner) => ::std::option::Option::Some(_inner),
            Self::PoliciesLimitExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::TooManyUpdates(_inner) => ::std::option::Option::Some(_inner),
            Self::UnsupportedParameterType(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::put_parameter::_put_parameter_output::PutParameterOutput;

pub use crate::operation::put_parameter::_put_parameter_input::PutParameterInput;

mod _put_parameter_input;

mod _put_parameter_output;

/// Builders
pub mod builders;
