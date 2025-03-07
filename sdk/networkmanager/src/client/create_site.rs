// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSite`](crate::operation::create_site::builders::CreateSiteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl ::std::convert::Into<String>)`](crate::operation::create_site::builders::CreateSiteFluentBuilder::global_network_id) / [`set_global_network_id(Option<String>)`](crate::operation::create_site::builders::CreateSiteFluentBuilder::set_global_network_id): <p>The ID of the global network.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_site::builders::CreateSiteFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_site::builders::CreateSiteFluentBuilder::set_description): <p>A description of your site.</p>  <p>Constraints: Maximum length of 256 characters.</p>
    ///   - [`location(Location)`](crate::operation::create_site::builders::CreateSiteFluentBuilder::location) / [`set_location(Option<Location>)`](crate::operation::create_site::builders::CreateSiteFluentBuilder::set_location): <p>The site location. This information is used for visualization in the Network Manager console. If you specify the address, the latitude and longitude are automatically calculated.</p>  <ul>   <li> <p> <code>Address</code>: The physical address of the site.</p> </li>   <li> <p> <code>Latitude</code>: The latitude of the site. </p> </li>   <li> <p> <code>Longitude</code>: The longitude of the site.</p> </li>  </ul>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_site::builders::CreateSiteFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_site::builders::CreateSiteFluentBuilder::set_tags): <p>The tags to apply to the resource during creation.</p>
    /// - On success, responds with [`CreateSiteOutput`](crate::operation::create_site::CreateSiteOutput) with field(s):
    ///   - [`site(Option<Site>)`](crate::operation::create_site::CreateSiteOutput::site): <p>Information about the site.</p>
    /// - On failure, responds with [`SdkError<CreateSiteError>`](crate::operation::create_site::CreateSiteError)
    pub fn create_site(&self) -> crate::operation::create_site::builders::CreateSiteFluentBuilder {
        crate::operation::create_site::builders::CreateSiteFluentBuilder::new(self.handle.clone())
    }
}
