// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the input of a <code>DeletePipeline</code> action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeletePipelineInput {
    /// <p>The name of the pipeline to be deleted.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl DeletePipelineInput {
    /// <p>The name of the pipeline to be deleted.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl DeletePipelineInput {
    /// Creates a new builder-style object to manufacture [`DeletePipelineInput`](crate::operation::delete_pipeline::DeletePipelineInput).
    pub fn builder() -> crate::operation::delete_pipeline::builders::DeletePipelineInputBuilder {
        crate::operation::delete_pipeline::builders::DeletePipelineInputBuilder::default()
    }
}

/// A builder for [`DeletePipelineInput`](crate::operation::delete_pipeline::DeletePipelineInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeletePipelineInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl DeletePipelineInputBuilder {
    /// <p>The name of the pipeline to be deleted.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the pipeline to be deleted.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeletePipelineInput`](crate::operation::delete_pipeline::DeletePipelineInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_pipeline::DeletePipelineInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_pipeline::DeletePipelineInput {
            name: self.name,
        })
    }
}
