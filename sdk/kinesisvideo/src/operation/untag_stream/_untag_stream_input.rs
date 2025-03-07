// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UntagStreamInput {
    /// <p>The Amazon Resource Name (ARN) of the stream that you want to remove tags from.</p>
    #[doc(hidden)]
    pub stream_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the stream that you want to remove tags from.</p>
    #[doc(hidden)]
    pub stream_name: ::std::option::Option<::std::string::String>,
    /// <p>A list of the keys of the tags that you want to remove.</p>
    #[doc(hidden)]
    pub tag_key_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UntagStreamInput {
    /// <p>The Amazon Resource Name (ARN) of the stream that you want to remove tags from.</p>
    pub fn stream_arn(&self) -> ::std::option::Option<&str> {
        self.stream_arn.as_deref()
    }
    /// <p>The name of the stream that you want to remove tags from.</p>
    pub fn stream_name(&self) -> ::std::option::Option<&str> {
        self.stream_name.as_deref()
    }
    /// <p>A list of the keys of the tags that you want to remove.</p>
    pub fn tag_key_list(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.tag_key_list.as_deref()
    }
}
impl UntagStreamInput {
    /// Creates a new builder-style object to manufacture [`UntagStreamInput`](crate::operation::untag_stream::UntagStreamInput).
    pub fn builder() -> crate::operation::untag_stream::builders::UntagStreamInputBuilder {
        crate::operation::untag_stream::builders::UntagStreamInputBuilder::default()
    }
}

/// A builder for [`UntagStreamInput`](crate::operation::untag_stream::UntagStreamInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UntagStreamInputBuilder {
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
    pub(crate) stream_name: ::std::option::Option<::std::string::String>,
    pub(crate) tag_key_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UntagStreamInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the stream that you want to remove tags from.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the stream that you want to remove tags from.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_arn = input;
        self
    }
    /// <p>The name of the stream that you want to remove tags from.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the stream that you want to remove tags from.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_name = input;
        self
    }
    /// Appends an item to `tag_key_list`.
    ///
    /// To override the contents of this collection use [`set_tag_key_list`](Self::set_tag_key_list).
    ///
    /// <p>A list of the keys of the tags that you want to remove.</p>
    pub fn tag_key_list(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.tag_key_list.unwrap_or_default();
        v.push(input.into());
        self.tag_key_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the keys of the tags that you want to remove.</p>
    pub fn set_tag_key_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.tag_key_list = input;
        self
    }
    /// Consumes the builder and constructs a [`UntagStreamInput`](crate::operation::untag_stream::UntagStreamInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::untag_stream::UntagStreamInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::untag_stream::UntagStreamInput {
            stream_arn: self.stream_arn,
            stream_name: self.stream_name,
            tag_key_list: self.tag_key_list,
        })
    }
}
