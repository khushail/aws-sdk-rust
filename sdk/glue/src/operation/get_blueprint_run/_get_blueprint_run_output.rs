// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBlueprintRunOutput {
    /// <p>Returns a <code>BlueprintRun</code> object.</p>
    #[doc(hidden)]
    pub blueprint_run: ::std::option::Option<crate::types::BlueprintRun>,
    _request_id: Option<String>,
}
impl GetBlueprintRunOutput {
    /// <p>Returns a <code>BlueprintRun</code> object.</p>
    pub fn blueprint_run(&self) -> ::std::option::Option<&crate::types::BlueprintRun> {
        self.blueprint_run.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetBlueprintRunOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetBlueprintRunOutput {
    /// Creates a new builder-style object to manufacture [`GetBlueprintRunOutput`](crate::operation::get_blueprint_run::GetBlueprintRunOutput).
    pub fn builder() -> crate::operation::get_blueprint_run::builders::GetBlueprintRunOutputBuilder
    {
        crate::operation::get_blueprint_run::builders::GetBlueprintRunOutputBuilder::default()
    }
}

/// A builder for [`GetBlueprintRunOutput`](crate::operation::get_blueprint_run::GetBlueprintRunOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetBlueprintRunOutputBuilder {
    pub(crate) blueprint_run: ::std::option::Option<crate::types::BlueprintRun>,
    _request_id: Option<String>,
}
impl GetBlueprintRunOutputBuilder {
    /// <p>Returns a <code>BlueprintRun</code> object.</p>
    pub fn blueprint_run(mut self, input: crate::types::BlueprintRun) -> Self {
        self.blueprint_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns a <code>BlueprintRun</code> object.</p>
    pub fn set_blueprint_run(
        mut self,
        input: ::std::option::Option<crate::types::BlueprintRun>,
    ) -> Self {
        self.blueprint_run = input;
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
    /// Consumes the builder and constructs a [`GetBlueprintRunOutput`](crate::operation::get_blueprint_run::GetBlueprintRunOutput).
    pub fn build(self) -> crate::operation::get_blueprint_run::GetBlueprintRunOutput {
        crate::operation::get_blueprint_run::GetBlueprintRunOutput {
            blueprint_run: self.blueprint_run,
            _request_id: self._request_id,
        }
    }
}
