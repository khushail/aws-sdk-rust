// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateHitTypeOfHitOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for UpdateHitTypeOfHitOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateHitTypeOfHitOutput {
    /// Creates a new builder-style object to manufacture [`UpdateHitTypeOfHitOutput`](crate::operation::update_hit_type_of_hit::UpdateHitTypeOfHitOutput).
    pub fn builder(
    ) -> crate::operation::update_hit_type_of_hit::builders::UpdateHitTypeOfHitOutputBuilder {
        crate::operation::update_hit_type_of_hit::builders::UpdateHitTypeOfHitOutputBuilder::default(
        )
    }
}

/// A builder for [`UpdateHitTypeOfHitOutput`](crate::operation::update_hit_type_of_hit::UpdateHitTypeOfHitOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateHitTypeOfHitOutputBuilder {
    _request_id: Option<String>,
}
impl UpdateHitTypeOfHitOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateHitTypeOfHitOutput`](crate::operation::update_hit_type_of_hit::UpdateHitTypeOfHitOutput).
    pub fn build(self) -> crate::operation::update_hit_type_of_hit::UpdateHitTypeOfHitOutput {
        crate::operation::update_hit_type_of_hit::UpdateHitTypeOfHitOutput {
            _request_id: self._request_id,
        }
    }
}
