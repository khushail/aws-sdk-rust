// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have permission to perform this action.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>Your request exceeds one or more of the service quotas.</p>
    ServiceQuotaExceededException(crate::types::error::ServiceQuotaExceededException),
    /// <p>The calls to AWS Application Cost Profiler API are throttled. The request was denied.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>The input fails to satisfy the constraints for the API.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_report_definition::DeleteReportDefinitionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::delete_report_definition::DeleteReportDefinitionError,
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
impl From<crate::operation::delete_report_definition::DeleteReportDefinitionError> for Error {
    fn from(err: crate::operation::delete_report_definition::DeleteReportDefinitionError) -> Self {
        match err {
            crate::operation::delete_report_definition::DeleteReportDefinitionError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::delete_report_definition::DeleteReportDefinitionError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::delete_report_definition::DeleteReportDefinitionError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::delete_report_definition::DeleteReportDefinitionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::delete_report_definition::DeleteReportDefinitionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_report_definition::GetReportDefinitionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::get_report_definition::GetReportDefinitionError,
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
impl From<crate::operation::get_report_definition::GetReportDefinitionError> for Error {
    fn from(err: crate::operation::get_report_definition::GetReportDefinitionError) -> Self {
        match err {
            crate::operation::get_report_definition::GetReportDefinitionError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::get_report_definition::GetReportDefinitionError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_report_definition::GetReportDefinitionError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_report_definition::GetReportDefinitionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_report_definition::GetReportDefinitionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::import_application_usage::ImportApplicationUsageError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::import_application_usage::ImportApplicationUsageError,
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
impl From<crate::operation::import_application_usage::ImportApplicationUsageError> for Error {
    fn from(err: crate::operation::import_application_usage::ImportApplicationUsageError) -> Self {
        match err {
            crate::operation::import_application_usage::ImportApplicationUsageError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::import_application_usage::ImportApplicationUsageError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::import_application_usage::ImportApplicationUsageError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::import_application_usage::ImportApplicationUsageError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::import_application_usage::ImportApplicationUsageError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_report_definitions::ListReportDefinitionsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::list_report_definitions::ListReportDefinitionsError,
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
impl From<crate::operation::list_report_definitions::ListReportDefinitionsError> for Error {
    fn from(err: crate::operation::list_report_definitions::ListReportDefinitionsError) -> Self {
        match err {
            crate::operation::list_report_definitions::ListReportDefinitionsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::list_report_definitions::ListReportDefinitionsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_report_definitions::ListReportDefinitionsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_report_definitions::ListReportDefinitionsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_report_definitions::ListReportDefinitionsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_report_definition::PutReportDefinitionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::put_report_definition::PutReportDefinitionError,
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
impl From<crate::operation::put_report_definition::PutReportDefinitionError> for Error {
    fn from(err: crate::operation::put_report_definition::PutReportDefinitionError) -> Self {
        match err {
            crate::operation::put_report_definition::PutReportDefinitionError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::put_report_definition::PutReportDefinitionError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::put_report_definition::PutReportDefinitionError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::operation::put_report_definition::PutReportDefinitionError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::put_report_definition::PutReportDefinitionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::put_report_definition::PutReportDefinitionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_report_definition::UpdateReportDefinitionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::update_report_definition::UpdateReportDefinitionError,
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
impl From<crate::operation::update_report_definition::UpdateReportDefinitionError> for Error {
    fn from(err: crate::operation::update_report_definition::UpdateReportDefinitionError) -> Self {
        match err {
            crate::operation::update_report_definition::UpdateReportDefinitionError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::update_report_definition::UpdateReportDefinitionError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::update_report_definition::UpdateReportDefinitionError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::update_report_definition::UpdateReportDefinitionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::update_report_definition::UpdateReportDefinitionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::AccessDeniedException(inner) => inner.source(),
            Error::InternalServerException(inner) => inner.source(),
            Error::ServiceQuotaExceededException(inner) => inner.source(),
            Error::ThrottlingException(inner) => inner.source(),
            Error::ValidationException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ServiceQuotaExceededException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
