// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl GetRecordInput {
    /// Consumes the builder and constructs an Operation<[`GetRecord`](crate::operation::get_record::GetRecord)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::get_record::GetRecord,
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
                _input: &crate::operation::get_record::GetRecordInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                let input_1 = &_input.feature_group_name;
                let input_1 = input_1.as_ref().ok_or_else(|| {
                    ::aws_smithy_http::operation::error::BuildError::missing_field(
                        "feature_group_name",
                        "cannot be empty or unset",
                    )
                })?;
                let feature_group_name = ::aws_smithy_http::label::fmt_string(
                    input_1,
                    ::aws_smithy_http::label::EncodingStrategy::Default,
                );
                if feature_group_name.is_empty() {
                    return ::std::result::Result::Err(
                        ::aws_smithy_http::operation::error::BuildError::missing_field(
                            "feature_group_name",
                            "cannot be empty or unset",
                        ),
                    );
                }
                ::std::write!(
                    output,
                    "/FeatureGroup/{FeatureGroupName}",
                    FeatureGroupName = feature_group_name
                )
                .expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            fn uri_query(
                _input: &crate::operation::get_record::GetRecordInput,
                mut output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                let mut query = ::aws_smithy_http::query::Writer::new(output);
                let inner_2 = &_input.record_identifier_value_as_string;
                let inner_2 = inner_2.as_ref().ok_or_else(|| {
                    ::aws_smithy_http::operation::error::BuildError::missing_field(
                        "record_identifier_value_as_string",
                        "cannot be empty or unset",
                    )
                })?;
                if inner_2.is_empty() {
                    return ::std::result::Result::Err(
                        ::aws_smithy_http::operation::error::BuildError::missing_field(
                            "record_identifier_value_as_string",
                            "cannot be empty or unset",
                        ),
                    );
                }
                query.push_kv(
                    "RecordIdentifierValueAsString",
                    &::aws_smithy_http::query::fmt_string(&inner_2),
                );
                if let ::std::option::Option::Some(inner_3) = &_input.feature_names {
                    {
                        for inner_4 in inner_3 {
                            query.push_kv(
                                "FeatureName",
                                &::aws_smithy_http::query::fmt_string(&inner_4),
                            );
                        }
                    }
                }
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::get_record::GetRecordInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<
                ::http::request::Builder,
                ::aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, ::http::request::Builder::new())?;
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from("");
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
            crate::operation::get_record::GetRecord::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "GetRecord",
            "sagemakerfeaturestoreruntime",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `GetRecord`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct GetRecord;
impl GetRecord {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for GetRecord {
    type Output = ::std::result::Result<
        crate::operation::get_record::GetRecordOutput,
        crate::operation::get_record::GetRecordError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_get_record::de_get_record_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_get_record::de_get_record_http_response_with_props(
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
pub type GetRecordErrorKind = GetRecordError;
/// Error type for the `GetRecordError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum GetRecordError {
    /// <p>You do not have permission to perform an action.</p>
    AccessForbidden(crate::types::error::AccessForbidden),
    /// <p>An internal failure occurred. Try your request again. If the problem persists, contact Amazon Web Services customer support.</p>
    InternalFailure(crate::types::error::InternalFailure),
    /// <p>A resource that is required to perform an action was not found.</p>
    ResourceNotFound(crate::types::error::ResourceNotFound),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(crate::types::error::ServiceUnavailable),
    /// <p>There was an error validating your request.</p>
    ValidationError(crate::types::error::ValidationError),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for GetRecordError {
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
impl ::std::fmt::Display for GetRecordError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::AccessForbidden(_inner) => _inner.fmt(f),
            Self::InternalFailure(_inner) => _inner.fmt(f),
            Self::ResourceNotFound(_inner) => _inner.fmt(f),
            Self::ServiceUnavailable(_inner) => _inner.fmt(f),
            Self::ValidationError(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetRecordError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AccessForbidden(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalFailure(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceNotFound(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ServiceUnavailable(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ValidationError(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::get_record::GetRecordError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for GetRecordError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl GetRecordError {
    /// Creates the `GetRecordError::Unhandled` variant from any error type.
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

    /// Creates the `GetRecordError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::AccessForbidden(e) => e.meta(),
            Self::InternalFailure(e) => e.meta(),
            Self::ResourceNotFound(e) => e.meta(),
            Self::ServiceUnavailable(e) => e.meta(),
            Self::ValidationError(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `GetRecordError::AccessForbidden`.
    pub fn is_access_forbidden(&self) -> bool {
        matches!(self, Self::AccessForbidden(_))
    }
    /// Returns `true` if the error kind is `GetRecordError::InternalFailure`.
    pub fn is_internal_failure(&self) -> bool {
        matches!(self, Self::InternalFailure(_))
    }
    /// Returns `true` if the error kind is `GetRecordError::ResourceNotFound`.
    pub fn is_resource_not_found(&self) -> bool {
        matches!(self, Self::ResourceNotFound(_))
    }
    /// Returns `true` if the error kind is `GetRecordError::ServiceUnavailable`.
    pub fn is_service_unavailable(&self) -> bool {
        matches!(self, Self::ServiceUnavailable(_))
    }
    /// Returns `true` if the error kind is `GetRecordError::ValidationError`.
    pub fn is_validation_error(&self) -> bool {
        matches!(self, Self::ValidationError(_))
    }
}
impl ::std::error::Error for GetRecordError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::AccessForbidden(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalFailure(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFound(_inner) => ::std::option::Option::Some(_inner),
            Self::ServiceUnavailable(_inner) => ::std::option::Option::Some(_inner),
            Self::ValidationError(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::get_record::_get_record_output::GetRecordOutput;

pub use crate::operation::get_record::_get_record_input::GetRecordInput;

mod _get_record_input;

mod _get_record_output;

/// Builders
pub mod builders;
