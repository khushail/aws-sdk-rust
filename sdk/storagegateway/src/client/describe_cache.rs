// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCache`](crate::operation::describe_cache::builders::DescribeCacheFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_cache::builders::DescribeCacheFluentBuilder::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::operation::describe_cache::builders::DescribeCacheFluentBuilder::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    /// - On success, responds with [`DescribeCacheOutput`](crate::operation::describe_cache::DescribeCacheOutput) with field(s):
    ///   - [`gateway_arn(Option<String>)`](crate::operation::describe_cache::DescribeCacheOutput::gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    ///   - [`disk_ids(Option<Vec<String>>)`](crate::operation::describe_cache::DescribeCacheOutput::disk_ids): <p>An array of strings that identify disks that are to be configured as working storage. Each string has a minimum length of 1 and maximum length of 300. You can get the disk IDs from the <code>ListLocalDisks</code> API.</p>
    ///   - [`cache_allocated_in_bytes(i64)`](crate::operation::describe_cache::DescribeCacheOutput::cache_allocated_in_bytes): <p>The amount of cache in bytes allocated to a gateway.</p>
    ///   - [`cache_used_percentage(f64)`](crate::operation::describe_cache::DescribeCacheOutput::cache_used_percentage): <p>Percent use of the gateway's cache storage. This metric applies only to the gateway-cached volume setup. The sample is taken at the end of the reporting period.</p>
    ///   - [`cache_dirty_percentage(f64)`](crate::operation::describe_cache::DescribeCacheOutput::cache_dirty_percentage): <p>The file share's contribution to the overall percentage of the gateway's cache that has not been persisted to Amazon Web Services. The sample is taken at the end of the reporting period.</p>
    ///   - [`cache_hit_percentage(f64)`](crate::operation::describe_cache::DescribeCacheOutput::cache_hit_percentage): <p>Percent of application read operations from the file shares that are served from cache. The sample is taken at the end of the reporting period.</p>
    ///   - [`cache_miss_percentage(f64)`](crate::operation::describe_cache::DescribeCacheOutput::cache_miss_percentage): <p>Percent of application read operations from the file shares that are not served from cache. The sample is taken at the end of the reporting period.</p>
    /// - On failure, responds with [`SdkError<DescribeCacheError>`](crate::operation::describe_cache::DescribeCacheError)
    pub fn describe_cache(
        &self,
    ) -> crate::operation::describe_cache::builders::DescribeCacheFluentBuilder {
        crate::operation::describe_cache::builders::DescribeCacheFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
