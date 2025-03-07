// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The tick label options of an axis.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AxisTickLabelOptions {
    /// <p>Determines whether or not the axis ticks are visible.</p>
    #[doc(hidden)]
    pub label_options: ::std::option::Option<crate::types::LabelOptions>,
    /// <p>The rotation angle of the axis tick labels.</p>
    #[doc(hidden)]
    pub rotation_angle: ::std::option::Option<f64>,
}
impl AxisTickLabelOptions {
    /// <p>Determines whether or not the axis ticks are visible.</p>
    pub fn label_options(&self) -> ::std::option::Option<&crate::types::LabelOptions> {
        self.label_options.as_ref()
    }
    /// <p>The rotation angle of the axis tick labels.</p>
    pub fn rotation_angle(&self) -> ::std::option::Option<f64> {
        self.rotation_angle
    }
}
impl AxisTickLabelOptions {
    /// Creates a new builder-style object to manufacture [`AxisTickLabelOptions`](crate::types::AxisTickLabelOptions).
    pub fn builder() -> crate::types::builders::AxisTickLabelOptionsBuilder {
        crate::types::builders::AxisTickLabelOptionsBuilder::default()
    }
}

/// A builder for [`AxisTickLabelOptions`](crate::types::AxisTickLabelOptions).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AxisTickLabelOptionsBuilder {
    pub(crate) label_options: ::std::option::Option<crate::types::LabelOptions>,
    pub(crate) rotation_angle: ::std::option::Option<f64>,
}
impl AxisTickLabelOptionsBuilder {
    /// <p>Determines whether or not the axis ticks are visible.</p>
    pub fn label_options(mut self, input: crate::types::LabelOptions) -> Self {
        self.label_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Determines whether or not the axis ticks are visible.</p>
    pub fn set_label_options(
        mut self,
        input: ::std::option::Option<crate::types::LabelOptions>,
    ) -> Self {
        self.label_options = input;
        self
    }
    /// <p>The rotation angle of the axis tick labels.</p>
    pub fn rotation_angle(mut self, input: f64) -> Self {
        self.rotation_angle = ::std::option::Option::Some(input);
        self
    }
    /// <p>The rotation angle of the axis tick labels.</p>
    pub fn set_rotation_angle(mut self, input: ::std::option::Option<f64>) -> Self {
        self.rotation_angle = input;
        self
    }
    /// Consumes the builder and constructs a [`AxisTickLabelOptions`](crate::types::AxisTickLabelOptions).
    pub fn build(self) -> crate::types::AxisTickLabelOptions {
        crate::types::AxisTickLabelOptions {
            label_options: self.label_options,
            rotation_angle: self.rotation_angle,
        }
    }
}
