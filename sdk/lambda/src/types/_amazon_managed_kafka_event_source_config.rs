// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specific configuration settings for an Amazon Managed Streaming for Apache Kafka (Amazon MSK) event source.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AmazonManagedKafkaEventSourceConfig {
    /// <p>The identifier for the Kafka consumer group to join. The consumer group ID must be unique among all your Kafka event sources. After creating a Kafka event source mapping with the consumer group ID specified, you cannot update this value. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-msk.html#services-msk-consumer-group-id">Customizable consumer group ID</a>.</p>
    #[doc(hidden)]
    pub consumer_group_id: ::std::option::Option<::std::string::String>,
}
impl AmazonManagedKafkaEventSourceConfig {
    /// <p>The identifier for the Kafka consumer group to join. The consumer group ID must be unique among all your Kafka event sources. After creating a Kafka event source mapping with the consumer group ID specified, you cannot update this value. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-msk.html#services-msk-consumer-group-id">Customizable consumer group ID</a>.</p>
    pub fn consumer_group_id(&self) -> ::std::option::Option<&str> {
        self.consumer_group_id.as_deref()
    }
}
impl AmazonManagedKafkaEventSourceConfig {
    /// Creates a new builder-style object to manufacture [`AmazonManagedKafkaEventSourceConfig`](crate::types::AmazonManagedKafkaEventSourceConfig).
    pub fn builder() -> crate::types::builders::AmazonManagedKafkaEventSourceConfigBuilder {
        crate::types::builders::AmazonManagedKafkaEventSourceConfigBuilder::default()
    }
}

/// A builder for [`AmazonManagedKafkaEventSourceConfig`](crate::types::AmazonManagedKafkaEventSourceConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AmazonManagedKafkaEventSourceConfigBuilder {
    pub(crate) consumer_group_id: ::std::option::Option<::std::string::String>,
}
impl AmazonManagedKafkaEventSourceConfigBuilder {
    /// <p>The identifier for the Kafka consumer group to join. The consumer group ID must be unique among all your Kafka event sources. After creating a Kafka event source mapping with the consumer group ID specified, you cannot update this value. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-msk.html#services-msk-consumer-group-id">Customizable consumer group ID</a>.</p>
    pub fn consumer_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.consumer_group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the Kafka consumer group to join. The consumer group ID must be unique among all your Kafka event sources. After creating a Kafka event source mapping with the consumer group ID specified, you cannot update this value. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-msk.html#services-msk-consumer-group-id">Customizable consumer group ID</a>.</p>
    pub fn set_consumer_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.consumer_group_id = input;
        self
    }
    /// Consumes the builder and constructs a [`AmazonManagedKafkaEventSourceConfig`](crate::types::AmazonManagedKafkaEventSourceConfig).
    pub fn build(self) -> crate::types::AmazonManagedKafkaEventSourceConfig {
        crate::types::AmazonManagedKafkaEventSourceConfig {
            consumer_group_id: self.consumer_group_id,
        }
    }
}
