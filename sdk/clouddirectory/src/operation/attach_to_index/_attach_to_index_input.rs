// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AttachToIndexInput {
    /// <p>The Amazon Resource Name (ARN) of the directory where the object and index exist.</p>
    #[doc(hidden)]
    pub directory_arn: ::std::option::Option<::std::string::String>,
    /// <p>A reference to the index that you are attaching the object to.</p>
    #[doc(hidden)]
    pub index_reference: ::std::option::Option<crate::types::ObjectReference>,
    /// <p>A reference to the object that you are attaching to the index.</p>
    #[doc(hidden)]
    pub target_reference: ::std::option::Option<crate::types::ObjectReference>,
}
impl AttachToIndexInput {
    /// <p>The Amazon Resource Name (ARN) of the directory where the object and index exist.</p>
    pub fn directory_arn(&self) -> ::std::option::Option<&str> {
        self.directory_arn.as_deref()
    }
    /// <p>A reference to the index that you are attaching the object to.</p>
    pub fn index_reference(&self) -> ::std::option::Option<&crate::types::ObjectReference> {
        self.index_reference.as_ref()
    }
    /// <p>A reference to the object that you are attaching to the index.</p>
    pub fn target_reference(&self) -> ::std::option::Option<&crate::types::ObjectReference> {
        self.target_reference.as_ref()
    }
}
impl AttachToIndexInput {
    /// Creates a new builder-style object to manufacture [`AttachToIndexInput`](crate::operation::attach_to_index::AttachToIndexInput).
    pub fn builder() -> crate::operation::attach_to_index::builders::AttachToIndexInputBuilder {
        crate::operation::attach_to_index::builders::AttachToIndexInputBuilder::default()
    }
}

/// A builder for [`AttachToIndexInput`](crate::operation::attach_to_index::AttachToIndexInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AttachToIndexInputBuilder {
    pub(crate) directory_arn: ::std::option::Option<::std::string::String>,
    pub(crate) index_reference: ::std::option::Option<crate::types::ObjectReference>,
    pub(crate) target_reference: ::std::option::Option<crate::types::ObjectReference>,
}
impl AttachToIndexInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the directory where the object and index exist.</p>
    pub fn directory_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.directory_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the directory where the object and index exist.</p>
    pub fn set_directory_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.directory_arn = input;
        self
    }
    /// <p>A reference to the index that you are attaching the object to.</p>
    pub fn index_reference(mut self, input: crate::types::ObjectReference) -> Self {
        self.index_reference = ::std::option::Option::Some(input);
        self
    }
    /// <p>A reference to the index that you are attaching the object to.</p>
    pub fn set_index_reference(
        mut self,
        input: ::std::option::Option<crate::types::ObjectReference>,
    ) -> Self {
        self.index_reference = input;
        self
    }
    /// <p>A reference to the object that you are attaching to the index.</p>
    pub fn target_reference(mut self, input: crate::types::ObjectReference) -> Self {
        self.target_reference = ::std::option::Option::Some(input);
        self
    }
    /// <p>A reference to the object that you are attaching to the index.</p>
    pub fn set_target_reference(
        mut self,
        input: ::std::option::Option<crate::types::ObjectReference>,
    ) -> Self {
        self.target_reference = input;
        self
    }
    /// Consumes the builder and constructs a [`AttachToIndexInput`](crate::operation::attach_to_index::AttachToIndexInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::attach_to_index::AttachToIndexInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::attach_to_index::AttachToIndexInput {
            directory_arn: self.directory_arn,
            index_reference: self.index_reference,
            target_reference: self.target_reference,
        })
    }
}
