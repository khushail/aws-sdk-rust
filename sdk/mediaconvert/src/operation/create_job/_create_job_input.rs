// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateJobInput {
    /// Optional. Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.
    #[doc(hidden)]
    pub acceleration_settings: ::std::option::Option<crate::types::AccelerationSettings>,
    /// Optional. Choose a tag type that AWS Billing and Cost Management will use to sort your AWS Elemental MediaConvert costs on any billing report that you set up. Any transcoding outputs that don't have an associated tag will appear in your billing report unsorted. If you don't choose a valid value for this field, your job outputs will appear on the billing report unsorted.
    #[doc(hidden)]
    pub billing_tags_source: ::std::option::Option<crate::types::BillingTagsSource>,
    /// Prevent duplicate jobs from being created and ensure idempotency for your requests. A client request token can be any string that includes up to 64 ASCII characters. If you reuse a client request token within one minute of a successful request, the API returns the job details of the original request instead. For more information see https://docs.aws.amazon.com/mediaconvert/latest/apireference/idempotency.html.
    #[doc(hidden)]
    pub client_request_token: ::std::option::Option<::std::string::String>,
    /// Optional. Use queue hopping to avoid overly long waits in the backlog of the queue that you submit your job to. Specify an alternate queue and the maximum time that your job will wait in the initial queue before hopping. For more information about this feature, see the AWS Elemental MediaConvert User Guide.
    #[doc(hidden)]
    pub hop_destinations: ::std::option::Option<::std::vec::Vec<crate::types::HopDestination>>,
    /// Optional. When you create a job, you can either specify a job template or specify the transcoding settings individually.
    #[doc(hidden)]
    pub job_template: ::std::option::Option<::std::string::String>,
    /// Optional. Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don't specify a priority, the service uses the default value 0.
    #[doc(hidden)]
    pub priority: ::std::option::Option<i32>,
    /// Optional. When you create a job, you can specify a queue to send it to. If you don't specify, the job will go to the default queue. For more about queues, see the User Guide topic at https://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html.
    #[doc(hidden)]
    pub queue: ::std::option::Option<::std::string::String>,
    /// Required. The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html.
    #[doc(hidden)]
    pub role: ::std::option::Option<::std::string::String>,
    /// JobSettings contains all the transcode settings for a job.
    #[doc(hidden)]
    pub settings: ::std::option::Option<crate::types::JobSettings>,
    /// Optional. Enable this setting when you run a test job to estimate how many reserved transcoding slots (RTS) you need. When this is enabled, MediaConvert runs your job from an on-demand queue with similar performance to what you will see with one RTS in a reserved queue. This setting is disabled by default.
    #[doc(hidden)]
    pub simulate_reserved_queue: ::std::option::Option<crate::types::SimulateReservedQueue>,
    /// Optional. Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.
    #[doc(hidden)]
    pub status_update_interval: ::std::option::Option<crate::types::StatusUpdateInterval>,
    /// Optional. The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key. Use standard AWS tags on your job for automatic integration with AWS services and for custom integrations and workflows.
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// Optional. User-defined metadata that you want to associate with an MediaConvert job. You specify metadata in key/value pairs. Use only for existing integrations or workflows that rely on job metadata tags. Otherwise, we recommend that you use standard AWS tags.
    #[doc(hidden)]
    pub user_metadata: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateJobInput {
    /// Optional. Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.
    pub fn acceleration_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::AccelerationSettings> {
        self.acceleration_settings.as_ref()
    }
    /// Optional. Choose a tag type that AWS Billing and Cost Management will use to sort your AWS Elemental MediaConvert costs on any billing report that you set up. Any transcoding outputs that don't have an associated tag will appear in your billing report unsorted. If you don't choose a valid value for this field, your job outputs will appear on the billing report unsorted.
    pub fn billing_tags_source(&self) -> ::std::option::Option<&crate::types::BillingTagsSource> {
        self.billing_tags_source.as_ref()
    }
    /// Prevent duplicate jobs from being created and ensure idempotency for your requests. A client request token can be any string that includes up to 64 ASCII characters. If you reuse a client request token within one minute of a successful request, the API returns the job details of the original request instead. For more information see https://docs.aws.amazon.com/mediaconvert/latest/apireference/idempotency.html.
    pub fn client_request_token(&self) -> ::std::option::Option<&str> {
        self.client_request_token.as_deref()
    }
    /// Optional. Use queue hopping to avoid overly long waits in the backlog of the queue that you submit your job to. Specify an alternate queue and the maximum time that your job will wait in the initial queue before hopping. For more information about this feature, see the AWS Elemental MediaConvert User Guide.
    pub fn hop_destinations(&self) -> ::std::option::Option<&[crate::types::HopDestination]> {
        self.hop_destinations.as_deref()
    }
    /// Optional. When you create a job, you can either specify a job template or specify the transcoding settings individually.
    pub fn job_template(&self) -> ::std::option::Option<&str> {
        self.job_template.as_deref()
    }
    /// Optional. Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don't specify a priority, the service uses the default value 0.
    pub fn priority(&self) -> ::std::option::Option<i32> {
        self.priority
    }
    /// Optional. When you create a job, you can specify a queue to send it to. If you don't specify, the job will go to the default queue. For more about queues, see the User Guide topic at https://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html.
    pub fn queue(&self) -> ::std::option::Option<&str> {
        self.queue.as_deref()
    }
    /// Required. The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html.
    pub fn role(&self) -> ::std::option::Option<&str> {
        self.role.as_deref()
    }
    /// JobSettings contains all the transcode settings for a job.
    pub fn settings(&self) -> ::std::option::Option<&crate::types::JobSettings> {
        self.settings.as_ref()
    }
    /// Optional. Enable this setting when you run a test job to estimate how many reserved transcoding slots (RTS) you need. When this is enabled, MediaConvert runs your job from an on-demand queue with similar performance to what you will see with one RTS in a reserved queue. This setting is disabled by default.
    pub fn simulate_reserved_queue(
        &self,
    ) -> ::std::option::Option<&crate::types::SimulateReservedQueue> {
        self.simulate_reserved_queue.as_ref()
    }
    /// Optional. Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.
    pub fn status_update_interval(
        &self,
    ) -> ::std::option::Option<&crate::types::StatusUpdateInterval> {
        self.status_update_interval.as_ref()
    }
    /// Optional. The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key. Use standard AWS tags on your job for automatic integration with AWS services and for custom integrations and workflows.
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
    /// Optional. User-defined metadata that you want to associate with an MediaConvert job. You specify metadata in key/value pairs. Use only for existing integrations or workflows that rely on job metadata tags. Otherwise, we recommend that you use standard AWS tags.
    pub fn user_metadata(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.user_metadata.as_ref()
    }
}
impl CreateJobInput {
    /// Creates a new builder-style object to manufacture [`CreateJobInput`](crate::operation::create_job::CreateJobInput).
    pub fn builder() -> crate::operation::create_job::builders::CreateJobInputBuilder {
        crate::operation::create_job::builders::CreateJobInputBuilder::default()
    }
}

