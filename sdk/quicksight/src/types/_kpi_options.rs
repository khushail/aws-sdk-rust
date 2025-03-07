// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The options that determine the presentation of a KPI visual.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KpiOptions {
    /// <p>The options that determine the presentation of the progress bar of a KPI visual.</p>
    #[doc(hidden)]
    pub progress_bar: ::std::option::Option<crate::types::ProgressBarOptions>,
    /// <p>The options that determine the presentation of trend arrows in a KPI visual.</p>
    #[doc(hidden)]
    pub trend_arrows: ::std::option::Option<crate::types::TrendArrowOptions>,
    /// <p>The options that determine the presentation of the secondary value of a KPI visual.</p>
    #[doc(hidden)]
    pub secondary_value: ::std::option::Option<crate::types::SecondaryValueOptions>,
    /// <p>The comparison configuration of a KPI visual.</p>
    #[doc(hidden)]
    pub comparison: ::std::option::Option<crate::types::ComparisonConfiguration>,
    /// <p>The options that determine the primary value display type.</p>
    #[doc(hidden)]
    pub primary_value_display_type: ::std::option::Option<crate::types::PrimaryValueDisplayType>,
    /// <p>The options that determine the primary value font configuration.</p>
    #[doc(hidden)]
    pub primary_value_font_configuration: ::std::option::Option<crate::types::FontConfiguration>,
    /// <p>The options that determine the secondary value font configuration.</p>
    #[doc(hidden)]
    pub secondary_value_font_configuration: ::std::option::Option<crate::types::FontConfiguration>,
}
impl KpiOptions {
    /// <p>The options that determine the presentation of the progress bar of a KPI visual.</p>
    pub fn progress_bar(&self) -> ::std::option::Option<&crate::types::ProgressBarOptions> {
        self.progress_bar.as_ref()
    }
    /// <p>The options that determine the presentation of trend arrows in a KPI visual.</p>
    pub fn trend_arrows(&self) -> ::std::option::Option<&crate::types::TrendArrowOptions> {
        self.trend_arrows.as_ref()
    }
    /// <p>The options that determine the presentation of the secondary value of a KPI visual.</p>
    pub fn secondary_value(&self) -> ::std::option::Option<&crate::types::SecondaryValueOptions> {
        self.secondary_value.as_ref()
    }
    /// <p>The comparison configuration of a KPI visual.</p>
    pub fn comparison(&self) -> ::std::option::Option<&crate::types::ComparisonConfiguration> {
        self.comparison.as_ref()
    }
    /// <p>The options that determine the primary value display type.</p>
    pub fn primary_value_display_type(
        &self,
    ) -> ::std::option::Option<&crate::types::PrimaryValueDisplayType> {
        self.primary_value_display_type.as_ref()
    }
    /// <p>The options that determine the primary value font configuration.</p>
    pub fn primary_value_font_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::FontConfiguration> {
        self.primary_value_font_configuration.as_ref()
    }
    /// <p>The options that determine the secondary value font configuration.</p>
    pub fn secondary_value_font_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::FontConfiguration> {
        self.secondary_value_font_configuration.as_ref()
    }
}
impl KpiOptions {
    /// Creates a new builder-style object to manufacture [`KpiOptions`](crate::types::KpiOptions).
    pub fn builder() -> crate::types::builders::KpiOptionsBuilder {
        crate::types::builders::KpiOptionsBuilder::default()
    }
}

