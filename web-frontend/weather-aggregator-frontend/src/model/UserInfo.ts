import type { UserType } from "./UserPreferencesWithLocationHistory";

export interface UserInfo {
    email: string | null,
    user_type: UserType,
    user_id: string
}