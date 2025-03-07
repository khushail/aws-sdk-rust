// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The tooltip.</p>
/// <p>This is a union type structure. For this structure to be valid, only one of the attributes can be defined.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TooltipItem {
    /// <p>The tooltip item for the fields.</p>
    #[doc(hidden)]
    pub field_tooltip_item: ::std::option::Option<crate::types::FieldTooltipItem>,
    /// <p>The tooltip item for the columns that are not part of a field well.</p>
    #[doc(hidden)]
    pub column_tooltip_item: ::std::option::Option<crate::types::ColumnTooltipItem>,
}
impl TooltipItem {
    /// <p>The tooltip item for the fields.</p>
    pub fn field_tooltip_item(&self) -> ::std::option::Option<&crate::types::FieldTooltipItem> {
        self.field_tooltip_item.as_ref()
    }
    /// <p>The tooltip item for the columns that are not part of a field well.</p>
    pub fn column_tooltip_item(&self) -> ::std::option::Option<&crate::types::ColumnTooltipItem> {
        self.column_tooltip_item.as_ref()
    }
}
impl TooltipItem {
    /// Creates a new builder-style object to manufacture [`TooltipItem`](crate::types::TooltipItem).
    pub fn builder() -> crate::types::builders::TooltipItemBuilder {
        crate::types::builders::TooltipItemBuilder::default()
    }
}

/// A builder for [`TooltipItem`](crate::types::TooltipItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TooltipItemBuilder {
    pub(crate) field_tooltip_item: ::std::option::Option<crate::types::FieldTooltipItem>,
    pub(crate) column_tooltip_item: ::std::option::Option<crate::types::ColumnTooltipItem>,
}
impl TooltipItemBuilder {
    /// <p>The tooltip item for the fields.</p>
    pub fn field_tooltip_item(mut self, input: crate::types::FieldTooltipItem) -> Self {
        self.field_tooltip_item = ::std::option::Option::Some(input);
        self
    }
    /// <p>The tooltip item for the fields.</p>
    pub fn set_field_tooltip_item(
        mut self,
        input: ::std::option::Option<crate::types::FieldTooltipItem>,
    ) -> Self {
        self.field_tooltip_item = input;
        self
    }
    /// <p>The tooltip item for the columns that are not part of a field well.</p>
    pub fn column_tooltip_item(mut self, input: crate::types::ColumnTooltipItem) -> Self {
        self.column_tooltip_item = ::std::option::Option::Some(input);
        self
    }
    /// <p>The tooltip item for the columns that are not part of a field well.</p>
    pub fn set_column_tooltip_item(
        mut self,
        input: ::std::option::Option<crate::types::ColumnTooltipItem>,
    ) -> Self {
        self.column_tooltip_item = input;
        self
    }
    /// Consumes the builder and constructs a [`TooltipItem`](crate::types::TooltipItem).
    pub fn build(self) -> crate::types::TooltipItem {
        crate::types::TooltipItem {
            field_tooltip_item: self.field_tooltip_item,
            column_tooltip_item: self.column_tooltip_item,
        }
    }
}
