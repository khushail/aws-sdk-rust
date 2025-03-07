// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateSiteRackPhysicalPropertiesOutput {
    /// <p>Information about a site.</p>
    #[doc(hidden)]
    pub site: ::std::option::Option<crate::types::Site>,
    _request_id: Option<String>,
}
impl UpdateSiteRackPhysicalPropertiesOutput {
    /// <p>Information about a site.</p>
    pub fn site(&self) -> ::std::option::Option<&crate::types::Site> {
        self.site.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateSiteRackPhysicalPropertiesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateSiteRackPhysicalPropertiesOutput {
    /// Creates a new builder-style object to manufacture [`UpdateSiteRackPhysicalPropertiesOutput`](crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesOutput).
    pub fn builder() -> crate::operation::update_site_rack_physical_properties::builders::UpdateSiteRackPhysicalPropertiesOutputBuilder{
        crate::operation::update_site_rack_physical_properties::builders::UpdateSiteRackPhysicalPropertiesOutputBuilder::default()
    }
}

/// A builder for [`UpdateSiteRackPhysicalPropertiesOutput`](crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateSiteRackPhysicalPropertiesOutputBuilder {
    pub(crate) site: ::std::option::Option<crate::types::Site>,
    _request_id: Option<String>,
}
impl UpdateSiteRackPhysicalPropertiesOutputBuilder {
    /// <p>Information about a site.</p>
    pub fn site(mut self, input: crate::types::Site) -> Self {
        self.site = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about a site.</p>
    pub fn set_site(mut self, input: ::std::option::Option<crate::types::Site>) -> Self {
        self.site = input;
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
    /// Consumes the builder and constructs a [`UpdateSiteRackPhysicalPropertiesOutput`](crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesOutput).
    pub fn build(self) -> crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesOutput{
        crate::operation::update_site_rack_physical_properties::UpdateSiteRackPhysicalPropertiesOutput {
            site: self.site
            ,
            _request_id: self._request_id,
        }
    }
}