/// A builder for [`CreateJobInput`](crate::operation::create_job::CreateJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateJobInputBuilder {
    pub(crate) acceleration_settings: ::std::option::Option<crate::types::AccelerationSettings>,
    pub(crate) billing_tags_source: ::std::option::Option<crate::types::BillingTagsSource>,
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
    pub(crate) hop_destinations:
        ::std::option::Option<::std::vec::Vec<crate::types::HopDestination>>,
    pub(crate) job_template: ::std::option::Option<::std::string::String>,
    pub(crate) priority: ::std::option::Option<i32>,
    pub(crate) queue: ::std::option::Option<::std::string::String>,
    pub(crate) role: ::std::option::Option<::std::string::String>,
    pub(crate) settings: ::std::option::Option<crate::types::JobSettings>,
    pub(crate) simulate_reserved_queue: ::std::option::Option<crate::types::SimulateReservedQueue>,
    pub(crate) status_update_interval: ::std::option::Option<crate::types::StatusUpdateInterval>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) user_metadata: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateJobInputBuilder {
    /// Optional. Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.
    pub fn acceleration_settings(mut self, input: crate::types::AccelerationSettings) -> Self {
        self.acceleration_settings = ::std::option::Option::Some(input);
        self
    }
    /// Optional. Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.
    pub fn set_acceleration_settings(
        mut self,
        input: ::std::option::Option<crate::types::AccelerationSettings>,
    ) -> Self {
        self.acceleration_settings = input;
        self
    }
    /// Optional. Choose a tag type that AWS Billing and Cost Management will use to sort your AWS Elemental MediaConvert costs on any billing report that you set up. Any transcoding outputs that don't have an associated tag will appear in your billing report unsorted. If you don't choose a valid value for this field, your job outputs will appear on the billing report unsorted.
    pub fn billing_tags_source(mut self, input: crate::types::BillingTagsSource) -> Self {
        self.billing_tags_source = ::std::option::Option::Some(input);
        self
    }
    /// Optional. Choose a tag type that AWS Billing and Cost Management will use to sort your AWS Elemental MediaConvert costs on any billing report that you set up. Any transcoding outputs that don't have an associated tag will appear in your billing report unsorted. If you don't choose a valid value for this field, your job outputs will appear on the billing report unsorted.
    pub fn set_billing_tags_source(
        mut self,
        input: ::std::option::Option<crate::types::BillingTagsSource>,
    ) -> Self {
        self.billing_tags_source = input;
        self
    }
    /// Prevent duplicate jobs from being created and ensure idempotency for your requests. A client request token can be any string that includes up to 64 ASCII characters. If you reuse a client request token within one minute of a successful request, the API returns the job details of the original request instead. For more information see https://docs.aws.amazon.com/mediaconvert/latest/apireference/idempotency.html.
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.client_request_token = ::std::option::Option::Some(input.into());
        self
    }
    /// Prevent duplicate jobs from being created and ensure idempotency for your requests. A client request token can be any string that includes up to 64 ASCII characters. If you reuse a client request token within one minute of a successful request, the API returns the job details of the original request instead. For more information see https://docs.aws.amazon.com/mediaconvert/latest/apireference/idempotency.html.
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.client_request_token = input;
        self
    }
    /// Appends an item to `hop_destinations`.
    ///
    /// To override the contents of this collection use [`set_hop_destinations`](Self::set_hop_destinations).
    ///
    /// Optional. Use queue hopping to avoid overly long waits in the backlog of the queue that you submit your job to. Specify an alternate queue and the maximum time that your job will wait in the initial queue before hopping. For more information about this feature, see the AWS Elemental MediaConvert User Guide.
    pub fn hop_destinations(mut self, input: crate::types::HopDestination) -> Self {
        let mut v = self.hop_destinations.unwrap_or_default();
        v.push(input);
        self.hop_destinations = ::std::option::Option::Some(v);
        self
    }
    /// Optional. Use queue hopping to avoid overly long waits in the backlog of the queue that you submit your job to. Specify an alternate queue and the maximum time that your job will wait in the initial queue before hopping. For more information about this feature, see the AWS Elemental MediaConvert User Guide.
    pub fn set_hop_destinations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::HopDestination>>,
    ) -> Self {
        self.hop_destinations = input;
        self
    }
    /// Optional. When you create a job, you can either specify a job template or specify the transcoding settings individually.
    pub fn job_template(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_template = ::std::option::Option::Some(input.into());
        self
    }
    /// Optional. When you create a job, you can either specify a job template or specify the transcoding settings individually.
    pub fn set_job_template(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_template = input;
        self
    }
    /// Optional. Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don't specify a priority, the service uses the default value 0.
    pub fn priority(mut self, input: i32) -> Self {
        self.priority = ::std::option::Option::Some(input);
        self
    }
    /// Optional. Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don't specify a priority, the service uses the default value 0.
    pub fn set_priority(mut self, input: ::std::option::Option<i32>) -> Self {
        self.priority = input;
        self
    }
    /// Optional. When you create a job, you can specify a queue to send it to. If you don't specify, the job will go to the default queue. For more about queues, see the User Guide topic at https://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html.
    pub fn queue(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.queue = ::std::option::Option::Some(input.into());
        self
    }
    /// Optional. When you create a job, you can specify a queue to send it to. If you don't specify, the job will go to the default queue. For more about queues, see the User Guide topic at https://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html.
    pub fn set_queue(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.queue = input;
        self
    }
    /// Required. The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html.
    pub fn role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role = ::std::option::Option::Some(input.into());
        self
    }
    /// Required. The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html.
    pub fn set_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role = input;
        self
    }
    /// JobSettings contains all the transcode settings for a job.
    pub fn settings(mut self, input: crate::types::JobSettings) -> Self {
        self.settings = ::std::option::Option::Some(input);
        self
    }
    /// JobSettings contains all the transcode settings for a job.
    pub fn set_settings(mut self, input: ::std::option::Option<crate::types::JobSettings>) -> Self {
        self.settings = input;
        self
    }
    /// Optional. Enable this setting when you run a test job to estimate how many reserved transcoding slots (RTS) you need. When this is enabled, MediaConvert runs your job from an on-demand queue with similar performance to what you will see with one RTS in a reserved queue. This setting is disabled by default.
    pub fn simulate_reserved_queue(mut self, input: crate::types::SimulateReservedQueue) -> Self {
        self.simulate_reserved_queue = ::std::option::Option::Some(input);
        self
    }
    /// Optional. Enable this setting when you run a test job to estimate how many reserved transcoding slots (RTS) you need. When this is enabled, MediaConvert runs your job from an on-demand queue with similar performance to what you will see with one RTS in a reserved queue. This setting is disabled by default.
    pub fn set_simulate_reserved_queue(
        mut self,
        input: ::std::option::Option<crate::types::SimulateReservedQueue>,
    ) -> Self {
        self.simulate_reserved_queue = input;
        self
    }
    /// Optional. Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.
    pub fn status_update_interval(mut self, input: crate::types::StatusUpdateInterval) -> Self {
        self.status_update_interval = ::std::option::Option::Some(input);
        self
    }
    /// Optional. Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.
    pub fn set_status_update_interval(
        mut self,
        input: ::std::option::Option<crate::types::StatusUpdateInterval>,
    ) -> Self {
        self.status_update_interval = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// Optional. The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key. Use standard AWS tags on your job for automatic integration with AWS services and for custom integrations and workflows.
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// Optional. The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key. Use standard AWS tags on your job for automatic integration with AWS services and for custom integrations and workflows.
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Adds a key-value pair to `user_metadata`.
    ///
    /// To override the contents of this collection use [`set_user_metadata`](Self::set_user_metadata).
    ///
    /// Optional. User-defined metadata that you want to associate with an MediaConvert job. You specify metadata in key/value pairs. Use only for existing integrations or workflows that rely on job metadata tags. Otherwise, we recommend that you use standard AWS tags.
    pub fn user_metadata(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.user_metadata.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.user_metadata = ::std::option::Option::Some(hash_map);
        self
    }
    /// Optional. User-defined metadata that you want to associate with an MediaConvert job. You specify metadata in key/value pairs. Use only for existing integrations or workflows that rely on job metadata tags. Otherwise, we recommend that you use standard AWS tags.
    pub fn set_user_metadata(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.user_metadata = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateJobInput`](crate::operation::create_job::CreateJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_job::CreateJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_job::CreateJobInput {
            acceleration_settings: self.acceleration_settings,
            billing_tags_source: self.billing_tags_source,
            client_request_token: self.client_request_token,
            hop_destinations: self.hop_destinations,
            job_template: self.job_template,
            priority: self.priority,
            queue: self.queue,
            role: self.role,
            settings: self.settings,
            simulate_reserved_queue: self.simulate_reserved_queue,
            status_update_interval: self.status_update_interval,
            tags: self.tags,
            user_metadata: self.user_metadata,
        })
    }
}
