// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output of a <code>GetLinkAttributes</code> response operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetLinkAttributesResponse {
    /// <p>The attributes that are associated with the typed link.</p>
    #[doc(hidden)]
    pub attributes: ::std::option::Option<::std::vec::Vec<crate::types::AttributeKeyAndValue>>,
}
impl BatchGetLinkAttributesResponse {
    /// <p>The attributes that are associated with the typed link.</p>
    pub fn attributes(&self) -> ::std::option::Option<&[crate::types::AttributeKeyAndValue]> {
        self.attributes.as_deref()
    }
}
impl BatchGetLinkAttributesResponse {
    /// Creates a new builder-style object to manufacture [`BatchGetLinkAttributesResponse`](crate::types::BatchGetLinkAttributesResponse).
    pub fn builder() -> crate::types::builders::BatchGetLinkAttributesResponseBuilder {
        crate::types::builders::BatchGetLinkAttributesResponseBuilder::default()
    }
}

/// A builder for [`BatchGetLinkAttributesResponse`](crate::types::BatchGetLinkAttributesResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchGetLinkAttributesResponseBuilder {
    pub(crate) attributes:
        ::std::option::Option<::std::vec::Vec<crate::types::AttributeKeyAndValue>>,
}
impl BatchGetLinkAttributesResponseBuilder {
    /// Appends an item to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>The attributes that are associated with the typed link.</p>
    pub fn attributes(mut self, input: crate::types::AttributeKeyAndValue) -> Self {
        let mut v = self.attributes.unwrap_or_default();
        v.push(input);
        self.attributes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The attributes that are associated with the typed link.</p>
    pub fn set_attributes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AttributeKeyAndValue>>,
    ) -> Self {
        self.attributes = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchGetLinkAttributesResponse`](crate::types::BatchGetLinkAttributesResponse).
    pub fn build(self) -> crate::types::BatchGetLinkAttributesResponse {
        crate::types::BatchGetLinkAttributesResponse {
            attributes: self.attributes,
        }
    }
}
