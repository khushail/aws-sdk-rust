// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SourceSchema {
    /// <p>Specifies the format of the records on the streaming source.</p>
    #[doc(hidden)]
    pub record_format: ::std::option::Option<crate::types::RecordFormat>,
    /// <p>Specifies the encoding of the records in the streaming source. For example, UTF-8.</p>
    #[doc(hidden)]
    pub record_encoding: ::std::option::Option<::std::string::String>,
    /// <p>A list of <code>RecordColumn</code> objects.</p>
    #[doc(hidden)]
    pub record_columns: ::std::option::Option<::std::vec::Vec<crate::types::RecordColumn>>,
}
impl SourceSchema {
    /// <p>Specifies the format of the records on the streaming source.</p>
    pub fn record_format(&self) -> ::std::option::Option<&crate::types::RecordFormat> {
        self.record_format.as_ref()
    }
    /// <p>Specifies the encoding of the records in the streaming source. For example, UTF-8.</p>
    pub fn record_encoding(&self) -> ::std::option::Option<&str> {
        self.record_encoding.as_deref()
    }
    /// <p>A list of <code>RecordColumn</code> objects.</p>
    pub fn record_columns(&self) -> ::std::option::Option<&[crate::types::RecordColumn]> {
        self.record_columns.as_deref()
    }
}
impl SourceSchema {
    /// Creates a new builder-style object to manufacture [`SourceSchema`](crate::types::SourceSchema).
    pub fn builder() -> crate::types::builders::SourceSchemaBuilder {
        crate::types::builders::SourceSchemaBuilder::default()
    }
}

/// A builder for [`SourceSchema`](crate::types::SourceSchema).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SourceSchemaBuilder {
    pub(crate) record_format: ::std::option::Option<crate::types::RecordFormat>,
    pub(crate) record_encoding: ::std::option::Option<::std::string::String>,
    pub(crate) record_columns: ::std::option::Option<::std::vec::Vec<crate::types::RecordColumn>>,
}
impl SourceSchemaBuilder {
    /// <p>Specifies the format of the records on the streaming source.</p>
    pub fn record_format(mut self, input: crate::types::RecordFormat) -> Self {
        self.record_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the format of the records on the streaming source.</p>
    pub fn set_record_format(
        mut self,
        input: ::std::option::Option<crate::types::RecordFormat>,
    ) -> Self {
        self.record_format = input;
        self
    }
    /// <p>Specifies the encoding of the records in the streaming source. For example, UTF-8.</p>
    pub fn record_encoding(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.record_encoding = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the encoding of the records in the streaming source. For example, UTF-8.</p>
    pub fn set_record_encoding(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.record_encoding = input;
        self
    }
    /// Appends an item to `record_columns`.
    ///
    /// To override the contents of this collection use [`set_record_columns`](Self::set_record_columns).
    ///
    /// <p>A list of <code>RecordColumn</code> objects.</p>
    pub fn record_columns(mut self, input: crate::types::RecordColumn) -> Self {
        let mut v = self.record_columns.unwrap_or_default();
        v.push(input);
        self.record_columns = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>RecordColumn</code> objects.</p>
    pub fn set_record_columns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RecordColumn>>,
    ) -> Self {
        self.record_columns = input;
        self
    }
    /// Consumes the builder and constructs a [`SourceSchema`](crate::types::SourceSchema).
    pub fn build(self) -> crate::types::SourceSchema {
        crate::types::SourceSchema {
            record_format: self.record_format,
            record_encoding: self.record_encoding,
            record_columns: self.record_columns,
        }
    }
}
