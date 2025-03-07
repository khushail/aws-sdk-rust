// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the dimension settings for a segment.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SegmentDimensions {
    /// <p>One or more custom attributes to use as criteria for the segment.</p>
    #[doc(hidden)]
    pub attributes: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::AttributeDimension>,
    >,
    /// <p>The behavior-based criteria, such as how recently users have used your app, for the segment.</p>
    #[doc(hidden)]
    pub behavior: ::std::option::Option<crate::types::SegmentBehaviors>,
    /// <p>The demographic-based criteria, such as device platform, for the segment.</p>
    #[doc(hidden)]
    pub demographic: ::std::option::Option<crate::types::SegmentDemographics>,
    /// <p>The location-based criteria, such as region or GPS coordinates, for the segment.</p>
    #[doc(hidden)]
    pub location: ::std::option::Option<crate::types::SegmentLocation>,
    /// <p>One or more custom metrics to use as criteria for the segment.</p>
    #[doc(hidden)]
    pub metrics: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::MetricDimension>,
    >,
    /// <p>One or more custom user attributes to use as criteria for the segment.</p>
    #[doc(hidden)]
    pub user_attributes: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::AttributeDimension>,
    >,
}
impl SegmentDimensions {
    /// <p>One or more custom attributes to use as criteria for the segment.</p>
    pub fn attributes(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::AttributeDimension>,
    > {
        self.attributes.as_ref()
    }
    /// <p>The behavior-based criteria, such as how recently users have used your app, for the segment.</p>
    pub fn behavior(&self) -> ::std::option::Option<&crate::types::SegmentBehaviors> {
        self.behavior.as_ref()
    }
    /// <p>The demographic-based criteria, such as device platform, for the segment.</p>
    pub fn demographic(&self) -> ::std::option::Option<&crate::types::SegmentDemographics> {
        self.demographic.as_ref()
    }
    /// <p>The location-based criteria, such as region or GPS coordinates, for the segment.</p>
    pub fn location(&self) -> ::std::option::Option<&crate::types::SegmentLocation> {
        self.location.as_ref()
    }
    /// <p>One or more custom metrics to use as criteria for the segment.</p>
    pub fn metrics(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::MetricDimension>,
    > {
        self.metrics.as_ref()
    }
    /// <p>One or more custom user attributes to use as criteria for the segment.</p>
    pub fn user_attributes(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::AttributeDimension>,
    > {
        self.user_attributes.as_ref()
    }
}
impl SegmentDimensions {
    /// Creates a new builder-style object to manufacture [`SegmentDimensions`](crate::types::SegmentDimensions).
    pub fn builder() -> crate::types::builders::SegmentDimensionsBuilder {
        crate::types::builders::SegmentDimensionsBuilder::default()
    }
}

/// A builder for [`SegmentDimensions`](crate::types::SegmentDimensions).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SegmentDimensionsBuilder {
    pub(crate) attributes: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::AttributeDimension>,
    >,
    pub(crate) behavior: ::std::option::Option<crate::types::SegmentBehaviors>,
    pub(crate) demographic: ::std::option::Option<crate::types::SegmentDemographics>,
    pub(crate) location: ::std::option::Option<crate::types::SegmentLocation>,
    pub(crate) metrics: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::MetricDimension>,
    >,
    pub(crate) user_attributes: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::AttributeDimension>,
    >,
}
impl SegmentDimensionsBuilder {
    /// Adds a key-value pair to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>One or more custom attributes to use as criteria for the segment.</p>
    pub fn attributes(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::AttributeDimension,
    ) -> Self {
        let mut hash_map = self.attributes.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.attributes = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>One or more custom attributes to use as criteria for the segment.</p>
    pub fn set_attributes(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::AttributeDimension>,
        >,
    ) -> Self {
        self.attributes = input;
        self
    }
    /// <p>The behavior-based criteria, such as how recently users have used your app, for the segment.</p>
    pub fn behavior(mut self, input: crate::types::SegmentBehaviors) -> Self {
        self.behavior = ::std::option::Option::Some(input);
        self
    }
    /// <p>The behavior-based criteria, such as how recently users have used your app, for the segment.</p>
    pub fn set_behavior(
        mut self,
        input: ::std::option::Option<crate::types::SegmentBehaviors>,
    ) -> Self {
        self.behavior = input;
        self
    }
    /// <p>The demographic-based criteria, such as device platform, for the segment.</p>
    pub fn demographic(mut self, input: crate::types::SegmentDemographics) -> Self {
        self.demographic = ::std::option::Option::Some(input);
        self
    }
    /// <p>The demographic-based criteria, such as device platform, for the segment.</p>
    pub fn set_demographic(
        mut self,
        input: ::std::option::Option<crate::types::SegmentDemographics>,
    ) -> Self {
        self.demographic = input;
        self
    }
    /// <p>The location-based criteria, such as region or GPS coordinates, for the segment.</p>
    pub fn location(mut self, input: crate::types::SegmentLocation) -> Self {
        self.location = ::std::option::Option::Some(input);
        self
    }
    /// <p>The location-based criteria, such as region or GPS coordinates, for the segment.</p>
    pub fn set_location(
        mut self,
        input: ::std::option::Option<crate::types::SegmentLocation>,
    ) -> Self {
        self.location = input;
        self
    }
    /// Adds a key-value pair to `metrics`.
    ///
    /// To override the contents of this collection use [`set_metrics`](Self::set_metrics).
    ///
    /// <p>One or more custom metrics to use as criteria for the segment.</p>
    pub fn metrics(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::MetricDimension,
    ) -> Self {
        let mut hash_map = self.metrics.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.metrics = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>One or more custom metrics to use as criteria for the segment.</p>
    pub fn set_metrics(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::MetricDimension>,
        >,
    ) -> Self {
        self.metrics = input;
        self
    }
    /// Adds a key-value pair to `user_attributes`.
    ///
    /// To override the contents of this collection use [`set_user_attributes`](Self::set_user_attributes).
    ///
    /// <p>One or more custom user attributes to use as criteria for the segment.</p>
    pub fn user_attributes(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::AttributeDimension,
    ) -> Self {
        let mut hash_map = self.user_attributes.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.user_attributes = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>One or more custom user attributes to use as criteria for the segment.</p>
    pub fn set_user_attributes(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::AttributeDimension>,
        >,
    ) -> Self {
        self.user_attributes = input;
        self
    }
    /// Consumes the builder and constructs a [`SegmentDimensions`](crate::types::SegmentDimensions).
    pub fn build(self) -> crate::types::SegmentDimensions {
        crate::types::SegmentDimensions {
            attributes: self.attributes,
            behavior: self.behavior,
            demographic: self.demographic,
            location: self.location,
            metrics: self.metrics,
            user_attributes: self.user_attributes,
        }
    }
}
