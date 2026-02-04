export class ApiError extends Error {
  isServerError: boolean;
  status?: number;

  constructor(message: string, isServerError: boolean, status?: number, cause?: any) {
    super(message, { cause });
    this.isServerError = isServerError;
    this.status = status;
  }
}