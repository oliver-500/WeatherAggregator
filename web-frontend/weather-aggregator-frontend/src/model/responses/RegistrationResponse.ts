import type { UserType } from "../UserPreferencesWithLocationHistory";

export interface RegistrationResponse {
    user_id: string;
    user_type: UserType;
    user_email: string;
}