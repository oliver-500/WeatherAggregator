import { useEffect, useState } from 'react';
import './App.css'
import Home from './pages/Home';
import type { UserPreferencesWithHistory } from './model/UserPreferencesWithLocationHistory';
import { getUserPreferencesWithHistory, updateUserPreferencesWithHistory } from './api/user';
import { Toaster } from 'react-hot-toast';
import TopBar from './components/TopBar';
import { getUserInfo, logoutUser, refreshAccessToken, registerAnonymousUser } from './api/auth';
import type { UserInfo } from './model/UserInfo';
import toast from 'react-hot-toast';
import type { UpdateUserPreferencesRequest } from './model/requests/UpdateUserPreferencesRequest';


function App() {
  const [userPreferencesWithHistory, setUserPreferencesWithHistory] = useState<UserPreferencesWithHistory | null>(null);
  const [userInfo, setUserInfo] = useState<UserInfo | null>(null);

  useEffect(() => {
    const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

    const initialize = async () => {
      let success = false;
      let retryDelay = 500; // Start with 2 seconds
      const MAX_DELAY = 30000; // Cap the delay at 30 seconds

      while (!success) {
        try {
          // 1. Core Goal: Get the info
          let info;

          if (!userInfo) {
            info = await getUserInfo();
          } else {
            info = userInfo;
          }

          
          setUserInfo(info);

          // 2. Secondary Goal: Get preferences
          if (info?.user_id) {
            try {
              const prefs = await getUserPreferencesWithHistory({ user_id: info.user_id });
              setUserPreferencesWithHistory(prefs);
              success = true; // <--- This breaks the loop
              toast.success("Connected!");
            } catch (prefErr) {
              // If prefs fail but info succeeded, we might still consider this "success"
              // depending on your requirements.
              console.error("Prefs failed, but continuing...", prefErr);
            }
          }

         

        } catch (err: any) {
          const status = err.status;

          if (status === 409) {
            await logoutUser();
            await registerAnonymousUser();
          }

          // Handle specific logic-based errors first
          if (status === 404 || status === 400) {
            await registerAnonymousUser();
            // Don't set success to true; loop will run again naturally
          } else if (status === 401) {
            try {
              await refreshAccessToken();
            } catch {
              toast.error("Session expired. Please log in again.");
              return; // Stop looping if user MUST take manual action
            }
          } else {
            // This is where the "Server Down" logic lives
            console.warn(`Server unavailable or error ${status}. Retrying in ${retryDelay / 1000}s...`);
            
            await sleep(retryDelay);
            
            // Optional: Exponential backoff (doubles delay each time until it hits MAX_DELAY)
            retryDelay = Math.min(retryDelay * 2, MAX_DELAY);
          }
        }
      }
    };
    initialize();
  }, []);

  
  const handleUnitChange = async (newSystem: "METRIC" | "IMPERIAL") => {
    // 1. Calculate the new state object locally
    // This ensures we aren't waiting for React's slow update cycle

    if (!userPreferencesWithHistory) {
      toast.error("User preferences not loaded.");
      return;
    }

    let old_state = userPreferencesWithHistory;

    const updatedPrefs : UserPreferencesWithHistory = {
      ...userPreferencesWithHistory,
      preferences: { 
        ...userPreferencesWithHistory.preferences, 
        unit_system: newSystem 
      }
    };

    // 2. Update the UI (Async - will happen whenever React is ready)
    setUserPreferencesWithHistory(updatedPrefs);

    // 3. Sync to Backend (Using the fresh local variable, NOT the state)
    try {
      console.log("Syncing fresh data to backend...");
      await syncUserPreferences(updatedPrefs); 
      toast.success("Preferences saved");
    } catch (err) {
      toast.error("Sync failed. Reverting...");
      // Optional: Revert UI state if the API fails
      setUserPreferencesWithHistory(old_state);
    }
  };


  const syncUserPreferences = async (prefs: UserPreferencesWithHistory | null) => {
    try {
      let req : UpdateUserPreferencesRequest = {
        ...prefs!.preferences,
        unit_system: prefs!.preferences.unit_system || "METRIC" // Default to METRIC if undefined,
      };
      await updateUserPreferencesWithHistory(req);

    } catch (err) {
      toast.error("Failed to sync preferences to cloud.");
    }
  };

  return (
    <div style={styles.app}>
      <Toaster position="bottom-center" reverseOrder={false} />
      <TopBar user_info={userInfo} userPreferencesWithHistory={userPreferencesWithHistory} onUnitChange={handleUnitChange} />
      <Home userPreferencesWithHistory={userPreferencesWithHistory} syncUserPreferences={syncUserPreferences} />   
    </div>
  )
}

export default App

const styles = {
  app: {
    height: "100vh",
    display: "flex",
    flexDirection: "column",
  },
} as const;
