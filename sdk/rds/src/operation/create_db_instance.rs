// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl CreateDbInstanceInput {
    /// Consumes the builder and constructs an Operation<[`CreateDBInstance`](crate::operation::create_db_instance::CreateDBInstance)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::create_db_instance::CreateDBInstance,
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
                _input: &crate::operation::create_db_instance::CreateDbInstanceInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_db_instance::CreateDbInstanceInput,
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
            crate::protocol_serde::shape_create_db_instance_input::ser_create_db_instance_input_input(&self)?
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
            crate::operation::create_db_instance::CreateDBInstance::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "CreateDBInstance",
            "rds",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `CreateDBInstance`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct CreateDBInstance;
impl CreateDBInstance {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for CreateDBInstance {
    type Output = ::std::result::Result<
        crate::operation::create_db_instance::CreateDbInstanceOutput,
        crate::operation::create_db_instance::CreateDBInstanceError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_create_db_instance::de_create_db_instance_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_create_db_instance::de_create_db_instance_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type CreateDBInstanceErrorKind = CreateDBInstanceError;
/// Error type for the `CreateDBInstanceError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum CreateDBInstanceError {
    /// <p>The specified CIDR IP range or Amazon EC2 security group might not be authorized for the specified DB security group.</p>
    /// <p>Or, RDS might not be authorized to perform necessary actions using IAM on your behalf.</p>
    AuthorizationNotFoundFault(crate::types::error::AuthorizationNotFoundFault),
    #[allow(missing_docs)] // documentation missing in model
    #[deprecated(note = "Please avoid using this fault")]
    BackupPolicyNotFoundFault(crate::types::error::BackupPolicyNotFoundFault),
    /// <p> <code>CertificateIdentifier</code> doesn't refer to an existing certificate.</p>
    CertificateNotFoundFault(crate::types::error::CertificateNotFoundFault),
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster.</p>
    DbClusterNotFoundFault(crate::types::error::DbClusterNotFoundFault),
    /// <p>The user already has a DB instance with the given identifier.</p>
    DbInstanceAlreadyExistsFault(crate::types::error::DbInstanceAlreadyExistsFault),
    /// <p> <code>DBParameterGroupName</code> doesn't refer to an existing DB parameter group.</p>
    DbParameterGroupNotFoundFault(crate::types::error::DbParameterGroupNotFoundFault),
    /// <p> <code>DBSecurityGroupName</code> doesn't refer to an existing DB security group.</p>
    DbSecurityGroupNotFoundFault(crate::types::error::DbSecurityGroupNotFoundFault),
    /// <p>Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone.</p>
    DbSubnetGroupDoesNotCoverEnoughAZs(crate::types::error::DbSubnetGroupDoesNotCoverEnoughAZs),
    /// <p> <code>DBSubnetGroupName</code> doesn't refer to an existing DB subnet group.</p>
    DbSubnetGroupNotFoundFault(crate::types::error::DbSubnetGroupNotFoundFault),
    /// <p> <code>Domain</code> doesn't refer to an existing Active Directory domain.</p>
    DomainNotFoundFault(crate::types::error::DomainNotFoundFault),
    /// <p>The request would result in the user exceeding the allowed number of DB instances.</p>
    InstanceQuotaExceededFault(crate::types::error::InstanceQuotaExceededFault),
    /// <p>The specified DB instance class isn't available in the specified Availability Zone.</p>
    InsufficientDbInstanceCapacityFault(crate::types::error::InsufficientDbInstanceCapacityFault),
    /// <p>The requested operation can't be performed while the cluster is in this state.</p>
    InvalidDbClusterStateFault(crate::types::error::InvalidDbClusterStateFault),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(crate::types::error::InvalidSubnet),
    /// <p>The DB subnet group doesn't cover all Availability Zones after it's created because of users' change.</p>
    InvalidVpcNetworkStateFault(crate::types::error::InvalidVpcNetworkStateFault),
    /// <p>An error occurred accessing an Amazon Web Services KMS key.</p>
    KmsKeyNotAccessibleFault(crate::types::error::KmsKeyNotAccessibleFault),
    /// <p>The network type is invalid for the DB instance. Valid nework type values are <code>IPV4</code> and <code>DUAL</code>.</p>
    NetworkTypeNotSupported(crate::types::error::NetworkTypeNotSupported),
    /// <p>The specified option group could not be found.</p>
    OptionGroupNotFoundFault(crate::types::error::OptionGroupNotFoundFault),
    /// <p>Provisioned IOPS not available in the specified Availability Zone.</p>
    ProvisionedIopsNotAvailableInAzFault(crate::types::error::ProvisionedIopsNotAvailableInAzFault),
    /// <p>The request would result in the user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(crate::types::error::StorageQuotaExceededFault),
    /// <p>The specified <code>StorageType</code> can't be associated with the DB instance.</p>
    StorageTypeNotSupportedFault(crate::types::error::StorageTypeNotSupportedFault),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for CreateDBInstanceError {
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
impl ::std::fmt::Display for CreateDBInstanceError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::AuthorizationNotFoundFault(_inner) => _inner.fmt(f),
            Self::BackupPolicyNotFoundFault(_inner) => _inner.fmt(f),
            Self::CertificateNotFoundFault(_inner) => _inner.fmt(f),
            Self::DbClusterNotFoundFault(_inner) => _inner.fmt(f),
            Self::DbInstanceAlreadyExistsFault(_inner) => _inner.fmt(f),
            Self::DbParameterGroupNotFoundFault(_inner) => _inner.fmt(f),
            Self::DbSecurityGroupNotFoundFault(_inner) => _inner.fmt(f),
            Self::DbSubnetGroupDoesNotCoverEnoughAZs(_inner) => _inner.fmt(f),
            Self::DbSubnetGroupNotFoundFault(_inner) => _inner.fmt(f),
            Self::DomainNotFoundFault(_inner) => _inner.fmt(f),
            Self::InstanceQuotaExceededFault(_inner) => _inner.fmt(f),
            Self::InsufficientDbInstanceCapacityFault(_inner) => _inner.fmt(f),
            Self::InvalidDbClusterStateFault(_inner) => _inner.fmt(f),
            Self::InvalidSubnet(_inner) => _inner.fmt(f),
            Self::InvalidVpcNetworkStateFault(_inner) => _inner.fmt(f),
            Self::KmsKeyNotAccessibleFault(_inner) => _inner.fmt(f),
            Self::NetworkTypeNotSupported(_inner) => _inner.fmt(f),
            Self::OptionGroupNotFoundFault(_inner) => _inner.fmt(f),
            Self::ProvisionedIopsNotAvailableInAzFault(_inner) => _inner.fmt(f),
            Self::StorageQuotaExceededFault(_inner) => _inner.fmt(f),
            Self::StorageTypeNotSupportedFault(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateDBInstanceError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AuthorizationNotFoundFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::BackupPolicyNotFoundFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CertificateNotFoundFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DbClusterNotFoundFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DbInstanceAlreadyExistsFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DbParameterGroupNotFoundFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DbSecurityGroupNotFoundFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DbSubnetGroupDoesNotCoverEnoughAZs(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DbSubnetGroupNotFoundFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DomainNotFoundFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InstanceQuotaExceededFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InsufficientDbInstanceCapacityFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidDbClusterStateFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidSubnet(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidVpcNetworkStateFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsKeyNotAccessibleFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NetworkTypeNotSupported(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::OptionGroupNotFoundFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ProvisionedIopsNotAvailableInAzFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::StorageQuotaExceededFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::StorageTypeNotSupportedFault(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::create_db_instance::CreateDBInstanceError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for CreateDBInstanceError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl CreateDBInstanceError {
    /// Creates the `CreateDBInstanceError::Unhandled` variant from any error type.
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

    /// Creates the `CreateDBInstanceError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::AuthorizationNotFoundFault(e) => e.meta(),
            Self::BackupPolicyNotFoundFault(e) => e.meta(),
            Self::CertificateNotFoundFault(e) => e.meta(),
            Self::DbClusterNotFoundFault(e) => e.meta(),
            Self::DbInstanceAlreadyExistsFault(e) => e.meta(),
            Self::DbParameterGroupNotFoundFault(e) => e.meta(),
            Self::DbSecurityGroupNotFoundFault(e) => e.meta(),
            Self::DbSubnetGroupDoesNotCoverEnoughAZs(e) => e.meta(),
            Self::DbSubnetGroupNotFoundFault(e) => e.meta(),
            Self::DomainNotFoundFault(e) => e.meta(),
            Self::InstanceQuotaExceededFault(e) => e.meta(),
            Self::InsufficientDbInstanceCapacityFault(e) => e.meta(),
            Self::InvalidDbClusterStateFault(e) => e.meta(),
            Self::InvalidSubnet(e) => e.meta(),
            Self::InvalidVpcNetworkStateFault(e) => e.meta(),
            Self::KmsKeyNotAccessibleFault(e) => e.meta(),
            Self::NetworkTypeNotSupported(e) => e.meta(),
            Self::OptionGroupNotFoundFault(e) => e.meta(),
            Self::ProvisionedIopsNotAvailableInAzFault(e) => e.meta(),
            Self::StorageQuotaExceededFault(e) => e.meta(),
            Self::StorageTypeNotSupportedFault(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::AuthorizationNotFoundFault`.
    pub fn is_authorization_not_found_fault(&self) -> bool {
        matches!(self, Self::AuthorizationNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::BackupPolicyNotFoundFault`.
    pub fn is_backup_policy_not_found_fault(&self) -> bool {
        matches!(self, Self::BackupPolicyNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::CertificateNotFoundFault`.
    pub fn is_certificate_not_found_fault(&self) -> bool {
        matches!(self, Self::CertificateNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::DbClusterNotFoundFault`.
    pub fn is_db_cluster_not_found_fault(&self) -> bool {
        matches!(self, Self::DbClusterNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::DbInstanceAlreadyExistsFault`.
    pub fn is_db_instance_already_exists_fault(&self) -> bool {
        matches!(self, Self::DbInstanceAlreadyExistsFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::DbParameterGroupNotFoundFault`.
    pub fn is_db_parameter_group_not_found_fault(&self) -> bool {
        matches!(self, Self::DbParameterGroupNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::DbSecurityGroupNotFoundFault`.
    pub fn is_db_security_group_not_found_fault(&self) -> bool {
        matches!(self, Self::DbSecurityGroupNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::DbSubnetGroupDoesNotCoverEnoughAZs`.
    pub fn is_db_subnet_group_does_not_cover_enough_a_zs(&self) -> bool {
        matches!(self, Self::DbSubnetGroupDoesNotCoverEnoughAZs(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::DbSubnetGroupNotFoundFault`.
    pub fn is_db_subnet_group_not_found_fault(&self) -> bool {
        matches!(self, Self::DbSubnetGroupNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::DomainNotFoundFault`.
    pub fn is_domain_not_found_fault(&self) -> bool {
        matches!(self, Self::DomainNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::InstanceQuotaExceededFault`.
    pub fn is_instance_quota_exceeded_fault(&self) -> bool {
        matches!(self, Self::InstanceQuotaExceededFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::InsufficientDbInstanceCapacityFault`.
    pub fn is_insufficient_db_instance_capacity_fault(&self) -> bool {
        matches!(self, Self::InsufficientDbInstanceCapacityFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::InvalidDbClusterStateFault`.
    pub fn is_invalid_db_cluster_state_fault(&self) -> bool {
        matches!(self, Self::InvalidDbClusterStateFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::InvalidSubnet`.
    pub fn is_invalid_subnet(&self) -> bool {
        matches!(self, Self::InvalidSubnet(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::InvalidVpcNetworkStateFault`.
    pub fn is_invalid_vpc_network_state_fault(&self) -> bool {
        matches!(self, Self::InvalidVpcNetworkStateFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::KmsKeyNotAccessibleFault`.
    pub fn is_kms_key_not_accessible_fault(&self) -> bool {
        matches!(self, Self::KmsKeyNotAccessibleFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::NetworkTypeNotSupported`.
    pub fn is_network_type_not_supported(&self) -> bool {
        matches!(self, Self::NetworkTypeNotSupported(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::OptionGroupNotFoundFault`.
    pub fn is_option_group_not_found_fault(&self) -> bool {
        matches!(self, Self::OptionGroupNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::ProvisionedIopsNotAvailableInAzFault`.
    pub fn is_provisioned_iops_not_available_in_az_fault(&self) -> bool {
        matches!(self, Self::ProvisionedIopsNotAvailableInAzFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::StorageQuotaExceededFault`.
    pub fn is_storage_quota_exceeded_fault(&self) -> bool {
        matches!(self, Self::StorageQuotaExceededFault(_))
    }
    /// Returns `true` if the error kind is `CreateDBInstanceError::StorageTypeNotSupportedFault`.
    pub fn is_storage_type_not_supported_fault(&self) -> bool {
        matches!(self, Self::StorageTypeNotSupportedFault(_))
    }
}
impl ::std::error::Error for CreateDBInstanceError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::AuthorizationNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::BackupPolicyNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::CertificateNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DbClusterNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DbInstanceAlreadyExistsFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DbParameterGroupNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DbSecurityGroupNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DbSubnetGroupDoesNotCoverEnoughAZs(_inner) => ::std::option::Option::Some(_inner),
            Self::DbSubnetGroupNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::DomainNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InstanceQuotaExceededFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InsufficientDbInstanceCapacityFault(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::InvalidDbClusterStateFault(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidSubnet(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidVpcNetworkStateFault(_inner) => ::std::option::Option::Some(_inner),
            Self::KmsKeyNotAccessibleFault(_inner) => ::std::option::Option::Some(_inner),
            Self::NetworkTypeNotSupported(_inner) => ::std::option::Option::Some(_inner),
            Self::OptionGroupNotFoundFault(_inner) => ::std::option::Option::Some(_inner),
            Self::ProvisionedIopsNotAvailableInAzFault(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::StorageQuotaExceededFault(_inner) => ::std::option::Option::Some(_inner),
            Self::StorageTypeNotSupportedFault(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::create_db_instance::_create_db_instance_output::CreateDbInstanceOutput;

pub use crate::operation::create_db_instance::_create_db_instance_input::CreateDbInstanceInput;

mod _create_db_instance_input;

mod _create_db_instance_output;

/// Builders
pub mod builders;
