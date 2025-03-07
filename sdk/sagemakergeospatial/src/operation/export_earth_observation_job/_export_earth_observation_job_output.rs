// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExportEarthObservationJobOutput {
    /// <p>The output Amazon Resource Name (ARN) of the Earth Observation job being exported.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The creation time.</p>
    #[doc(hidden)]
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The status of the results of the Earth Observation job being exported.</p>
    #[doc(hidden)]
    pub export_status: ::std::option::Option<crate::types::EarthObservationJobExportStatus>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the job.</p>
    #[doc(hidden)]
    pub execution_role_arn: ::std::option::Option<::std::string::String>,
    /// <p>An object containing information about the output file.</p>
    #[doc(hidden)]
    pub output_config: ::std::option::Option<crate::types::OutputConfigInput>,
    /// <p>The source images provided to the Earth Observation job being exported.</p>
    #[doc(hidden)]
    pub export_source_images: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ExportEarthObservationJobOutput {
    /// <p>The output Amazon Resource Name (ARN) of the Earth Observation job being exported.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The creation time.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The status of the results of the Earth Observation job being exported.</p>
    pub fn export_status(
        &self,
    ) -> ::std::option::Option<&crate::types::EarthObservationJobExportStatus> {
        self.export_status.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the job.</p>
    pub fn execution_role_arn(&self) -> ::std::option::Option<&str> {
        self.execution_role_arn.as_deref()
    }
    /// <p>An object containing information about the output file.</p>
    pub fn output_config(&self) -> ::std::option::Option<&crate::types::OutputConfigInput> {
        self.output_config.as_ref()
    }
    /// <p>The source images provided to the Earth Observation job being exported.</p>
    pub fn export_source_images(&self) -> ::std::option::Option<bool> {
        self.export_source_images
    }
}
impl ::aws_http::request_id::RequestId for ExportEarthObservationJobOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ExportEarthObservationJobOutput {
    /// Creates a new builder-style object to manufacture [`ExportEarthObservationJobOutput`](crate::operation::export_earth_observation_job::ExportEarthObservationJobOutput).
    pub fn builder() -> crate::operation::export_earth_observation_job::builders::ExportEarthObservationJobOutputBuilder{
        crate::operation::export_earth_observation_job::builders::ExportEarthObservationJobOutputBuilder::default()
    }
}

/// A builder for [`ExportEarthObservationJobOutput`](crate::operation::export_earth_observation_job::ExportEarthObservationJobOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExportEarthObservationJobOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) export_status: ::std::option::Option<crate::types::EarthObservationJobExportStatus>,
    pub(crate) execution_role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) output_config: ::std::option::Option<crate::types::OutputConfigInput>,
    pub(crate) export_source_images: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ExportEarthObservationJobOutputBuilder {
    /// <p>The output Amazon Resource Name (ARN) of the Earth Observation job being exported.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The output Amazon Resource Name (ARN) of the Earth Observation job being exported.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The creation time.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The creation time.</p>
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The status of the results of the Earth Observation job being exported.</p>
    pub fn export_status(mut self, input: crate::types::EarthObservationJobExportStatus) -> Self {
        self.export_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the results of the Earth Observation job being exported.</p>
    pub fn set_export_status(
        mut self,
        input: ::std::option::Option<crate::types::EarthObservationJobExportStatus>,
    ) -> Self {
        self.export_status = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the job.</p>
    pub fn execution_role_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.execution_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the job.</p>
    pub fn set_execution_role_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.execution_role_arn = input;
        self
    }
    /// <p>An object containing information about the output file.</p>
    pub fn output_config(mut self, input: crate::types::OutputConfigInput) -> Self {
        self.output_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>An object containing information about the output file.</p>
    pub fn set_output_config(
        mut self,
        input: ::std::option::Option<crate::types::OutputConfigInput>,
    ) -> Self {
        self.output_config = input;
        self
    }
    /// <p>The source images provided to the Earth Observation job being exported.</p>
    pub fn export_source_images(mut self, input: bool) -> Self {
        self.export_source_images = ::std::option::Option::Some(input);
        self
    }
    /// <p>The source images provided to the Earth Observation job being exported.</p>
    pub fn set_export_source_images(mut self, input: ::std::option::Option<bool>) -> Self {
        self.export_source_images = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ExportEarthObservationJobOutput`](crate::operation::export_earth_observation_job::ExportEarthObservationJobOutput).
    pub fn build(
        self,
    ) -> crate::operation::export_earth_observation_job::ExportEarthObservationJobOutput {
        crate::operation::export_earth_observation_job::ExportEarthObservationJobOutput {
            arn: self.arn,
            creation_time: self.creation_time,
            export_status: self.export_status,
            execution_role_arn: self.execution_role_arn,
            output_config: self.output_config,
            export_source_images: self.export_source_images,
            _request_id: self._request_id,
        }
    }
}
