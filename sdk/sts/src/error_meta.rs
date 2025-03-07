// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>The web identity token that was passed is expired or is not valid. Get a new identity token from the identity provider and then retry the request.</p>
    ExpiredTokenException(crate::types::error::ExpiredTokenException),
    /// <p>The request could not be fulfilled because the identity provider (IDP) that was asked to verify the incoming identity token could not be reached. This is often a transient error caused by network conditions. Retry the request a limited number of times so that you don't exceed the request rate. If the error persists, the identity provider might be down or not responding.</p>
    IdpCommunicationErrorException(crate::types::error::IdpCommunicationErrorException),
    /// <p>The identity provider (IdP) reported that authentication failed. This might be because the claim is invalid.</p>
    /// <p>If this error is returned for the <code>AssumeRoleWithWebIdentity</code> operation, it can also mean that the claim has expired or has been explicitly revoked. </p>
    IdpRejectedClaimException(crate::types::error::IdpRejectedClaimException),
    /// <p>The error returned if the message passed to <code>DecodeAuthorizationMessage</code> was invalid. This can happen if the token contains invalid characters, such as linebreaks. </p>
    InvalidAuthorizationMessageException(crate::types::error::InvalidAuthorizationMessageException),
    /// <p>The web identity token that was passed could not be validated by Amazon Web Services. Get a new identity token from the identity provider and then retry the request.</p>
    InvalidIdentityTokenException(crate::types::error::InvalidIdentityTokenException),
    /// <p>The request was rejected because the policy document was malformed. The error message describes the specific error.</p>
    MalformedPolicyDocumentException(crate::types::error::MalformedPolicyDocumentException),
    /// <p>The request was rejected because the total packed size of the session policies and session tags combined was too large. An Amazon Web Services conversion compresses the session policy document, session policy ARNs, and session tags into a packed binary format that has a separate limit. The error message indicates by percentage how close the policies and tags are to the upper size limit. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_session-tags.html">Passing Session Tags in STS</a> in the <i>IAM User Guide</i>.</p>
    /// <p>You could receive this error even though you meet other defined session policy and session tag limits. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-limits-entity-length">IAM and STS Entity Character Limits</a> in the <i>IAM User Guide</i>.</p>
    PackedPolicyTooLargeException(crate::types::error::PackedPolicyTooLargeException),
    /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and Deactivating Amazon Web Services STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
    RegionDisabledException(crate::types::error::RegionDisabledException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ExpiredTokenException(inner) => inner.fmt(f),
            Error::IdpCommunicationErrorException(inner) => inner.fmt(f),
            Error::IdpRejectedClaimException(inner) => inner.fmt(f),
            Error::InvalidAuthorizationMessageException(inner) => inner.fmt(f),
            Error::InvalidIdentityTokenException(inner) => inner.fmt(f),
            Error::MalformedPolicyDocumentException(inner) => inner.fmt(f),
            Error::PackedPolicyTooLargeException(inner) => inner.fmt(f),
            Error::RegionDisabledException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<::aws_smithy_http::result::SdkError<crate::operation::assume_role::AssumeRoleError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<crate::operation::assume_role::AssumeRoleError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::assume_role::AssumeRoleError> for Error {
    fn from(err: crate::operation::assume_role::AssumeRoleError) -> Self {
        match err {
            crate::operation::assume_role::AssumeRoleError::ExpiredTokenException(inner) => {
                Error::ExpiredTokenException(inner)
            }
            crate::operation::assume_role::AssumeRoleError::MalformedPolicyDocumentException(
                inner,
            ) => Error::MalformedPolicyDocumentException(inner),
            crate::operation::assume_role::AssumeRoleError::PackedPolicyTooLargeException(
                inner,
            ) => Error::PackedPolicyTooLargeException(inner),
            crate::operation::assume_role::AssumeRoleError::RegionDisabledException(inner) => {
                Error::RegionDisabledException(inner)
            }
            crate::operation::assume_role::AssumeRoleError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError> for Error {
    fn from(err: crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError) -> Self {
        match err {
            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::IdpRejectedClaimException(inner) => Error::IdpRejectedClaimException(inner),
            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::InvalidIdentityTokenException(inner) => Error::InvalidIdentityTokenException(inner),
            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::MalformedPolicyDocumentException(inner) => Error::MalformedPolicyDocumentException(inner),
            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::PackedPolicyTooLargeException(inner) => Error::PackedPolicyTooLargeException(inner),
            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
            crate::operation::assume_role_with_saml::AssumeRoleWithSAMLError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError>
    for Error
{
    fn from(
        err: crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError,
    ) -> Self {
        match err {
            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::IdpCommunicationErrorException(inner) => Error::IdpCommunicationErrorException(inner),
            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::IdpRejectedClaimException(inner) => Error::IdpRejectedClaimException(inner),
            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::InvalidIdentityTokenException(inner) => Error::InvalidIdentityTokenException(inner),
            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::MalformedPolicyDocumentException(inner) => Error::MalformedPolicyDocumentException(inner),
            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::PackedPolicyTooLargeException(inner) => Error::PackedPolicyTooLargeException(inner),
            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
            crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::decode_authorization_message::DecodeAuthorizationMessageError>
    for Error
{
    fn from(
        err: crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
    ) -> Self {
        match err {
            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::InvalidAuthorizationMessageException(inner) => Error::InvalidAuthorizationMessageException(inner),
            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_access_key_info::GetAccessKeyInfoError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_access_key_info::GetAccessKeyInfoError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::get_access_key_info::GetAccessKeyInfoError> for Error {
    fn from(err: crate::operation::get_access_key_info::GetAccessKeyInfoError) -> Self {
        match err {
            crate::operation::get_access_key_info::GetAccessKeyInfoError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_caller_identity::GetCallerIdentityError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_caller_identity::GetCallerIdentityError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::get_caller_identity::GetCallerIdentityError> for Error {
    fn from(err: crate::operation::get_caller_identity::GetCallerIdentityError) -> Self {
        match err {
            crate::operation::get_caller_identity::GetCallerIdentityError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_federation_token::GetFederationTokenError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_federation_token::GetFederationTokenError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::get_federation_token::GetFederationTokenError> for Error {
    fn from(err: crate::operation::get_federation_token::GetFederationTokenError) -> Self {
        match err {
            crate::operation::get_federation_token::GetFederationTokenError::MalformedPolicyDocumentException(inner) => Error::MalformedPolicyDocumentException(inner),
            crate::operation::get_federation_token::GetFederationTokenError::PackedPolicyTooLargeException(inner) => Error::PackedPolicyTooLargeException(inner),
            crate::operation::get_federation_token::GetFederationTokenError::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
            crate::operation::get_federation_token::GetFederationTokenError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_session_token::GetSessionTokenError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_session_token::GetSessionTokenError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::get_session_token::GetSessionTokenError> for Error {
    fn from(err: crate::operation::get_session_token::GetSessionTokenError) -> Self {
        match err {
            crate::operation::get_session_token::GetSessionTokenError::RegionDisabledException(
                inner,
            ) => Error::RegionDisabledException(inner),
            crate::operation::get_session_token::GetSessionTokenError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::ExpiredTokenException(inner) => inner.source(),
            Error::IdpCommunicationErrorException(inner) => inner.source(),
            Error::IdpRejectedClaimException(inner) => inner.source(),
            Error::InvalidAuthorizationMessageException(inner) => inner.source(),
            Error::InvalidIdentityTokenException(inner) => inner.source(),
            Error::MalformedPolicyDocumentException(inner) => inner.source(),
            Error::PackedPolicyTooLargeException(inner) => inner.source(),
            Error::RegionDisabledException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::ExpiredTokenException(e) => e.request_id(),
            Self::IdpCommunicationErrorException(e) => e.request_id(),
            Self::IdpRejectedClaimException(e) => e.request_id(),
            Self::InvalidAuthorizationMessageException(e) => e.request_id(),
            Self::InvalidIdentityTokenException(e) => e.request_id(),
            Self::MalformedPolicyDocumentException(e) => e.request_id(),
            Self::PackedPolicyTooLargeException(e) => e.request_id(),
            Self::RegionDisabledException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
