// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Reserved Instance listing.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReservedInstancesListing {
    /// <p>A unique, case-sensitive key supplied by the client to ensure that the request is idempotent. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The time the listing was created.</p>
    #[doc(hidden)]
    pub create_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The number of instances in this state.</p>
    #[doc(hidden)]
    pub instance_counts: ::std::option::Option<::std::vec::Vec<crate::types::InstanceCount>>,
    /// <p>The price of the Reserved Instance listing.</p>
    #[doc(hidden)]
    pub price_schedules: ::std::option::Option<::std::vec::Vec<crate::types::PriceSchedule>>,
    /// <p>The ID of the Reserved Instance.</p>
    #[doc(hidden)]
    pub reserved_instances_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Reserved Instance listing.</p>
    #[doc(hidden)]
    pub reserved_instances_listing_id: ::std::option::Option<::std::string::String>,
    /// <p>The status of the Reserved Instance listing.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ListingStatus>,
    /// <p>The reason for the current status of the Reserved Instance listing. The response can be blank.</p>
    #[doc(hidden)]
    pub status_message: ::std::option::Option<::std::string::String>,
    /// <p>Any tags assigned to the resource.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The last modified timestamp of the listing.</p>
    #[doc(hidden)]
    pub update_date: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ReservedInstancesListing {
    /// <p>A unique, case-sensitive key supplied by the client to ensure that the request is idempotent. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The time the listing was created.</p>
    pub fn create_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.create_date.as_ref()
    }
    /// <p>The number of instances in this state.</p>
    pub fn instance_counts(&self) -> ::std::option::Option<&[crate::types::InstanceCount]> {
        self.instance_counts.as_deref()
    }
    /// <p>The price of the Reserved Instance listing.</p>
    pub fn price_schedules(&self) -> ::std::option::Option<&[crate::types::PriceSchedule]> {
        self.price_schedules.as_deref()
    }
    /// <p>The ID of the Reserved Instance.</p>
    pub fn reserved_instances_id(&self) -> ::std::option::Option<&str> {
        self.reserved_instances_id.as_deref()
    }
    /// <p>The ID of the Reserved Instance listing.</p>
    pub fn reserved_instances_listing_id(&self) -> ::std::option::Option<&str> {
        self.reserved_instances_listing_id.as_deref()
    }
    /// <p>The status of the Reserved Instance listing.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ListingStatus> {
        self.status.as_ref()
    }
    /// <p>The reason for the current status of the Reserved Instance listing. The response can be blank.</p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p>Any tags assigned to the resource.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>The last modified timestamp of the listing.</p>
    pub fn update_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.update_date.as_ref()
    }
}
impl ReservedInstancesListing {
    /// Creates a new builder-style object to manufacture [`ReservedInstancesListing`](crate::types::ReservedInstancesListing).
    pub fn builder() -> crate::types::builders::ReservedInstancesListingBuilder {
        crate::types::builders::ReservedInstancesListingBuilder::default()
    }
}

/// A builder for [`ReservedInstancesListing`](crate::types::ReservedInstancesListing).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReservedInstancesListingBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) create_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) instance_counts: ::std::option::Option<::std::vec::Vec<crate::types::InstanceCount>>,
    pub(crate) price_schedules: ::std::option::Option<::std::vec::Vec<crate::types::PriceSchedule>>,
    pub(crate) reserved_instances_id: ::std::option::Option<::std::string::String>,
    pub(crate) reserved_instances_listing_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ListingStatus>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) update_date: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ReservedInstancesListingBuilder {
    /// <p>A unique, case-sensitive key supplied by the client to ensure that the request is idempotent. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive key supplied by the client to ensure that the request is idempotent. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The time the listing was created.</p>
    pub fn create_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.create_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time the listing was created.</p>
    pub fn set_create_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.create_date = input;
        self
    }
    /// Appends an item to `instance_counts`.
    ///
    /// To override the contents of this collection use [`set_instance_counts`](Self::set_instance_counts).
    ///
    /// <p>The number of instances in this state.</p>
    pub fn instance_counts(mut self, input: crate::types::InstanceCount) -> Self {
        let mut v = self.instance_counts.unwrap_or_default();
        v.push(input);
        self.instance_counts = ::std::option::Option::Some(v);
        self
    }
    /// <p>The number of instances in this state.</p>
    pub fn set_instance_counts(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceCount>>,
    ) -> Self {
        self.instance_counts = input;
        self
    }
    /// Appends an item to `price_schedules`.
    ///
    /// To override the contents of this collection use [`set_price_schedules`](Self::set_price_schedules).
    ///
    /// <p>The price of the Reserved Instance listing.</p>
    pub fn price_schedules(mut self, input: crate::types::PriceSchedule) -> Self {
        let mut v = self.price_schedules.unwrap_or_default();
        v.push(input);
        self.price_schedules = ::std::option::Option::Some(v);
        self
    }
    /// <p>The price of the Reserved Instance listing.</p>
    pub fn set_price_schedules(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PriceSchedule>>,
    ) -> Self {
        self.price_schedules = input;
        self
    }
    /// <p>The ID of the Reserved Instance.</p>
    pub fn reserved_instances_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.reserved_instances_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Reserved Instance.</p>
    pub fn set_reserved_instances_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.reserved_instances_id = input;
        self
    }
    /// <p>The ID of the Reserved Instance listing.</p>
    pub fn reserved_instances_listing_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.reserved_instances_listing_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Reserved Instance listing.</p>
    pub fn set_reserved_instances_listing_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.reserved_instances_listing_id = input;
        self
    }
    /// <p>The status of the Reserved Instance listing.</p>
    pub fn status(mut self, input: crate::types::ListingStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the Reserved Instance listing.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ListingStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The reason for the current status of the Reserved Instance listing. The response can be blank.</p>
    pub fn status_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason for the current status of the Reserved Instance listing. The response can be blank.</p>
    pub fn set_status_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.status_message = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags assigned to the resource.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Any tags assigned to the resource.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The last modified timestamp of the listing.</p>
    pub fn update_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.update_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last modified timestamp of the listing.</p>
    pub fn set_update_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.update_date = input;
        self
    }
    /// Consumes the builder and constructs a [`ReservedInstancesListing`](crate::types::ReservedInstancesListing).
    pub fn build(self) -> crate::types::ReservedInstancesListing {
        crate::types::ReservedInstancesListing {
            client_token: self.client_token,
            create_date: self.create_date,
            instance_counts: self.instance_counts,
            price_schedules: self.price_schedules,
            reserved_instances_id: self.reserved_instances_id,
            reserved_instances_listing_id: self.reserved_instances_listing_id,
            status: self.status,
            status_message: self.status_message,
            tags: self.tags,
            update_date: self.update_date,
        }
    }
}
