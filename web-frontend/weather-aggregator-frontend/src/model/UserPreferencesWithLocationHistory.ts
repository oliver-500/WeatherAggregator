export interface UserPreferencesWithHistory {
  preferences: UserPreferencesEntity;
  history: LocationHistoryEntity[];
}

export interface UserPreferencesEntity {
  user_id: string; // Uuid is handled as a string
  user_type: UserType | null; // Assuming UserType is an enum or string
  unit_system: UnitSystemType; // Assuming UnitSystemType is "metric" | "imperial"
  favorite_location_name?: string | null;
  favorite_lat?: number | null;
  favorite_lon?: number | null;
  updated_at: string; // ISO 8601 Date string
}

export interface LocationHistoryEntity {
  lat: number;
  lon: number;
  location_name: string;
  id: string;
  user_id: string;
  searched_at: string; // ISO 8601 Date string
}

// Helper enums based on common Rust weather app patterns
export type UnitSystemType = 'METRIC' | 'IMPERIAL';
export type UserType = 'GUEST' | 'STANDARD';