// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTagsForResourceOutput {
    /// <p>A collection of key and value pairs.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl ListTagsForResourceOutput {
    /// <p>A collection of key and value pairs.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListTagsForResourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput).
    pub fn builder(
    ) -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder {
        crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder::default()
    }
}

/// A builder for [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTagsForResourceOutputBuilder {
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl ListTagsForResourceOutputBuilder {
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A collection of key and value pairs.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>A collection of key and value pairs.</p>
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
    /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput).
    pub fn build(self) -> crate::operation::list_tags_for_resource::ListTagsForResourceOutput {
        crate::operation::list_tags_for_resource::ListTagsForResourceOutput {
            tags: self.tags,
            _request_id: self._request_id,
        }
    }
}
