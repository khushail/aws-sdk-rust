// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The series item configuration of a line chart.</p>
/// <p>This is a union type structure. For this structure to be valid, only one of the attributes can be defined.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SeriesItem {
    /// <p>The field series item configuration of a line chart.</p>
    #[doc(hidden)]
    pub field_series_item: ::std::option::Option<crate::types::FieldSeriesItem>,
    /// <p>The data field series item configuration of a line chart.</p>
    #[doc(hidden)]
    pub data_field_series_item: ::std::option::Option<crate::types::DataFieldSeriesItem>,
}
impl SeriesItem {
    /// <p>The field series item configuration of a line chart.</p>
    pub fn field_series_item(&self) -> ::std::option::Option<&crate::types::FieldSeriesItem> {
        self.field_series_item.as_ref()
    }
    /// <p>The data field series item configuration of a line chart.</p>
    pub fn data_field_series_item(
        &self,
    ) -> ::std::option::Option<&crate::types::DataFieldSeriesItem> {
        self.data_field_series_item.as_ref()
    }
}
impl SeriesItem {
    /// Creates a new builder-style object to manufacture [`SeriesItem`](crate::types::SeriesItem).
    pub fn builder() -> crate::types::builders::SeriesItemBuilder {
        crate::types::builders::SeriesItemBuilder::default()
    }
}

/// A builder for [`SeriesItem`](crate::types::SeriesItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SeriesItemBuilder {
    pub(crate) field_series_item: ::std::option::Option<crate::types::FieldSeriesItem>,
    pub(crate) data_field_series_item: ::std::option::Option<crate::types::DataFieldSeriesItem>,
}
impl SeriesItemBuilder {
    /// <p>The field series item configuration of a line chart.</p>
    pub fn field_series_item(mut self, input: crate::types::FieldSeriesItem) -> Self {
        self.field_series_item = ::std::option::Option::Some(input);
        self
    }
    /// <p>The field series item configuration of a line chart.</p>
    pub fn set_field_series_item(
        mut self,
        input: ::std::option::Option<crate::types::FieldSeriesItem>,
    ) -> Self {
        self.field_series_item = input;
        self
    }
    /// <p>The data field series item configuration of a line chart.</p>
    pub fn data_field_series_item(mut self, input: crate::types::DataFieldSeriesItem) -> Self {
        self.data_field_series_item = ::std::option::Option::Some(input);
        self
    }
    /// <p>The data field series item configuration of a line chart.</p>
    pub fn set_data_field_series_item(
        mut self,
        input: ::std::option::Option<crate::types::DataFieldSeriesItem>,
    ) -> Self {
        self.data_field_series_item = input;
        self
    }
    /// Consumes the builder and constructs a [`SeriesItem`](crate::types::SeriesItem).
    pub fn build(self) -> crate::types::SeriesItem {
        crate::types::SeriesItem {
            field_series_item: self.field_series_item,
            data_field_series_item: self.data_field_series_item,
        }
    }
}
