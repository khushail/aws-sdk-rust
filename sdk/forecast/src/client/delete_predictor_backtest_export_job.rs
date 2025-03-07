// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeletePredictorBacktestExportJob`](crate::operation::delete_predictor_backtest_export_job::builders::DeletePredictorBacktestExportJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`predictor_backtest_export_job_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_predictor_backtest_export_job::builders::DeletePredictorBacktestExportJobFluentBuilder::predictor_backtest_export_job_arn) / [`set_predictor_backtest_export_job_arn(Option<String>)`](crate::operation::delete_predictor_backtest_export_job::builders::DeletePredictorBacktestExportJobFluentBuilder::set_predictor_backtest_export_job_arn): <p>The Amazon Resource Name (ARN) of the predictor backtest export job to delete.</p>
    /// - On success, responds with [`DeletePredictorBacktestExportJobOutput`](crate::operation::delete_predictor_backtest_export_job::DeletePredictorBacktestExportJobOutput)
    /// - On failure, responds with [`SdkError<DeletePredictorBacktestExportJobError>`](crate::operation::delete_predictor_backtest_export_job::DeletePredictorBacktestExportJobError)
    pub fn delete_predictor_backtest_export_job(&self) -> crate::operation::delete_predictor_backtest_export_job::builders::DeletePredictorBacktestExportJobFluentBuilder{
        crate::operation::delete_predictor_backtest_export_job::builders::DeletePredictorBacktestExportJobFluentBuilder::new(self.handle.clone())
    }
}
