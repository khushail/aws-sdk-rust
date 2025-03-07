// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetLinkAttributesInput {
    /// <p>The Amazon Resource Name (ARN) that is associated with the Directory where the typed link resides. For more information, see <code>arns</code> or <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    #[doc(hidden)]
    pub directory_arn: ::std::option::Option<::std::string::String>,
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    #[doc(hidden)]
    pub typed_link_specifier: ::std::option::Option<crate::types::TypedLinkSpecifier>,
    /// <p>A list of attribute names whose values will be retrieved.</p>
    #[doc(hidden)]
    pub attribute_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The consistency level at which to retrieve the attributes on a typed link.</p>
    #[doc(hidden)]
    pub consistency_level: ::std::option::Option<crate::types::ConsistencyLevel>,
}
impl GetLinkAttributesInput {
    /// <p>The Amazon Resource Name (ARN) that is associated with the Directory where the typed link resides. For more information, see <code>arns</code> or <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    pub fn directory_arn(&self) -> ::std::option::Option<&str> {
        self.directory_arn.as_deref()
    }
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    pub fn typed_link_specifier(&self) -> ::std::option::Option<&crate::types::TypedLinkSpecifier> {
        self.typed_link_specifier.as_ref()
    }
    /// <p>A list of attribute names whose values will be retrieved.</p>
    pub fn attribute_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.attribute_names.as_deref()
    }
    /// <p>The consistency level at which to retrieve the attributes on a typed link.</p>
    pub fn consistency_level(&self) -> ::std::option::Option<&crate::types::ConsistencyLevel> {
        self.consistency_level.as_ref()
    }
}
impl GetLinkAttributesInput {
    /// Creates a new builder-style object to manufacture [`GetLinkAttributesInput`](crate::operation::get_link_attributes::GetLinkAttributesInput).
    pub fn builder(
    ) -> crate::operation::get_link_attributes::builders::GetLinkAttributesInputBuilder {
        crate::operation::get_link_attributes::builders::GetLinkAttributesInputBuilder::default()
    }
}

/// A builder for [`GetLinkAttributesInput`](crate::operation::get_link_attributes::GetLinkAttributesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetLinkAttributesInputBuilder {
    pub(crate) directory_arn: ::std::option::Option<::std::string::String>,
    pub(crate) typed_link_specifier: ::std::option::Option<crate::types::TypedLinkSpecifier>,
    pub(crate) attribute_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) consistency_level: ::std::option::Option<crate::types::ConsistencyLevel>,
}
impl GetLinkAttributesInputBuilder {
    /// <p>The Amazon Resource Name (ARN) that is associated with the Directory where the typed link resides. For more information, see <code>arns</code> or <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    pub fn directory_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.directory_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that is associated with the Directory where the typed link resides. For more information, see <code>arns</code> or <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    pub fn set_directory_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.directory_arn = input;
        self
    }
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    pub fn typed_link_specifier(mut self, input: crate::types::TypedLinkSpecifier) -> Self {
        self.typed_link_specifier = ::std::option::Option::Some(input);
        self
    }
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    pub fn set_typed_link_specifier(
        mut self,
        input: ::std::option::Option<crate::types::TypedLinkSpecifier>,
    ) -> Self {
        self.typed_link_specifier = input;
        self
    }
    /// Appends an item to `attribute_names`.
    ///
    /// To override the contents of this collection use [`set_attribute_names`](Self::set_attribute_names).
    ///
    /// <p>A list of attribute names whose values will be retrieved.</p>
    pub fn attribute_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.attribute_names.unwrap_or_default();
        v.push(input.into());
        self.attribute_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of attribute names whose values will be retrieved.</p>
    pub fn set_attribute_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.attribute_names = input;
        self
    }
    /// <p>The consistency level at which to retrieve the attributes on a typed link.</p>
    pub fn consistency_level(mut self, input: crate::types::ConsistencyLevel) -> Self {
        self.consistency_level = ::std::option::Option::Some(input);
        self
    }
    /// <p>The consistency level at which to retrieve the attributes on a typed link.</p>
    pub fn set_consistency_level(
        mut self,
        input: ::std::option::Option<crate::types::ConsistencyLevel>,
    ) -> Self {
        self.consistency_level = input;
        self
    }
    /// Consumes the builder and constructs a [`GetLinkAttributesInput`](crate::operation::get_link_attributes::GetLinkAttributesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_link_attributes::GetLinkAttributesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_link_attributes::GetLinkAttributesInput {
                directory_arn: self.directory_arn,
                typed_link_specifier: self.typed_link_specifier,
                attribute_names: self.attribute_names,
                consistency_level: self.consistency_level,
            },
        )
    }
}
