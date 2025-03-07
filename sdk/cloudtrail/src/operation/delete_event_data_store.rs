// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl DeleteEventDataStoreInput {
    /// Consumes the builder and constructs an Operation<[`DeleteEventDataStore`](crate::operation::delete_event_data_store::DeleteEventDataStore)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::delete_event_data_store::DeleteEventDataStore,
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
                _input: &crate::operation::delete_event_data_store::DeleteEventDataStoreInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::delete_event_data_store::DeleteEventDataStoreInput,
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
                "CloudTrail_20131101.DeleteEventDataStore",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_delete_event_data_store::ser_delete_event_data_store_input(&self)?
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
            crate::operation::delete_event_data_store::DeleteEventDataStore::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "DeleteEventDataStore",
            "cloudtrail",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `DeleteEventDataStore`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct DeleteEventDataStore;
impl DeleteEventDataStore {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for DeleteEventDataStore {
    type Output = ::std::result::Result<
        crate::operation::delete_event_data_store::DeleteEventDataStoreOutput,
        crate::operation::delete_event_data_store::DeleteEventDataStoreError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_delete_event_data_store::de_delete_event_data_store_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_delete_event_data_store::de_delete_event_data_store_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type DeleteEventDataStoreErrorKind = DeleteEventDataStoreError;
/// Error type for the `DeleteEventDataStoreError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum DeleteEventDataStoreError {
    /// <p>This exception is thrown when the specified event data store cannot yet be deleted because it is in use by a channel.</p>
    ChannelExistsForEdsException(crate::types::error::ChannelExistsForEdsException),
    /// <p>The specified event data store ARN is not valid or does not map to an event data store in your account.</p>
    EventDataStoreArnInvalidException(crate::types::error::EventDataStoreArnInvalidException),
    /// <p> This exception is thrown when you try to update or delete an event data store that currently has an import in progress. </p>
    EventDataStoreHasOngoingImportException(
        crate::types::error::EventDataStoreHasOngoingImportException,
    ),
    /// <p>The specified event data store was not found.</p>
    EventDataStoreNotFoundException(crate::types::error::EventDataStoreNotFoundException),
    /// <p>The event data store cannot be deleted because termination protection is enabled for it.</p>
    EventDataStoreTerminationProtectedException(
        crate::types::error::EventDataStoreTerminationProtectedException,
    ),
    /// <p>The event data store is inactive.</p>
    InactiveEventDataStoreException(crate::types::error::InactiveEventDataStoreException),
    /// <p>This exception is thrown when the IAM identity that is used to create the organization resource lacks one or more required permissions for creating an organization resource in a required service.</p>
    InsufficientDependencyServiceAccessPermissionException(
        crate::types::error::InsufficientDependencyServiceAccessPermissionException,
    ),
    /// <p>The request includes a parameter that is not valid.</p>
    InvalidParameterException(crate::types::error::InvalidParameterException),
    /// <p> This exception is thrown when the management account does not have a service-linked role. </p>
    NoManagementAccountSlrExistsException(
        crate::types::error::NoManagementAccountSlrExistsException,
    ),
    /// <p>This exception is thrown when the Amazon Web Services account making the request to create or update an organization trail or event data store is not the management account for an organization in Organizations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a> or <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-event-data-store.html">Create an event data store</a>.</p>
    NotOrganizationMasterAccountException(
        crate::types::error::NotOrganizationMasterAccountException,
    ),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermittedException(crate::types::error::OperationNotPermittedException),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperationException(crate::types::error::UnsupportedOperationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for DeleteEventDataStoreError {
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
impl ::std::fmt::Display for DeleteEventDataStoreError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::ChannelExistsForEdsException(_inner) => _inner.fmt(f),
            Self::EventDataStoreArnInvalidException(_inner) => _inner.fmt(f),
            Self::EventDataStoreHasOngoingImportException(_inner) => _inner.fmt(f),
            Self::EventDataStoreNotFoundException(_inner) => _inner.fmt(f),
            Self::EventDataStoreTerminationProtectedException(_inner) => _inner.fmt(f),
            Self::InactiveEventDataStoreException(_inner) => _inner.fmt(f),
            Self::InsufficientDependencyServiceAccessPermissionException(_inner) => _inner.fmt(f),
            Self::InvalidParameterException(_inner) => _inner.fmt(f),
            Self::NoManagementAccountSlrExistsException(_inner) => _inner.fmt(f),
            Self::NotOrganizationMasterAccountException(_inner) => _inner.fmt(f),
            Self::OperationNotPermittedException(_inner) => _inner.fmt(f),
            Self::UnsupportedOperationException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for DeleteEventDataStoreError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ChannelExistsForEdsException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EventDataStoreArnInvalidException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EventDataStoreHasOngoingImportException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EventDataStoreNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EventDataStoreTerminationProtectedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InactiveEventDataStoreException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InsufficientDependencyServiceAccessPermissionException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidParameterException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NoManagementAccountSlrExistsException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NotOrganizationMasterAccountException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::OperationNotPermittedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnsupportedOperationException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::delete_event_data_store::DeleteEventDataStoreError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for DeleteEventDataStoreError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl DeleteEventDataStoreError {
    /// Creates the `DeleteEventDataStoreError::Unhandled` variant from any error type.
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

    /// Creates the `DeleteEventDataStoreError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::ChannelExistsForEdsException(e) => e.meta(),
            Self::EventDataStoreArnInvalidException(e) => e.meta(),
            Self::EventDataStoreHasOngoingImportException(e) => e.meta(),
            Self::EventDataStoreNotFoundException(e) => e.meta(),
            Self::EventDataStoreTerminationProtectedException(e) => e.meta(),
            Self::InactiveEventDataStoreException(e) => e.meta(),
            Self::InsufficientDependencyServiceAccessPermissionException(e) => e.meta(),
            Self::InvalidParameterException(e) => e.meta(),
            Self::NoManagementAccountSlrExistsException(e) => e.meta(),
            Self::NotOrganizationMasterAccountException(e) => e.meta(),
            Self::OperationNotPermittedException(e) => e.meta(),
            Self::UnsupportedOperationException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::ChannelExistsForEdsException`.
    pub fn is_channel_exists_for_eds_exception(&self) -> bool {
        matches!(self, Self::ChannelExistsForEdsException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::EventDataStoreArnInvalidException`.
    pub fn is_event_data_store_arn_invalid_exception(&self) -> bool {
        matches!(self, Self::EventDataStoreArnInvalidException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::EventDataStoreHasOngoingImportException`.
    pub fn is_event_data_store_has_ongoing_import_exception(&self) -> bool {
        matches!(self, Self::EventDataStoreHasOngoingImportException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::EventDataStoreNotFoundException`.
    pub fn is_event_data_store_not_found_exception(&self) -> bool {
        matches!(self, Self::EventDataStoreNotFoundException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::EventDataStoreTerminationProtectedException`.
    pub fn is_event_data_store_termination_protected_exception(&self) -> bool {
        matches!(self, Self::EventDataStoreTerminationProtectedException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::InactiveEventDataStoreException`.
    pub fn is_inactive_event_data_store_exception(&self) -> bool {
        matches!(self, Self::InactiveEventDataStoreException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::InsufficientDependencyServiceAccessPermissionException`.
    pub fn is_insufficient_dependency_service_access_permission_exception(&self) -> bool {
        matches!(
            self,
            Self::InsufficientDependencyServiceAccessPermissionException(_)
        )
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::InvalidParameterException`.
    pub fn is_invalid_parameter_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::NoManagementAccountSlrExistsException`.
    pub fn is_no_management_account_slr_exists_exception(&self) -> bool {
        matches!(self, Self::NoManagementAccountSlrExistsException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::NotOrganizationMasterAccountException`.
    pub fn is_not_organization_master_account_exception(&self) -> bool {
        matches!(self, Self::NotOrganizationMasterAccountException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::OperationNotPermittedException`.
    pub fn is_operation_not_permitted_exception(&self) -> bool {
        matches!(self, Self::OperationNotPermittedException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventDataStoreError::UnsupportedOperationException`.
    pub fn is_unsupported_operation_exception(&self) -> bool {
        matches!(self, Self::UnsupportedOperationException(_))
    }
}
impl ::std::error::Error for DeleteEventDataStoreError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::ChannelExistsForEdsException(_inner) => ::std::option::Option::Some(_inner),
            Self::EventDataStoreArnInvalidException(_inner) => ::std::option::Option::Some(_inner),
            Self::EventDataStoreHasOngoingImportException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::EventDataStoreNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::EventDataStoreTerminationProtectedException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::InactiveEventDataStoreException(_inner) => ::std::option::Option::Some(_inner),
            Self::InsufficientDependencyServiceAccessPermissionException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::InvalidParameterException(_inner) => ::std::option::Option::Some(_inner),
            Self::NoManagementAccountSlrExistsException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::NotOrganizationMasterAccountException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::OperationNotPermittedException(_inner) => ::std::option::Option::Some(_inner),
            Self::UnsupportedOperationException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::delete_event_data_store::_delete_event_data_store_output::DeleteEventDataStoreOutput;

pub use crate::operation::delete_event_data_store::_delete_event_data_store_input::DeleteEventDataStoreInput;

mod _delete_event_data_store_input;

mod _delete_event_data_store_output;

/// Builders
pub mod builders;
