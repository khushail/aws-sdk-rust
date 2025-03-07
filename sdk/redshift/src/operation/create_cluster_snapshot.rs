// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl CreateClusterSnapshotInput {
    /// Consumes the builder and constructs an Operation<[`CreateClusterSnapshot`](crate::operation::create_cluster_snapshot::CreateClusterSnapshot)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::create_cluster_snapshot::CreateClusterSnapshot,
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
                _input: &crate::operation::create_cluster_snapshot::CreateClusterSnapshotInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_cluster_snapshot::CreateClusterSnapshotInput,
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
            crate::protocol_serde::shape_create_cluster_snapshot_input::ser_create_cluster_snapshot_input_input(&self)?
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
            crate::operation::create_cluster_snapshot::CreateClusterSnapshot::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "CreateClusterSnapshot",
            "redshift",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `CreateClusterSnapshot`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct CreateClusterSnapshot;
impl CreateClusterSnapshot {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for CreateClusterSnapshot {
    type Output = ::std::result::Result<
        crate::operation::create_cluster_snapshot::CreateClusterSnapshotOutput,
        crate::operation::create_cluster_snapshot::CreateClusterSnapshotError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_create_cluster_snapshot::de_create_cluster_snapshot_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_create_cluster_snapshot::de_create_cluster_snapshot_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type CreateClusterSnapshotErrorKind = CreateClusterSnapshotError;
/// Error type for the `CreateClusterSnapshotError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum CreateClusterSnapshotError {
    /// <p>The <code>ClusterIdentifier</code> parameter does not refer to an existing cluster. </p>
    ClusterNotFoundFault(crate::types::error::ClusterNotFoundFault),
    /// <p>The value specified as a snapshot identifier is already used by an existing snapshot.</p>
    ClusterSnapshotAlreadyExistsFault(crate::types::error::ClusterSnapshotAlreadyExistsFault),
    /// <p>The request would result in the user exceeding the allowed number of cluster snapshots.</p>
    ClusterSnapshotQuotaExceededFault(crate::types::error::ClusterSnapshotQuotaExceededFault),
    /// <p>The specified cluster is not in the <code>available</code> state. </p>
    InvalidClusterStateFault(crate::types::error::InvalidClusterStateFault),
    /// <p>The retention period specified is either in the past or is not a valid value.</p>
    /// <p>The value must be either -1 or an integer between 1 and 3,653.</p>
    InvalidRetentionPeriodFault(crate::types::error::InvalidRetentionPeriodFault),
    /// <p>The tag is invalid.</p>
    InvalidTagFault(crate::types::error::InvalidTagFault),
    /// <p>You have exceeded the number of tags allowed.</p>
    TagLimitExceededFault(crate::types::error::TagLimitExceededFault),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for CreateClusterSnapshotError {
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
impl ::std::fmt::Display for CreateClusterSnapshotError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::ClusterNotFoundFault(_inner) => _inner.fmt(f),
            Self::ClusterSnapshotAlreadyExistsFault(_inner) => _inner.fmt(f),
            Self::ClusterSnapshotQuotaExceededFault(_inner) => _inner.fmt(f),
            Self::InvalidClusterStateFault(_inner) => _inner.fmt(f),
            Self::InvalidRetentionPeriodFault(_inner) => _inner.fmt(f),
            Self::InvalidTagFault(_inner) => _inner.fmt(f),
            Self::TagLimitExceededFault(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateClusterSnapshotError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ClusterNotFoundFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ClusterSnapshotAlreadyExistsFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ClusterSnapshotQuotaExceededFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidClusterStateFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidRetentionPeriodFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidTagFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TagLimitExceededFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::create_cluster_snapshot::CreateClusterSnapshotError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for CreateClusterSnapshotError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl CreateClusterSnapshotError {
    /// Creates the `CreateClusterSnapshotError::Unhandled` variant from any error type.
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

    /// Creates the `CreateClusterSnapshotError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::ClusterNotFoundFault(e) => e.meta(),
            Self::ClusterSnapshotAlreadyExistsFault(e) => e.meta(),
            Self::ClusterSnapshotQuotaExceededFault(e) => e.meta(),
            Self::InvalidClusterStateFault(e) => e.meta(),
            Self::InvalidRetentionPeriodFault(e) => e.meta(),
            Self::InvalidTagFault(e) => e.meta(),
            Self::TagLimitExceededFault(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `CreateClusterSnapshotError::ClusterNotFoundFault`.
    pub fn is_cluster_not_found_fault(&self) -> bool {
        matches!(self, Self::ClusterNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `CreateClusterSnapshotError::ClusterSnapshotAlreadyExistsFault`.
    pub fn is_cluster_snapshot_already_exists_fault(&self) -> bool {
        matches!(self, Self::ClusterSnapshotAlreadyExistsFault(_))
    }
    /// Returns `true` if the error kind is `CreateClusterSnapshotError::ClusterSnapshotQuotaExceededFault`.
    pub fn is_cluster_snapshot_quota_exceeded_fault(&self) -> bool {
        matches!(self, Self::ClusterSnapshotQuotaExceededFault(_))
    }
    /// Returns `true` if the error kind is `CreateClusterSnapshotError::InvalidClusterStateFault`.
    pub fn is_invalid_cluster_state_fault(&self) -> bool {
        matches!(self, Self::InvalidClusterStateFault(_))
    }
    /// Returns `true` if the error kind is `CreateClusterSnapshotError::InvalidRetentionPeriodFault`.
    pub fn is_invalid_retention_period_fault(&self) -> bool {
        matches!(self, Self::InvalidRetentionPeriodFault(_))
    }
    /// Returns `true` if the error kind is `CreateClusterSnapshotError::InvalidTagFault`.
    pub fn is_invalid_tag_fault(&self) -> bool {
        matches!(self, Self::InvalidTagFault(_))
    }
    /// Returns `true` if the error kind is `CreateClusterSnapshotError::TagLimitExceededFault`.
    pub fn is_tag_limit_exceeded_fault(&self) -> bool {
        matches!(self, Self::TagLimitExceededFault(_))
    }
}
impl ::std::error::Error for CreateClusterSnapshotError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::ClusterNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::ClusterSnapshotAlreadyExistsFault(_inner) => ::std::option::Option::Some(_inner),
            Self::ClusterSnapshotQuotaExceededFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidClusterStateFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidRetentionPeriodFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidTagFault(_inner) => ::std::option::Option::Some(_inner),
            Self::TagLimitExceededFault(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::create_cluster_snapshot::_create_cluster_snapshot_output::CreateClusterSnapshotOutput;

pub use crate::operation::create_cluster_snapshot::_create_cluster_snapshot_input::CreateClusterSnapshotInput;

mod _create_cluster_snapshot_input;

mod _create_cluster_snapshot_output;

/// Builders
pub mod builders;
