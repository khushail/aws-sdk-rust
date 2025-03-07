// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetDocumentStatusInput {
    /// <p>The identifier of the index to add documents to. The index ID is returned by the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_CreateIndex.html">CreateIndex </a> API.</p>
    #[doc(hidden)]
    pub index_id: ::std::option::Option<::std::string::String>,
    /// <p>A list of <code>DocumentInfo</code> objects that identify the documents for which to get the status. You identify the documents by their document ID and optional attributes.</p>
    #[doc(hidden)]
    pub document_info_list: ::std::option::Option<::std::vec::Vec<crate::types::DocumentInfo>>,
}
impl BatchGetDocumentStatusInput {
    /// <p>The identifier of the index to add documents to. The index ID is returned by the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_CreateIndex.html">CreateIndex </a> API.</p>
    pub fn index_id(&self) -> ::std::option::Option<&str> {
        self.index_id.as_deref()
    }
    /// <p>A list of <code>DocumentInfo</code> objects that identify the documents for which to get the status. You identify the documents by their document ID and optional attributes.</p>
    pub fn document_info_list(&self) -> ::std::option::Option<&[crate::types::DocumentInfo]> {
        self.document_info_list.as_deref()
    }
}
impl BatchGetDocumentStatusInput {
    /// Creates a new builder-style object to manufacture [`BatchGetDocumentStatusInput`](crate::operation::batch_get_document_status::BatchGetDocumentStatusInput).
    pub fn builder(
    ) -> crate::operation::batch_get_document_status::builders::BatchGetDocumentStatusInputBuilder
    {
        crate::operation::batch_get_document_status::builders::BatchGetDocumentStatusInputBuilder::default()
    }
}

/// A builder for [`BatchGetDocumentStatusInput`](crate::operation::batch_get_document_status::BatchGetDocumentStatusInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchGetDocumentStatusInputBuilder {
    pub(crate) index_id: ::std::option::Option<::std::string::String>,
    pub(crate) document_info_list:
        ::std::option::Option<::std::vec::Vec<crate::types::DocumentInfo>>,
}
impl BatchGetDocumentStatusInputBuilder {
    /// <p>The identifier of the index to add documents to. The index ID is returned by the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_CreateIndex.html">CreateIndex </a> API.</p>
    pub fn index_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the index to add documents to. The index ID is returned by the <a href="https://docs.aws.amazon.com/kendra/latest/dg/API_CreateIndex.html">CreateIndex </a> API.</p>
    pub fn set_index_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_id = input;
        self
    }
    /// Appends an item to `document_info_list`.
    ///
    /// To override the contents of this collection use [`set_document_info_list`](Self::set_document_info_list).
    ///
    /// <p>A list of <code>DocumentInfo</code> objects that identify the documents for which to get the status. You identify the documents by their document ID and optional attributes.</p>
    pub fn document_info_list(mut self, input: crate::types::DocumentInfo) -> Self {
        let mut v = self.document_info_list.unwrap_or_default();
        v.push(input);
        self.document_info_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>DocumentInfo</code> objects that identify the documents for which to get the status. You identify the documents by their document ID and optional attributes.</p>
    pub fn set_document_info_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DocumentInfo>>,
    ) -> Self {
        self.document_info_list = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchGetDocumentStatusInput`](crate::operation::batch_get_document_status::BatchGetDocumentStatusInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_get_document_status::BatchGetDocumentStatusInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::batch_get_document_status::BatchGetDocumentStatusInput {
                index_id: self.index_id,
                document_info_list: self.document_info_list,
            },
        )
    }
}
