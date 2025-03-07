// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetChannelInput {
    /// <p>Array of ARNs, one per channel.</p>
    #[doc(hidden)]
    pub arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl BatchGetChannelInput {
    /// <p>Array of ARNs, one per channel.</p>
    pub fn arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.arns.as_deref()
    }
}
impl BatchGetChannelInput {
    /// Creates a new builder-style object to manufacture [`BatchGetChannelInput`](crate::operation::batch_get_channel::BatchGetChannelInput).
    pub fn builder() -> crate::operation::batch_get_channel::builders::BatchGetChannelInputBuilder {
        crate::operation::batch_get_channel::builders::BatchGetChannelInputBuilder::default()
    }
}

/// A builder for [`BatchGetChannelInput`](crate::operation::batch_get_channel::BatchGetChannelInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchGetChannelInputBuilder {
    pub(crate) arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl BatchGetChannelInputBuilder {
    /// Appends an item to `arns`.
    ///
    /// To override the contents of this collection use [`set_arns`](Self::set_arns).
    ///
    /// <p>Array of ARNs, one per channel.</p>
    pub fn arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.arns.unwrap_or_default();
        v.push(input.into());
        self.arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>Array of ARNs, one per channel.</p>
    pub fn set_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.arns = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchGetChannelInput`](crate::operation::batch_get_channel::BatchGetChannelInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_get_channel::BatchGetChannelInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::batch_get_channel::BatchGetChannelInput {
            arns: self.arns,
        })
    }
}
