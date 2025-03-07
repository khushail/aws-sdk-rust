// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetAssetPropertyValueOutput {
    /// <p>A list of the errors (if any) associated with the batch request. Each error entry contains the <code>entryId</code> of the entry that failed.</p>
    #[doc(hidden)]
    pub error_entries:
        ::std::option::Option<::std::vec::Vec<crate::types::BatchGetAssetPropertyValueErrorEntry>>,
    /// <p>A list of entries that were processed successfully by this batch request. Each success entry contains the <code>entryId</code> of the entry that succeeded and the latest query result.</p>
    #[doc(hidden)]
    pub success_entries: ::std::option::Option<
        ::std::vec::Vec<crate::types::BatchGetAssetPropertyValueSuccessEntry>,
    >,
    /// <p>A list of entries that were not processed by this batch request. because these entries had been completely processed by previous paginated requests. Each skipped entry contains the <code>entryId</code> of the entry that skipped.</p>
    #[doc(hidden)]
    pub skipped_entries: ::std::option::Option<
        ::std::vec::Vec<crate::types::BatchGetAssetPropertyValueSkippedEntry>,
    >,
    /// <p>The token for the next set of results, or null if there are no additional results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl BatchGetAssetPropertyValueOutput {
    /// <p>A list of the errors (if any) associated with the batch request. Each error entry contains the <code>entryId</code> of the entry that failed.</p>
    pub fn error_entries(
        &self,
    ) -> ::std::option::Option<&[crate::types::BatchGetAssetPropertyValueErrorEntry]> {
        self.error_entries.as_deref()
    }
    /// <p>A list of entries that were processed successfully by this batch request. Each success entry contains the <code>entryId</code> of the entry that succeeded and the latest query result.</p>
    pub fn success_entries(
        &self,
    ) -> ::std::option::Option<&[crate::types::BatchGetAssetPropertyValueSuccessEntry]> {
        self.success_entries.as_deref()
    }
    /// <p>A list of entries that were not processed by this batch request. because these entries had been completely processed by previous paginated requests. Each skipped entry contains the <code>entryId</code> of the entry that skipped.</p>
    pub fn skipped_entries(
        &self,
    ) -> ::std::option::Option<&[crate::types::BatchGetAssetPropertyValueSkippedEntry]> {
        self.skipped_entries.as_deref()
    }
    /// <p>The token for the next set of results, or null if there are no additional results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for BatchGetAssetPropertyValueOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchGetAssetPropertyValueOutput {
    /// Creates a new builder-style object to manufacture [`BatchGetAssetPropertyValueOutput`](crate::operation::batch_get_asset_property_value::BatchGetAssetPropertyValueOutput).
    pub fn builder() -> crate::operation::batch_get_asset_property_value::builders::BatchGetAssetPropertyValueOutputBuilder{
        crate::operation::batch_get_asset_property_value::builders::BatchGetAssetPropertyValueOutputBuilder::default()
    }
}

/// A builder for [`BatchGetAssetPropertyValueOutput`](crate::operation::batch_get_asset_property_value::BatchGetAssetPropertyValueOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchGetAssetPropertyValueOutputBuilder {
    pub(crate) error_entries:
        ::std::option::Option<::std::vec::Vec<crate::types::BatchGetAssetPropertyValueErrorEntry>>,
    pub(crate) success_entries: ::std::option::Option<
        ::std::vec::Vec<crate::types::BatchGetAssetPropertyValueSuccessEntry>,
    >,
    pub(crate) skipped_entries: ::std::option::Option<
        ::std::vec::Vec<crate::types::BatchGetAssetPropertyValueSkippedEntry>,
    >,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl BatchGetAssetPropertyValueOutputBuilder {
    /// Appends an item to `error_entries`.
    ///
    /// To override the contents of this collection use [`set_error_entries`](Self::set_error_entries).
    ///
    /// <p>A list of the errors (if any) associated with the batch request. Each error entry contains the <code>entryId</code> of the entry that failed.</p>
    pub fn error_entries(
        mut self,
        input: crate::types::BatchGetAssetPropertyValueErrorEntry,
    ) -> Self {
        let mut v = self.error_entries.unwrap_or_default();
        v.push(input);
        self.error_entries = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the errors (if any) associated with the batch request. Each error entry contains the <code>entryId</code> of the entry that failed.</p>
    pub fn set_error_entries(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::BatchGetAssetPropertyValueErrorEntry>,
        >,
    ) -> Self {
        self.error_entries = input;
        self
    }
    /// Appends an item to `success_entries`.
    ///
    /// To override the contents of this collection use [`set_success_entries`](Self::set_success_entries).
    ///
    /// <p>A list of entries that were processed successfully by this batch request. Each success entry contains the <code>entryId</code> of the entry that succeeded and the latest query result.</p>
    pub fn success_entries(
        mut self,
        input: crate::types::BatchGetAssetPropertyValueSuccessEntry,
    ) -> Self {
        let mut v = self.success_entries.unwrap_or_default();
        v.push(input);
        self.success_entries = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of entries that were processed successfully by this batch request. Each success entry contains the <code>entryId</code> of the entry that succeeded and the latest query result.</p>
    pub fn set_success_entries(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::BatchGetAssetPropertyValueSuccessEntry>,
        >,
    ) -> Self {
        self.success_entries = input;
        self
    }
    /// Appends an item to `skipped_entries`.
    ///
    /// To override the contents of this collection use [`set_skipped_entries`](Self::set_skipped_entries).
    ///
    /// <p>A list of entries that were not processed by this batch request. because these entries had been completely processed by previous paginated requests. Each skipped entry contains the <code>entryId</code> of the entry that skipped.</p>
    pub fn skipped_entries(
        mut self,
        input: crate::types::BatchGetAssetPropertyValueSkippedEntry,
    ) -> Self {
        let mut v = self.skipped_entries.unwrap_or_default();
        v.push(input);
        self.skipped_entries = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of entries that were not processed by this batch request. because these entries had been completely processed by previous paginated requests. Each skipped entry contains the <code>entryId</code> of the entry that skipped.</p>
    pub fn set_skipped_entries(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::BatchGetAssetPropertyValueSkippedEntry>,
        >,
    ) -> Self {
        self.skipped_entries = input;
        self
    }
    /// <p>The token for the next set of results, or null if there are no additional results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of results, or null if there are no additional results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`BatchGetAssetPropertyValueOutput`](crate::operation::batch_get_asset_property_value::BatchGetAssetPropertyValueOutput).
    pub fn build(
        self,
    ) -> crate::operation::batch_get_asset_property_value::BatchGetAssetPropertyValueOutput {
        crate::operation::batch_get_asset_property_value::BatchGetAssetPropertyValueOutput {
            error_entries: self.error_entries,
            success_entries: self.success_entries,
            skipped_entries: self.skipped_entries,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
