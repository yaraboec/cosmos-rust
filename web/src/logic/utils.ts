export function getErrorFromStackTrace(stackTrace: Error): string {
  const stringifiedStackTrace =
    stackTrace.message || stackTrace.stack || stackTrace.toString() || "";
  const error = stringifiedStackTrace.match(/([^;]*)$/)?.[0]?.trimStart();

  return error || stringifiedStackTrace;
}
