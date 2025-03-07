// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RunPipelineActivityInput {
    /// <p>The pipeline activity that is run. This must not be a channel activity or a data store activity because these activities are used in a pipeline only to load the original message and to store the (possibly) transformed message. If a Lambda activity is specified, only short-running Lambda functions (those with a timeout of less than 30 seconds or less) can be used.</p>
    #[doc(hidden)]
    pub pipeline_activity: ::std::option::Option<crate::types::PipelineActivity>,
    /// <p>The sample message payloads on which the pipeline activity is run.</p>
    #[doc(hidden)]
    pub payloads: ::std::option::Option<::std::vec::Vec<::aws_smithy_types::Blob>>,
}
impl RunPipelineActivityInput {
    /// <p>The pipeline activity that is run. This must not be a channel activity or a data store activity because these activities are used in a pipeline only to load the original message and to store the (possibly) transformed message. If a Lambda activity is specified, only short-running Lambda functions (those with a timeout of less than 30 seconds or less) can be used.</p>
    pub fn pipeline_activity(&self) -> ::std::option::Option<&crate::types::PipelineActivity> {
        self.pipeline_activity.as_ref()
    }
    /// <p>The sample message payloads on which the pipeline activity is run.</p>
    pub fn payloads(&self) -> ::std::option::Option<&[::aws_smithy_types::Blob]> {
        self.payloads.as_deref()
    }
}
impl RunPipelineActivityInput {
    /// Creates a new builder-style object to manufacture [`RunPipelineActivityInput`](crate::operation::run_pipeline_activity::RunPipelineActivityInput).
    pub fn builder(
    ) -> crate::operation::run_pipeline_activity::builders::RunPipelineActivityInputBuilder {
        crate::operation::run_pipeline_activity::builders::RunPipelineActivityInputBuilder::default(
        )
    }
}

/// A builder for [`RunPipelineActivityInput`](crate::operation::run_pipeline_activity::RunPipelineActivityInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RunPipelineActivityInputBuilder {
    pub(crate) pipeline_activity: ::std::option::Option<crate::types::PipelineActivity>,
    pub(crate) payloads: ::std::option::Option<::std::vec::Vec<::aws_smithy_types::Blob>>,
}
impl RunPipelineActivityInputBuilder {
    /// <p>The pipeline activity that is run. This must not be a channel activity or a data store activity because these activities are used in a pipeline only to load the original message and to store the (possibly) transformed message. If a Lambda activity is specified, only short-running Lambda functions (those with a timeout of less than 30 seconds or less) can be used.</p>
    pub fn pipeline_activity(mut self, input: crate::types::PipelineActivity) -> Self {
        self.pipeline_activity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The pipeline activity that is run. This must not be a channel activity or a data store activity because these activities are used in a pipeline only to load the original message and to store the (possibly) transformed message. If a Lambda activity is specified, only short-running Lambda functions (those with a timeout of less than 30 seconds or less) can be used.</p>
    pub fn set_pipeline_activity(
        mut self,
        input: ::std::option::Option<crate::types::PipelineActivity>,
    ) -> Self {
        self.pipeline_activity = input;
        self
    }
    /// Appends an item to `payloads`.
    ///
    /// To override the contents of this collection use [`set_payloads`](Self::set_payloads).
    ///
    /// <p>The sample message payloads on which the pipeline activity is run.</p>
    pub fn payloads(mut self, input: ::aws_smithy_types::Blob) -> Self {
        let mut v = self.payloads.unwrap_or_default();
        v.push(input);
        self.payloads = ::std::option::Option::Some(v);
        self
    }
    /// <p>The sample message payloads on which the pipeline activity is run.</p>
    pub fn set_payloads(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::aws_smithy_types::Blob>>,
    ) -> Self {
        self.payloads = input;
        self
    }
    /// Consumes the builder and constructs a [`RunPipelineActivityInput`](crate::operation::run_pipeline_activity::RunPipelineActivityInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::run_pipeline_activity::RunPipelineActivityInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::run_pipeline_activity::RunPipelineActivityInput {
                pipeline_activity: self.pipeline_activity,
                payloads: self.payloads,
            },
        )
    }
}
