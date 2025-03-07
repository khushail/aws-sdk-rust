// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The display options for the layout of tiles on a sheet.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TileLayoutStyle {
    /// <p>The gutter settings that apply between tiles. </p>
    #[doc(hidden)]
    pub gutter: ::std::option::Option<crate::types::GutterStyle>,
    /// <p>The margin settings that apply around the outside edge of sheets.</p>
    #[doc(hidden)]
    pub margin: ::std::option::Option<crate::types::MarginStyle>,
}
impl TileLayoutStyle {
    /// <p>The gutter settings that apply between tiles. </p>
    pub fn gutter(&self) -> ::std::option::Option<&crate::types::GutterStyle> {
        self.gutter.as_ref()
    }
    /// <p>The margin settings that apply around the outside edge of sheets.</p>
    pub fn margin(&self) -> ::std::option::Option<&crate::types::MarginStyle> {
        self.margin.as_ref()
    }
}
impl TileLayoutStyle {
    /// Creates a new builder-style object to manufacture [`TileLayoutStyle`](crate::types::TileLayoutStyle).
    pub fn builder() -> crate::types::builders::TileLayoutStyleBuilder {
        crate::types::builders::TileLayoutStyleBuilder::default()
    }
}

/// A builder for [`TileLayoutStyle`](crate::types::TileLayoutStyle).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TileLayoutStyleBuilder {
    pub(crate) gutter: ::std::option::Option<crate::types::GutterStyle>,
    pub(crate) margin: ::std::option::Option<crate::types::MarginStyle>,
}
impl TileLayoutStyleBuilder {
    /// <p>The gutter settings that apply between tiles. </p>
    pub fn gutter(mut self, input: crate::types::GutterStyle) -> Self {
        self.gutter = ::std::option::Option::Some(input);
        self
    }
    /// <p>The gutter settings that apply between tiles. </p>
    pub fn set_gutter(mut self, input: ::std::option::Option<crate::types::GutterStyle>) -> Self {
        self.gutter = input;
        self
    }
    /// <p>The margin settings that apply around the outside edge of sheets.</p>
    pub fn margin(mut self, input: crate::types::MarginStyle) -> Self {
        self.margin = ::std::option::Option::Some(input);
        self
    }
    /// <p>The margin settings that apply around the outside edge of sheets.</p>
    pub fn set_margin(mut self, input: ::std::option::Option<crate::types::MarginStyle>) -> Self {
        self.margin = input;
        self
    }
    /// Consumes the builder and constructs a [`TileLayoutStyle`](crate::types::TileLayoutStyle).
    pub fn build(self) -> crate::types::TileLayoutStyle {
        crate::types::TileLayoutStyle {
            gutter: self.gutter,
            margin: self.margin,
        }
    }
}
