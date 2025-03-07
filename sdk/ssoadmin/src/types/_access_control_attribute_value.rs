// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The value used for mapping a specified attribute to an identity source. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/attributemappingsconcept.html">Attribute mappings</a> in the <i>IAM Identity Center User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AccessControlAttributeValue {
    /// <p>The identity source to use when mapping a specified attribute to IAM Identity Center.</p>
    #[doc(hidden)]
    pub source: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AccessControlAttributeValue {
    /// <p>The identity source to use when mapping a specified attribute to IAM Identity Center.</p>
    pub fn source(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.source.as_deref()
    }
}
impl AccessControlAttributeValue {
    /// Creates a new builder-style object to manufacture [`AccessControlAttributeValue`](crate::types::AccessControlAttributeValue).
    pub fn builder() -> crate::types::builders::AccessControlAttributeValueBuilder {
        crate::types::builders::AccessControlAttributeValueBuilder::default()
    }
}

/// A builder for [`AccessControlAttributeValue`](crate::types::AccessControlAttributeValue).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AccessControlAttributeValueBuilder {
    pub(crate) source: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AccessControlAttributeValueBuilder {
    /// Appends an item to `source`.
    ///
    /// To override the contents of this collection use [`set_source`](Self::set_source).
    ///
    /// <p>The identity source to use when mapping a specified attribute to IAM Identity Center.</p>
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.source.unwrap_or_default();
        v.push(input.into());
        self.source = ::std::option::Option::Some(v);
        self
    }
    /// <p>The identity source to use when mapping a specified attribute to IAM Identity Center.</p>
    pub fn set_source(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.source = input;
        self
    }
    /// Consumes the builder and constructs a [`AccessControlAttributeValue`](crate::types::AccessControlAttributeValue).
    pub fn build(self) -> crate::types::AccessControlAttributeValue {
        crate::types::AccessControlAttributeValue {
            source: self.source,
        }
    }
}
