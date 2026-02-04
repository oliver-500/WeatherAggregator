export interface AddHistoryItemRequest {
  user_id: string; // Uuid is handled as a string
  location_name: string;
  lat: number;
  lon: number;
};