// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl UpdateRegexMatchSetInput {
    /// Consumes the builder and constructs an Operation<[`UpdateRegexMatchSet`](crate::operation::update_regex_match_set::UpdateRegexMatchSet)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::update_regex_match_set::UpdateRegexMatchSet,
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
                _input: &crate::operation::update_regex_match_set::UpdateRegexMatchSetInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::update_regex_match_set::UpdateRegexMatchSetInput,
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
                "AWSWAF_Regional_20161128.UpdateRegexMatchSet",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_update_regex_match_set::ser_update_regex_match_set_input(
                &self,
            )?,
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
            crate::operation::update_regex_match_set::UpdateRegexMatchSet::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "UpdateRegexMatchSet",
            "wafregional",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `UpdateRegexMatchSet`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct UpdateRegexMatchSet;
impl UpdateRegexMatchSet {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for UpdateRegexMatchSet {
    type Output = ::std::result::Result<
        crate::operation::update_regex_match_set::UpdateRegexMatchSetOutput,
        crate::operation::update_regex_match_set::UpdateRegexMatchSetError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_update_regex_match_set::de_update_regex_match_set_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_update_regex_match_set::de_update_regex_match_set_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type UpdateRegexMatchSetErrorKind = UpdateRegexMatchSetError;
/// Error type for the `UpdateRegexMatchSetError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum UpdateRegexMatchSetError {
    /// <p>The name specified is invalid.</p>
    WafDisallowedNameException(crate::types::error::WafDisallowedNameException),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WafInternalErrorException(crate::types::error::WafInternalErrorException),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WafInvalidAccountException(crate::types::error::WafInvalidAccountException),
    /// <p>The operation failed because there was nothing to do. For example:</p>
    /// <ul>
    /// <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn't in the specified <code>WebACL</code>.</p> </li>
    /// <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn't in the specified <code>IPSet</code>.</p> </li>
    /// <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn't in the specified <code>WebACL</code>.</p> </li>
    /// <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li>
    /// <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li>
    /// </ul>
    WafInvalidOperationException(crate::types::error::WafInvalidOperationException),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WafLimitsExceededException(crate::types::error::WafLimitsExceededException),
    /// <p>The operation failed because you tried to add an object to or delete an object from another object that doesn't exist. For example:</p>
    /// <ul>
    /// <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn't exist.</p> </li>
    /// <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn't exist.</p> </li>
    /// <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn't exist.</p> </li>
    /// <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn't exist.</p> </li>
    /// </ul>
    WafNonexistentContainerException(crate::types::error::WafNonexistentContainerException),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WafNonexistentItemException(crate::types::error::WafNonexistentItemException),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WafStaleDataException(crate::types::error::WafStaleDataException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for UpdateRegexMatchSetError {
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
impl ::std::fmt::Display for UpdateRegexMatchSetError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::WafDisallowedNameException(_inner) => _inner.fmt(f),
            Self::WafInternalErrorException(_inner) => _inner.fmt(f),
            Self::WafInvalidAccountException(_inner) => _inner.fmt(f),
            Self::WafInvalidOperationException(_inner) => _inner.fmt(f),
            Self::WafLimitsExceededException(_inner) => _inner.fmt(f),
            Self::WafNonexistentContainerException(_inner) => _inner.fmt(f),
            Self::WafNonexistentItemException(_inner) => _inner.fmt(f),
            Self::WafStaleDataException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for UpdateRegexMatchSetError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::WafDisallowedNameException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafInternalErrorException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafInvalidAccountException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafInvalidOperationException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafLimitsExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafNonexistentContainerException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafNonexistentItemException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafStaleDataException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::update_regex_match_set::UpdateRegexMatchSetError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for UpdateRegexMatchSetError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl UpdateRegexMatchSetError {
    /// Creates the `UpdateRegexMatchSetError::Unhandled` variant from any error type.
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

    /// Creates the `UpdateRegexMatchSetError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::WafDisallowedNameException(e) => e.meta(),
            Self::WafInternalErrorException(e) => e.meta(),
            Self::WafInvalidAccountException(e) => e.meta(),
            Self::WafInvalidOperationException(e) => e.meta(),
            Self::WafLimitsExceededException(e) => e.meta(),
            Self::WafNonexistentContainerException(e) => e.meta(),
            Self::WafNonexistentItemException(e) => e.meta(),
            Self::WafStaleDataException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `UpdateRegexMatchSetError::WafDisallowedNameException`.
    pub fn is_waf_disallowed_name_exception(&self) -> bool {
        matches!(self, Self::WafDisallowedNameException(_))
    }
    /// Returns `true` if the error kind is `UpdateRegexMatchSetError::WafInternalErrorException`.
    pub fn is_waf_internal_error_exception(&self) -> bool {
        matches!(self, Self::WafInternalErrorException(_))
    }
    /// Returns `true` if the error kind is `UpdateRegexMatchSetError::WafInvalidAccountException`.
    pub fn is_waf_invalid_account_exception(&self) -> bool {
        matches!(self, Self::WafInvalidAccountException(_))
    }
    /// Returns `true` if the error kind is `UpdateRegexMatchSetError::WafInvalidOperationException`.
    pub fn is_waf_invalid_operation_exception(&self) -> bool {
        matches!(self, Self::WafInvalidOperationException(_))
    }
    /// Returns `true` if the error kind is `UpdateRegexMatchSetError::WafLimitsExceededException`.
    pub fn is_waf_limits_exceeded_exception(&self) -> bool {
        matches!(self, Self::WafLimitsExceededException(_))
    }
    /// Returns `true` if the error kind is `UpdateRegexMatchSetError::WafNonexistentContainerException`.
    pub fn is_waf_nonexistent_container_exception(&self) -> bool {
        matches!(self, Self::WafNonexistentContainerException(_))
    }
    /// Returns `true` if the error kind is `UpdateRegexMatchSetError::WafNonexistentItemException`.
    pub fn is_waf_nonexistent_item_exception(&self) -> bool {
        matches!(self, Self::WafNonexistentItemException(_))
    }
    /// Returns `true` if the error kind is `UpdateRegexMatchSetError::WafStaleDataException`.
    pub fn is_waf_stale_data_exception(&self) -> bool {
        matches!(self, Self::WafStaleDataException(_))
    }
}
impl ::std::error::Error for UpdateRegexMatchSetError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::WafDisallowedNameException(_inner) => ::std::option::Option::Some(_inner),
            Self::WafInternalErrorException(_inner) => ::std::option::Option::Some(_inner),
            Self::WafInvalidAccountException(_inner) => ::std::option::Option::Some(_inner),
            Self::WafInvalidOperationException(_inner) => ::std::option::Option::Some(_inner),
            Self::WafLimitsExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::WafNonexistentContainerException(_inner) => ::std::option::Option::Some(_inner),
            Self::WafNonexistentItemException(_inner) => ::std::option::Option::Some(_inner),
            Self::WafStaleDataException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::update_regex_match_set::_update_regex_match_set_output::UpdateRegexMatchSetOutput;

pub use crate::operation::update_regex_match_set::_update_regex_match_set_input::UpdateRegexMatchSetInput;

mod _update_regex_match_set_input;

mod _update_regex_match_set_output;

/// Builders
pub mod builders;
