// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A description of the column in the query results.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ColumnDescription {
    /// <p>The name of the column description.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The type of the column description.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::ColumnType>,
}
impl ColumnDescription {
    /// <p>The name of the column description.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The type of the column description.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::ColumnType> {
        self.r#type.as_ref()
    }
}
impl ColumnDescription {
    /// Creates a new builder-style object to manufacture [`ColumnDescription`](crate::types::ColumnDescription).
    pub fn builder() -> crate::types::builders::ColumnDescriptionBuilder {
        crate::types::builders::ColumnDescriptionBuilder::default()
    }
}

/// A builder for [`ColumnDescription`](crate::types::ColumnDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ColumnDescriptionBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::ColumnType>,
}
impl ColumnDescriptionBuilder {
    /// <p>The name of the column description.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the column description.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The type of the column description.</p>
    pub fn r#type(mut self, input: crate::types::ColumnType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the column description.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ColumnType>) -> Self {
        self.r#type = input;
        self
    }
    /// Consumes the builder and constructs a [`ColumnDescription`](crate::types::ColumnDescription).
    pub fn build(self) -> crate::types::ColumnDescription {
        crate::types::ColumnDescription {
            name: self.name,
            r#type: self.r#type,
        }
    }
}
