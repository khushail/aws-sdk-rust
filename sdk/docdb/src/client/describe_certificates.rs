// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCertificates`](crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`certificate_identifier(impl ::std::convert::Into<String>)`](crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder::certificate_identifier) / [`set_certificate_identifier(Option<String>)`](crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder::set_certificate_identifier): <p>The user-supplied certificate identifier. If this parameter is specified, information for only the specified certificate is returned. If this parameter is omitted, a list of up to <code>MaxRecords</code> certificates is returned. This parameter is not case sensitive.</p>  <p>Constraints</p>  <ul>   <li> <p>Must match an existing <code>CertificateIdentifier</code>.</p> </li>  </ul>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder::set_filters): <p>This parameter is not currently supported.</p>
    ///   - [`max_records(i32)`](crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder::max_records) / [`set_max_records(Option<i32>)`](crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder::set_max_records): <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>  <p>Default: 100</p>  <p>Constraints:</p>  <ul>   <li> <p>Minimum: 20</p> </li>   <li> <p>Maximum: 100</p> </li>  </ul>
    ///   - [`marker(impl ::std::convert::Into<String>)`](crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder::set_marker): <p>An optional pagination token provided by a previous <code>DescribeCertificates</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    /// - On success, responds with [`DescribeCertificatesOutput`](crate::operation::describe_certificates::DescribeCertificatesOutput) with field(s):
    ///   - [`certificates(Option<Vec<Certificate>>)`](crate::operation::describe_certificates::DescribeCertificatesOutput::certificates): <p>A list of certificates for this Amazon Web Services account.</p>
    ///   - [`marker(Option<String>)`](crate::operation::describe_certificates::DescribeCertificatesOutput::marker): <p>An optional pagination token provided if the number of records retrieved is greater than <code>MaxRecords</code>. If this parameter is specified, the marker specifies the next record in the list. Including the value of <code>Marker</code> in the next call to <code>DescribeCertificates</code> results in the next page of certificates.</p>
    /// - On failure, responds with [`SdkError<DescribeCertificatesError>`](crate::operation::describe_certificates::DescribeCertificatesError)
    pub fn describe_certificates(
        &self,
    ) -> crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder {
        crate::operation::describe_certificates::builders::DescribeCertificatesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