/// A builder for [`KpiOptions`](crate::types::KpiOptions).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KpiOptionsBuilder {
    pub(crate) progress_bar: ::std::option::Option<crate::types::ProgressBarOptions>,
    pub(crate) trend_arrows: ::std::option::Option<crate::types::TrendArrowOptions>,
    pub(crate) secondary_value: ::std::option::Option<crate::types::SecondaryValueOptions>,
    pub(crate) comparison: ::std::option::Option<crate::types::ComparisonConfiguration>,
    pub(crate) primary_value_display_type:
        ::std::option::Option<crate::types::PrimaryValueDisplayType>,
    pub(crate) primary_value_font_configuration:
        ::std::option::Option<crate::types::FontConfiguration>,
    pub(crate) secondary_value_font_configuration:
        ::std::option::Option<crate::types::FontConfiguration>,
}
impl KpiOptionsBuilder {
    /// <p>The options that determine the presentation of the progress bar of a KPI visual.</p>
    pub fn progress_bar(mut self, input: crate::types::ProgressBarOptions) -> Self {
        self.progress_bar = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options that determine the presentation of the progress bar of a KPI visual.</p>
    pub fn set_progress_bar(
        mut self,
        input: ::std::option::Option<crate::types::ProgressBarOptions>,
    ) -> Self {
        self.progress_bar = input;
        self
    }
    /// <p>The options that determine the presentation of trend arrows in a KPI visual.</p>
    pub fn trend_arrows(mut self, input: crate::types::TrendArrowOptions) -> Self {
        self.trend_arrows = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options that determine the presentation of trend arrows in a KPI visual.</p>
    pub fn set_trend_arrows(
        mut self,
        input: ::std::option::Option<crate::types::TrendArrowOptions>,
    ) -> Self {
        self.trend_arrows = input;
        self
    }
    /// <p>The options that determine the presentation of the secondary value of a KPI visual.</p>
    pub fn secondary_value(mut self, input: crate::types::SecondaryValueOptions) -> Self {
        self.secondary_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options that determine the presentation of the secondary value of a KPI visual.</p>
    pub fn set_secondary_value(
        mut self,
        input: ::std::option::Option<crate::types::SecondaryValueOptions>,
    ) -> Self {
        self.secondary_value = input;
        self
    }
    /// <p>The comparison configuration of a KPI visual.</p>
    pub fn comparison(mut self, input: crate::types::ComparisonConfiguration) -> Self {
        self.comparison = ::std::option::Option::Some(input);
        self
    }
    /// <p>The comparison configuration of a KPI visual.</p>
    pub fn set_comparison(
        mut self,
        input: ::std::option::Option<crate::types::ComparisonConfiguration>,
    ) -> Self {
        self.comparison = input;
        self
    }
    /// <p>The options that determine the primary value display type.</p>
    pub fn primary_value_display_type(
        mut self,
        input: crate::types::PrimaryValueDisplayType,
    ) -> Self {
        self.primary_value_display_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options that determine the primary value display type.</p>
    pub fn set_primary_value_display_type(
        mut self,
        input: ::std::option::Option<crate::types::PrimaryValueDisplayType>,
    ) -> Self {
        self.primary_value_display_type = input;
        self
    }
    /// <p>The options that determine the primary value font configuration.</p>
    pub fn primary_value_font_configuration(
        mut self,
        input: crate::types::FontConfiguration,
    ) -> Self {
        self.primary_value_font_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options that determine the primary value font configuration.</p>
    pub fn set_primary_value_font_configuration(
        mut self,
        input: ::std::option::Option<crate::types::FontConfiguration>,
    ) -> Self {
        self.primary_value_font_configuration = input;
        self
    }
    /// <p>The options that determine the secondary value font configuration.</p>
    pub fn secondary_value_font_configuration(
        mut self,
        input: crate::types::FontConfiguration,
    ) -> Self {
        self.secondary_value_font_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options that determine the secondary value font configuration.</p>
    pub fn set_secondary_value_font_configuration(
        mut self,
        input: ::std::option::Option<crate::types::FontConfiguration>,
    ) -> Self {
        self.secondary_value_font_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`KpiOptions`](crate::types::KpiOptions).
    pub fn build(self) -> crate::types::KpiOptions {
        crate::types::KpiOptions {
            progress_bar: self.progress_bar,
            trend_arrows: self.trend_arrows,
            secondary_value: self.secondary_value,
            comparison: self.comparison,
            primary_value_display_type: self.primary_value_display_type,
            primary_value_font_configuration: self.primary_value_font_configuration,
            secondary_value_font_configuration: self.secondary_value_font_configuration,
        }
    }
}
