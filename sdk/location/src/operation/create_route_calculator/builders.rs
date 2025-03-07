// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_route_calculator::_create_route_calculator_output::CreateRouteCalculatorOutputBuilder;

pub use crate::operation::create_route_calculator::_create_route_calculator_input::CreateRouteCalculatorInputBuilder;

/// Fluent builder constructing a request to `CreateRouteCalculator`.
///
/// <p>Creates a route calculator resource in your Amazon Web Services account.</p>
/// <p>You can send requests to a route calculator resource to estimate travel time, distance, and get directions. A route calculator sources traffic and road network data from your chosen data provider.</p> <note>
/// <p>If your application is tracking or routing assets you use in your business, such as delivery vehicles or employees, you must not use Esri as your geolocation provider. See section 82 of the <a href="http://aws.amazon.com/service-terms">Amazon Web Services service terms</a> for more details.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRouteCalculatorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_route_calculator::builders::CreateRouteCalculatorInputBuilder,
}
impl CreateRouteCalculatorFluentBuilder {
    /// Creates a new `CreateRouteCalculator`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_route_calculator::CreateRouteCalculator,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_route_calculator::CreateRouteCalculatorError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_route_calculator::CreateRouteCalculatorOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_route_calculator::CreateRouteCalculatorError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_route_calculator::CreateRouteCalculatorOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_route_calculator::CreateRouteCalculatorError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_route_calculator::CreateRouteCalculator,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_route_calculator::CreateRouteCalculatorError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the route calculator resource. </p>
    /// <p>Requirements:</p>
    /// <ul>
    /// <li> <p>Can use alphanumeric characters (A–Z, a–z, 0–9) , hyphens (-), periods (.), and underscores (_).</p> </li>
    /// <li> <p>Must be a unique Route calculator resource name.</p> </li>
    /// <li> <p>No spaces allowed. For example, <code>ExampleRouteCalculator</code>.</p> </li>
    /// </ul>
    pub fn calculator_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.calculator_name(input.into());
        self
    }
    /// <p>The name of the route calculator resource. </p>
    /// <p>Requirements:</p>
    /// <ul>
    /// <li> <p>Can use alphanumeric characters (A–Z, a–z, 0–9) , hyphens (-), periods (.), and underscores (_).</p> </li>
    /// <li> <p>Must be a unique Route calculator resource name.</p> </li>
    /// <li> <p>No spaces allowed. For example, <code>ExampleRouteCalculator</code>.</p> </li>
    /// </ul>
    pub fn set_calculator_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_calculator_name(input);
        self
    }
    /// <p>Specifies the data provider of traffic and road network data.</p> <note>
    /// <p>This field is case-sensitive. Enter the valid values as shown. For example, entering <code>HERE</code> returns an error.</p>
    /// </note>
    /// <p>Valid values include:</p>
    /// <ul>
    /// <li> <p> <code>Esri</code> – For additional information about <a href="https://docs.aws.amazon.com/location/latest/developerguide/esri.html">Esri</a>'s coverage in your region of interest, see <a href="https://doc.arcgis.com/en/arcgis-online/reference/network-coverage.htm">Esri details on street networks and traffic coverage</a>.</p> <p>Route calculators that use Esri as a data source only calculate routes that are shorter than 400 km.</p> </li>
    /// <li> <p> <code>Grab</code> – Grab provides routing functionality for Southeast Asia. For additional information about <a href="https://docs.aws.amazon.com/location/latest/developerguide/grab.html">GrabMaps</a>' coverage, see <a href="https://docs.aws.amazon.com/location/latest/developerguide/grab.html#grab-coverage-area">GrabMaps countries and areas covered</a>.</p> </li>
    /// <li> <p> <code>Here</code> – For additional information about <a href="https://docs.aws.amazon.com/location/latest/developerguide/HERE.html">HERE Technologies</a>' coverage in your region of interest, see <a href="https://developer.here.com/documentation/routing-api/dev_guide/topics/coverage/car-routing.html">HERE car routing coverage</a> and <a href="https://developer.here.com/documentation/routing-api/dev_guide/topics/coverage/truck-routing.html">HERE truck routing coverage</a>.</p> </li>
    /// </ul>
    /// <p>For additional information , see <a href="https://docs.aws.amazon.com/location/latest/developerguide/what-is-data-provider.html">Data providers</a> on the <i>Amazon Location Service Developer Guide</i>.</p>
    pub fn data_source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.data_source(input.into());
        self
    }
    /// <p>Specifies the data provider of traffic and road network data.</p> <note>
    /// <p>This field is case-sensitive. Enter the valid values as shown. For example, entering <code>HERE</code> returns an error.</p>
    /// </note>
    /// <p>Valid values include:</p>
    /// <ul>
    /// <li> <p> <code>Esri</code> – For additional information about <a href="https://docs.aws.amazon.com/location/latest/developerguide/esri.html">Esri</a>'s coverage in your region of interest, see <a href="https://doc.arcgis.com/en/arcgis-online/reference/network-coverage.htm">Esri details on street networks and traffic coverage</a>.</p> <p>Route calculators that use Esri as a data source only calculate routes that are shorter than 400 km.</p> </li>
    /// <li> <p> <code>Grab</code> – Grab provides routing functionality for Southeast Asia. For additional information about <a href="https://docs.aws.amazon.com/location/latest/developerguide/grab.html">GrabMaps</a>' coverage, see <a href="https://docs.aws.amazon.com/location/latest/developerguide/grab.html#grab-coverage-area">GrabMaps countries and areas covered</a>.</p> </li>
    /// <li> <p> <code>Here</code> – For additional information about <a href="https://docs.aws.amazon.com/location/latest/developerguide/HERE.html">HERE Technologies</a>' coverage in your region of interest, see <a href="https://developer.here.com/documentation/routing-api/dev_guide/topics/coverage/car-routing.html">HERE car routing coverage</a> and <a href="https://developer.here.com/documentation/routing-api/dev_guide/topics/coverage/truck-routing.html">HERE truck routing coverage</a>.</p> </li>
    /// </ul>
    /// <p>For additional information , see <a href="https://docs.aws.amazon.com/location/latest/developerguide/what-is-data-provider.html">Data providers</a> on the <i>Amazon Location Service Developer Guide</i>.</p>
    pub fn set_data_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_data_source(input);
        self
    }
    /// <p>No longer used. If included, the only allowed value is <code>RequestBasedUsage</code>.</p>
    #[deprecated(
        note = "Deprecated. If included, the only allowed value is RequestBasedUsage.",
        since = "2022-02-01"
    )]
    pub fn pricing_plan(mut self, input: crate::types::PricingPlan) -> Self {
        self.inner = self.inner.pricing_plan(input);
        self
    }
    /// <p>No longer used. If included, the only allowed value is <code>RequestBasedUsage</code>.</p>
    #[deprecated(
        note = "Deprecated. If included, the only allowed value is RequestBasedUsage.",
        since = "2022-02-01"
    )]
    pub fn set_pricing_plan(
        mut self,
        input: ::std::option::Option<crate::types::PricingPlan>,
    ) -> Self {
        self.inner = self.inner.set_pricing_plan(input);
        self
    }
    /// <p>The optional description for the route calculator resource.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The optional description for the route calculator resource.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Applies one or more tags to the route calculator resource. A tag is a key-value pair helps manage, identify, search, and filter your resources by labelling them.</p>
    /// <ul>
    /// <li> <p>For example: { <code>"tag1" : "value1"</code>, <code>"tag2" : "value2"</code>}</p> </li>
    /// </ul>
    /// <p>Format: <code>"key" : "value"</code> </p>
    /// <p>Restrictions:</p>
    /// <ul>
    /// <li> <p>Maximum 50 tags per resource</p> </li>
    /// <li> <p>Each resource tag must be unique with a maximum of one value.</p> </li>
    /// <li> <p>Maximum key length: 128 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Maximum value length: 256 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Can use alphanumeric characters (A–Z, a–z, 0–9), and the following characters: + - = . _ : / @. </p> </li>
    /// <li> <p>Cannot use "aws:" as a prefix for a key.</p> </li>
    /// </ul>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Applies one or more tags to the route calculator resource. A tag is a key-value pair helps manage, identify, search, and filter your resources by labelling them.</p>
    /// <ul>
    /// <li> <p>For example: { <code>"tag1" : "value1"</code>, <code>"tag2" : "value2"</code>}</p> </li>
    /// </ul>
    /// <p>Format: <code>"key" : "value"</code> </p>
    /// <p>Restrictions:</p>
    /// <ul>
    /// <li> <p>Maximum 50 tags per resource</p> </li>
    /// <li> <p>Each resource tag must be unique with a maximum of one value.</p> </li>
    /// <li> <p>Maximum key length: 128 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Maximum value length: 256 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Can use alphanumeric characters (A–Z, a–z, 0–9), and the following characters: + - = . _ : / @. </p> </li>
    /// <li> <p>Cannot use "aws:" as a prefix for a key.</p> </li>
    /// </ul>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
