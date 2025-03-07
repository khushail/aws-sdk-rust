// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>The input fails to satisfy the constraints specified by the service.</p>
    BadRequestException(crate::types::error::BadRequestException),
    /// <p>There was an internal failure in the service.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_latest_configuration::GetLatestConfigurationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_latest_configuration::GetLatestConfigurationError,
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
impl From<crate::operation::get_latest_configuration::GetLatestConfigurationError> for Error {
    fn from(err: crate::operation::get_latest_configuration::GetLatestConfigurationError) -> Self {
        match err {
            crate::operation::get_latest_configuration::GetLatestConfigurationError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::get_latest_configuration::GetLatestConfigurationError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_latest_configuration::GetLatestConfigurationError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_latest_configuration::GetLatestConfigurationError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_latest_configuration::GetLatestConfigurationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_configuration_session::StartConfigurationSessionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::start_configuration_session::StartConfigurationSessionError,
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
impl From<crate::operation::start_configuration_session::StartConfigurationSessionError> for Error {
    fn from(
        err: crate::operation::start_configuration_session::StartConfigurationSessionError,
    ) -> Self {
        match err {
            crate::operation::start_configuration_session::StartConfigurationSessionError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::start_configuration_session::StartConfigurationSessionError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::start_configuration_session::StartConfigurationSessionError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::start_configuration_session::StartConfigurationSessionError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::start_configuration_session::StartConfigurationSessionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::BadRequestException(inner) => inner.source(),
            Error::InternalServerException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::ThrottlingException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::BadRequestException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
