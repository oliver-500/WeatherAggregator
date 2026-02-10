import { useCallback, useEffect, useState } from 'react';
import './App.css'
import Home from './pages/Home';
import type { UserPreferencesWithHistory } from './model/UserPreferencesWithLocationHistory';
import { getUserPreferencesWithHistory, updateUserPreferencesWithHistory } from './api/user';
import { Toaster } from 'react-hot-toast';
import TopBar from './components/TopBar';
import { getUserInfo, refreshAccessToken, registerAnonymousUser, logoutUser} from './api/auth';
import type { UserInfo } from './model/UserInfo';
import toast from 'react-hot-toast';
import type { UpdateUserPreferencesRequest } from './model/requests/UpdateUserPreferencesRequest';

import type { LocationOption } from './model/LocationOption';


function App() {
  const [userPreferencesWithHistory, setUserPreferencesWithHistory] = useState<UserPreferencesWithHistory | null>(null);
  const [userInfo, setUserInfo] = useState<UserInfo | null>(null);
  const [currentSelectedLocationOption, setCurrentSelectedLocationOption] = useState<LocationOption | null>(null);

  const initializeUserRelatedInfo = useCallback(async (isReinitialization: boolean) => {
      const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

      let success = false;
      let retryDelay = 500; // Start with 2 seconds
      const MAX_DELAY = 30000; // Cap the delay at 30 seconds

      while (!success) {
        try {
          // 1. Core Goal: Get the info
          let info;

          if (isReinitialization || !userInfo) {
            info = await getUserInfo();
          } else {
            info = userInfo;
          }        
          setUserInfo(info);

          // 2. Secondary Goal: Get preferences
          if (info?.user_id) {
            try {
              console.log("Fetching user preferences...");
              const prefs = await getUserPreferencesWithHistory(
                { 
                  user_id: info.user_id 
                }
              );
              setUserPreferencesWithHistory(prefs);

              success = true; // <--- This breaks the loop
              toast.success("Connected!");
            } catch (prefErr) {
              throw prefErr;
            }
          }

        } catch (err: any) {
          const status = err.status;

          //u slucaju da user sa id-om u cookie-u ne postoji
          if (status === 409) {
            await logoutUser();
            setUserInfo(null);
            setUserPreferencesWithHistory(null);
            alert("1");
            await registerAnonymousUser();
            await sleep(retryDelay);
          }

          // Handle specific logic-based errors first
          if (status === 400) {
            alert("2");
            await registerAnonymousUser();
            await sleep(retryDelay);
            // Don't set success to true; loop will run again naturally
          } else if (status === 401) {
            try {
              try {
                if(isReinitialization) {
                  alert("3");
                  await registerAnonymousUser();
                }
              }
              catch(err) {
                continue;
              }       
              if(!isReinitialization) await refreshAccessToken();          
              
            } catch(err) {           
              toast.error("Session expired. Log in again.");
              await logoutUser();
              setUserInfo(null);
              setUserPreferencesWithHistory(null);
              alert("4");
              await registerAnonymousUser();    
              await sleep(retryDelay);    
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
    }, []); // Empty array means "only create this once"

 

  useEffect(() => {
    initializeUserRelatedInfo(false);
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
      console.log("12");
    } catch (err) {
      console.log("1");
      toast.error("Failed to sync preferences to cloud.");
      throw err;
    }
  };

  return (
    <div style={styles.app}>
      <Toaster position="bottom-center" reverseOrder={false} />
      <TopBar
      user_info={userInfo} 
      userPreferencesWithHistory={userPreferencesWithHistory} 
      onUnitChange={handleUnitChange} 
      setUserInfo={setUserInfo}
      setUserPreferencesWithHistory={setUserPreferencesWithHistory}
      initializeUserRelatedInfo={initializeUserRelatedInfo}
      setCurrentSelectedLocationOption={setCurrentSelectedLocationOption} 
      />
      <Home 
      userPreferencesWithHistory={userPreferencesWithHistory} 
      syncUserPreferences={syncUserPreferences}
      currentSelectedLocationOption={currentSelectedLocationOption}
      setCurrentSelectedLocationOption={setCurrentSelectedLocationOption} 
      />   
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
