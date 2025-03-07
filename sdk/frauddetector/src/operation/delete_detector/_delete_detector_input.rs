// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteDetectorInput {
    /// <p>The ID of the detector to delete.</p>
    #[doc(hidden)]
    pub detector_id: ::std::option::Option<::std::string::String>,
}
impl DeleteDetectorInput {
    /// <p>The ID of the detector to delete.</p>
    pub fn detector_id(&self) -> ::std::option::Option<&str> {
        self.detector_id.as_deref()
    }
}
impl DeleteDetectorInput {
    /// Creates a new builder-style object to manufacture [`DeleteDetectorInput`](crate::operation::delete_detector::DeleteDetectorInput).
    pub fn builder() -> crate::operation::delete_detector::builders::DeleteDetectorInputBuilder {
        crate::operation::delete_detector::builders::DeleteDetectorInputBuilder::default()
    }
}

/// A builder for [`DeleteDetectorInput`](crate::operation::delete_detector::DeleteDetectorInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteDetectorInputBuilder {
    pub(crate) detector_id: ::std::option::Option<::std::string::String>,
}
impl DeleteDetectorInputBuilder {
    /// <p>The ID of the detector to delete.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.detector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the detector to delete.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.detector_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteDetectorInput`](crate::operation::delete_detector::DeleteDetectorInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_detector::DeleteDetectorInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_detector::DeleteDetectorInput {
            detector_id: self.detector_id,
        })
    }
}
