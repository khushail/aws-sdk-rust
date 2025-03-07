// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The table calculation measure field for pivot tables.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CalculatedMeasureField {
    /// <p>The custom field ID.</p>
    #[doc(hidden)]
    pub field_id: ::std::option::Option<::std::string::String>,
    /// <p>The expression in the table calculation.</p>
    #[doc(hidden)]
    pub expression: ::std::option::Option<::std::string::String>,
}
impl CalculatedMeasureField {
    /// <p>The custom field ID.</p>
    pub fn field_id(&self) -> ::std::option::Option<&str> {
        self.field_id.as_deref()
    }
    /// <p>The expression in the table calculation.</p>
    pub fn expression(&self) -> ::std::option::Option<&str> {
        self.expression.as_deref()
    }
}
impl ::std::fmt::Debug for CalculatedMeasureField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CalculatedMeasureField");
        formatter.field("field_id", &self.field_id);
        formatter.field("expression", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl CalculatedMeasureField {
    /// Creates a new builder-style object to manufacture [`CalculatedMeasureField`](crate::types::CalculatedMeasureField).
    pub fn builder() -> crate::types::builders::CalculatedMeasureFieldBuilder {
        crate::types::builders::CalculatedMeasureFieldBuilder::default()
    }
}

/// A builder for [`CalculatedMeasureField`](crate::types::CalculatedMeasureField).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct CalculatedMeasureFieldBuilder {
    pub(crate) field_id: ::std::option::Option<::std::string::String>,
    pub(crate) expression: ::std::option::Option<::std::string::String>,
}
impl CalculatedMeasureFieldBuilder {
    /// <p>The custom field ID.</p>
    pub fn field_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.field_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The custom field ID.</p>
    pub fn set_field_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.field_id = input;
        self
    }
    /// <p>The expression in the table calculation.</p>
    pub fn expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expression = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The expression in the table calculation.</p>
    pub fn set_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expression = input;
        self
    }
    /// Consumes the builder and constructs a [`CalculatedMeasureField`](crate::types::CalculatedMeasureField).
    pub fn build(self) -> crate::types::CalculatedMeasureField {
        crate::types::CalculatedMeasureField {
            field_id: self.field_id,
            expression: self.expression,
        }
    }
}
impl ::std::fmt::Debug for CalculatedMeasureFieldBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CalculatedMeasureFieldBuilder");
        formatter.field("field_id", &self.field_id);
        formatter.field("expression", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
