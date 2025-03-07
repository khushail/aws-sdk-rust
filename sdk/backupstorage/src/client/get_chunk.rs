// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetChunk`](crate::operation::get_chunk::builders::GetChunkFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`storage_job_id(impl ::std::convert::Into<String>)`](crate::operation::get_chunk::builders::GetChunkFluentBuilder::storage_job_id) / [`set_storage_job_id(Option<String>)`](crate::operation::get_chunk::builders::GetChunkFluentBuilder::set_storage_job_id): Storage job id
    ///   - [`chunk_token(impl ::std::convert::Into<String>)`](crate::operation::get_chunk::builders::GetChunkFluentBuilder::chunk_token) / [`set_chunk_token(Option<String>)`](crate::operation::get_chunk::builders::GetChunkFluentBuilder::set_chunk_token): Chunk token
    /// - On success, responds with [`GetChunkOutput`](crate::operation::get_chunk::GetChunkOutput) with field(s):
    ///   - [`data(ByteStream)`](crate::operation::get_chunk::GetChunkOutput::data): Chunk data
    ///   - [`length(i64)`](crate::operation::get_chunk::GetChunkOutput::length): Data length
    ///   - [`checksum(Option<String>)`](crate::operation::get_chunk::GetChunkOutput::checksum): Data checksum
    ///   - [`checksum_algorithm(Option<DataChecksumAlgorithm>)`](crate::operation::get_chunk::GetChunkOutput::checksum_algorithm): Checksum algorithm
    /// - On failure, responds with [`SdkError<GetChunkError>`](crate::operation::get_chunk::GetChunkError)
    pub fn get_chunk(&self) -> crate::operation::get_chunk::builders::GetChunkFluentBuilder {
        crate::operation::get_chunk::builders::GetChunkFluentBuilder::new(self.handle.clone())
    }
}
