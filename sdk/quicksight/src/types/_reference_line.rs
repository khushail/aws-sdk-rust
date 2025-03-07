// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The reference line visual display options.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReferenceLine {
    /// <p>The status of the reference line. Choose one of the following options:</p>
    /// <ul>
    /// <li> <p> <code>ENABLE</code> </p> </li>
    /// <li> <p> <code>DISABLE</code> </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::WidgetStatus>,
    /// <p>The data configuration of the reference line.</p>
    #[doc(hidden)]
    pub data_configuration: ::std::option::Option<crate::types::ReferenceLineDataConfiguration>,
    /// <p>The style configuration of the reference line.</p>
    #[doc(hidden)]
    pub style_configuration: ::std::option::Option<crate::types::ReferenceLineStyleConfiguration>,
    /// <p>The label configuration of the reference line.</p>
    #[doc(hidden)]
    pub label_configuration: ::std::option::Option<crate::types::ReferenceLineLabelConfiguration>,
}
impl ReferenceLine {
    /// <p>The status of the reference line. Choose one of the following options:</p>
    /// <ul>
    /// <li> <p> <code>ENABLE</code> </p> </li>
    /// <li> <p> <code>DISABLE</code> </p> </li>
    /// </ul>
    pub fn status(&self) -> ::std::option::Option<&crate::types::WidgetStatus> {
        self.status.as_ref()
    }
    /// <p>The data configuration of the reference line.</p>
    pub fn data_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ReferenceLineDataConfiguration> {
        self.data_configuration.as_ref()
    }
    /// <p>The style configuration of the reference line.</p>
    pub fn style_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ReferenceLineStyleConfiguration> {
        self.style_configuration.as_ref()
    }
    /// <p>The label configuration of the reference line.</p>
    pub fn label_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ReferenceLineLabelConfiguration> {
        self.label_configuration.as_ref()
    }
}
impl ReferenceLine {
    /// Creates a new builder-style object to manufacture [`ReferenceLine`](crate::types::ReferenceLine).
    pub fn builder() -> crate::types::builders::ReferenceLineBuilder {
        crate::types::builders::ReferenceLineBuilder::default()
    }
}

/// A builder for [`ReferenceLine`](crate::types::ReferenceLine).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReferenceLineBuilder {
    pub(crate) status: ::std::option::Option<crate::types::WidgetStatus>,
    pub(crate) data_configuration:
        ::std::option::Option<crate::types::ReferenceLineDataConfiguration>,
    pub(crate) style_configuration:
        ::std::option::Option<crate::types::ReferenceLineStyleConfiguration>,
    pub(crate) label_configuration:
        ::std::option::Option<crate::types::ReferenceLineLabelConfiguration>,
}
impl ReferenceLineBuilder {
    /// <p>The status of the reference line. Choose one of the following options:</p>
    /// <ul>
    /// <li> <p> <code>ENABLE</code> </p> </li>
    /// <li> <p> <code>DISABLE</code> </p> </li>
    /// </ul>
    pub fn status(mut self, input: crate::types::WidgetStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the reference line. Choose one of the following options:</p>
    /// <ul>
    /// <li> <p> <code>ENABLE</code> </p> </li>
    /// <li> <p> <code>DISABLE</code> </p> </li>
    /// </ul>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::WidgetStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The data configuration of the reference line.</p>
    pub fn data_configuration(
        mut self,
        input: crate::types::ReferenceLineDataConfiguration,
    ) -> Self {
        self.data_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The data configuration of the reference line.</p>
    pub fn set_data_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ReferenceLineDataConfiguration>,
    ) -> Self {
        self.data_configuration = input;
        self
    }
    /// <p>The style configuration of the reference line.</p>
    pub fn style_configuration(
        mut self,
        input: crate::types::ReferenceLineStyleConfiguration,
    ) -> Self {
        self.style_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The style configuration of the reference line.</p>
    pub fn set_style_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ReferenceLineStyleConfiguration>,
    ) -> Self {
        self.style_configuration = input;
        self
    }
    /// <p>The label configuration of the reference line.</p>
    pub fn label_configuration(
        mut self,
        input: crate::types::ReferenceLineLabelConfiguration,
    ) -> Self {
        self.label_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The label configuration of the reference line.</p>
    pub fn set_label_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ReferenceLineLabelConfiguration>,
    ) -> Self {
        self.label_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`ReferenceLine`](crate::types::ReferenceLine).
    pub fn build(self) -> crate::types::ReferenceLine {
        crate::types::ReferenceLine {
            status: self.status,
            data_configuration: self.data_configuration,
            style_configuration: self.style_configuration,
            label_configuration: self.label_configuration,
        }
    }
}
