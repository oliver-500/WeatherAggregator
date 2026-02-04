export interface GenericServiceError {
  error: {
    code: string;         // Matches UserPreferencesServiceError
    code_numeric: number; // Matches u16
    message: string;      // This is what you want to print!
    timestamp: string;    // Matches DateTime<Utc>
  };
}