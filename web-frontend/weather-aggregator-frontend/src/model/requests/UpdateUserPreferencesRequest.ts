import type { UnitSystemType } from "../UserPreferencesWithLocationHistory";

export interface UpdateUserPreferencesRequest {
  user_id: string; // Uuid is handled as a string
  unit_system: UnitSystemType; // Assuming UnitSystemType is "metric" | "imperial"
  favorite_location_name?: string | null;
  favorite_lat?: number | null;
  favorite_lon?: number | null;
}

