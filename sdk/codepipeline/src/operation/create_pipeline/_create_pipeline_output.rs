// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output of a <code>CreatePipeline</code> action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreatePipelineOutput {
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    #[doc(hidden)]
    pub pipeline: ::std::option::Option<crate::types::PipelineDeclaration>,
    /// <p>Specifies the tags applied to the pipeline.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl CreatePipelineOutput {
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    pub fn pipeline(&self) -> ::std::option::Option<&crate::types::PipelineDeclaration> {
        self.pipeline.as_ref()
    }
    /// <p>Specifies the tags applied to the pipeline.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreatePipelineOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreatePipelineOutput {
    /// Creates a new builder-style object to manufacture [`CreatePipelineOutput`](crate::operation::create_pipeline::CreatePipelineOutput).
    pub fn builder() -> crate::operation::create_pipeline::builders::CreatePipelineOutputBuilder {
        crate::operation::create_pipeline::builders::CreatePipelineOutputBuilder::default()
    }
}

/// A builder for [`CreatePipelineOutput`](crate::operation::create_pipeline::CreatePipelineOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreatePipelineOutputBuilder {
    pub(crate) pipeline: ::std::option::Option<crate::types::PipelineDeclaration>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl CreatePipelineOutputBuilder {
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    pub fn pipeline(mut self, input: crate::types::PipelineDeclaration) -> Self {
        self.pipeline = ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    pub fn set_pipeline(
        mut self,
        input: ::std::option::Option<crate::types::PipelineDeclaration>,
    ) -> Self {
        self.pipeline = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Specifies the tags applied to the pipeline.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the tags applied to the pipeline.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
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
    /// Consumes the builder and constructs a [`CreatePipelineOutput`](crate::operation::create_pipeline::CreatePipelineOutput).
    pub fn build(self) -> crate::operation::create_pipeline::CreatePipelineOutput {
        crate::operation::create_pipeline::CreatePipelineOutput {
            pipeline: self.pipeline,
            tags: self.tags,
            _request_id: self._request_id,
        }
    }
}
