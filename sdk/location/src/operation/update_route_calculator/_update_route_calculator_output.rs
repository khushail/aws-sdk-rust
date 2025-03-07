// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateRouteCalculatorOutput {
    /// <p>The name of the updated route calculator resource.</p>
    #[doc(hidden)]
    pub calculator_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the updated route calculator resource. Used to specify a resource across AWS.</p>
    /// <ul>
    /// <li> <p>Format example: <code>arn:aws:geo:region:account-id:route- calculator/ExampleCalculator</code> </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub calculator_arn: ::std::option::Option<::std::string::String>,
    /// <p>The timestamp for when the route calculator was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    #[doc(hidden)]
    pub update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl UpdateRouteCalculatorOutput {
    /// <p>The name of the updated route calculator resource.</p>
    pub fn calculator_name(&self) -> ::std::option::Option<&str> {
        self.calculator_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the updated route calculator resource. Used to specify a resource across AWS.</p>
    /// <ul>
    /// <li> <p>Format example: <code>arn:aws:geo:region:account-id:route- calculator/ExampleCalculator</code> </p> </li>
    /// </ul>
    pub fn calculator_arn(&self) -> ::std::option::Option<&str> {
        self.calculator_arn.as_deref()
    }
    /// <p>The timestamp for when the route calculator was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    pub fn update_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.update_time.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateRouteCalculatorOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateRouteCalculatorOutput {
    /// Creates a new builder-style object to manufacture [`UpdateRouteCalculatorOutput`](crate::operation::update_route_calculator::UpdateRouteCalculatorOutput).
    pub fn builder(
    ) -> crate::operation::update_route_calculator::builders::UpdateRouteCalculatorOutputBuilder
    {
        crate::operation::update_route_calculator::builders::UpdateRouteCalculatorOutputBuilder::default()
    }
}

/// A builder for [`UpdateRouteCalculatorOutput`](crate::operation::update_route_calculator::UpdateRouteCalculatorOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateRouteCalculatorOutputBuilder {
    pub(crate) calculator_name: ::std::option::Option<::std::string::String>,
    pub(crate) calculator_arn: ::std::option::Option<::std::string::String>,
    pub(crate) update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl UpdateRouteCalculatorOutputBuilder {
    /// <p>The name of the updated route calculator resource.</p>
    pub fn calculator_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.calculator_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the updated route calculator resource.</p>
    pub fn set_calculator_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.calculator_name = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the updated route calculator resource. Used to specify a resource across AWS.</p>
    /// <ul>
    /// <li> <p>Format example: <code>arn:aws:geo:region:account-id:route- calculator/ExampleCalculator</code> </p> </li>
    /// </ul>
    pub fn calculator_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.calculator_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the updated route calculator resource. Used to specify a resource across AWS.</p>
    /// <ul>
    /// <li> <p>Format example: <code>arn:aws:geo:region:account-id:route- calculator/ExampleCalculator</code> </p> </li>
    /// </ul>
    pub fn set_calculator_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.calculator_arn = input;
        self
    }
    /// <p>The timestamp for when the route calculator was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    pub fn update_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.update_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the route calculator was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    pub fn set_update_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.update_time = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateRouteCalculatorOutput`](crate::operation::update_route_calculator::UpdateRouteCalculatorOutput).
    pub fn build(self) -> crate::operation::update_route_calculator::UpdateRouteCalculatorOutput {
        crate::operation::update_route_calculator::UpdateRouteCalculatorOutput {
            calculator_name: self.calculator_name,
            calculator_arn: self.calculator_arn,
            update_time: self.update_time,
            _request_id: self._request_id,
        }
    }
}
