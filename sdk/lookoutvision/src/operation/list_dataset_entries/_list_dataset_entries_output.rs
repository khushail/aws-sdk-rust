// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListDatasetEntriesOutput {
    /// <p>A list of the entries (JSON Lines) within the dataset.</p>
    #[doc(hidden)]
    pub dataset_entries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>If the response is truncated, Amazon Lookout for Vision returns this token that you can use in the subsequent request to retrieve the next set ofdataset entries.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListDatasetEntriesOutput {
    /// <p>A list of the entries (JSON Lines) within the dataset.</p>
    pub fn dataset_entries(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.dataset_entries.as_deref()
    }
    /// <p>If the response is truncated, Amazon Lookout for Vision returns this token that you can use in the subsequent request to retrieve the next set ofdataset entries.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListDatasetEntriesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListDatasetEntriesOutput {
    /// Creates a new builder-style object to manufacture [`ListDatasetEntriesOutput`](crate::operation::list_dataset_entries::ListDatasetEntriesOutput).
    pub fn builder(
    ) -> crate::operation::list_dataset_entries::builders::ListDatasetEntriesOutputBuilder {
        crate::operation::list_dataset_entries::builders::ListDatasetEntriesOutputBuilder::default()
    }
}

/// A builder for [`ListDatasetEntriesOutput`](crate::operation::list_dataset_entries::ListDatasetEntriesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListDatasetEntriesOutputBuilder {
    pub(crate) dataset_entries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListDatasetEntriesOutputBuilder {
    /// Appends an item to `dataset_entries`.
    ///
    /// To override the contents of this collection use [`set_dataset_entries`](Self::set_dataset_entries).
    ///
    /// <p>A list of the entries (JSON Lines) within the dataset.</p>
    pub fn dataset_entries(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.dataset_entries.unwrap_or_default();
        v.push(input.into());
        self.dataset_entries = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the entries (JSON Lines) within the dataset.</p>
    pub fn set_dataset_entries(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.dataset_entries = input;
        self
    }
    /// <p>If the response is truncated, Amazon Lookout for Vision returns this token that you can use in the subsequent request to retrieve the next set ofdataset entries.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the response is truncated, Amazon Lookout for Vision returns this token that you can use in the subsequent request to retrieve the next set ofdataset entries.</p>
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
    /// Consumes the builder and constructs a [`ListDatasetEntriesOutput`](crate::operation::list_dataset_entries::ListDatasetEntriesOutput).
    pub fn build(self) -> crate::operation::list_dataset_entries::ListDatasetEntriesOutput {
        crate::operation::list_dataset_entries::ListDatasetEntriesOutput {
            dataset_entries: self.dataset_entries,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
