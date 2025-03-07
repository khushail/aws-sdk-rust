// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The <code>DeletePipelineRequest</code> structure.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeletePipelineInput {
    /// <p>The identifier of the pipeline that you want to delete.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl DeletePipelineInput {
    /// <p>The identifier of the pipeline that you want to delete.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
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
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl DeletePipelineInputBuilder {
    /// <p>The identifier of the pipeline that you want to delete.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the pipeline that you want to delete.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
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
            id: self.id,
        })
    }
}
