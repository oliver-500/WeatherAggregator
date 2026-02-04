import { useEffect, useState } from 'react';
import  { getWeatherDataByCoordinates, getWeatherDataByIpAddress }  from '../api/weather';

import 'leaflet/dist/leaflet.css';
import toast from 'react-hot-toast';
import SidePanel from '../components/SidePanel';
import MapView from '../components/MapView';
import type { CurrentWeather } from '../model/CurrentWeather';
import { addHistoryItem } from '../api/user';
import type { UserPreferencesWithHistory } from '../model/UserPreferencesWithLocationHistory';


type HomeProps = {
  userPreferencesWithHistory: UserPreferencesWithHistory | null;
  syncUserPreferences: (prefs: UserPreferencesWithHistory | null) => void;
};

export default function Home({ 
  userPreferencesWithHistory, 
  syncUserPreferences 
}: HomeProps) {

  const [current, setCurrent] = useState<CurrentWeather | null>(null);
  const [history, setHistory] =  useState<CurrentWeather[]>([]);
  const [favorite, setFavorite] =  useState<CurrentWeather|null>(null);
  const [historyLoaded, setHistoryLoaded] = useState(false);

  const normalize = (num: number) => num.toFixed(2);

  const isCurrentFav = current &&
    userPreferencesWithHistory?.preferences.favorite_lat && 
    userPreferencesWithHistory?.preferences.favorite_lon &&
    userPreferencesWithHistory?.preferences.favorite_location_name &&
    current.location.name === userPreferencesWithHistory?.preferences.favorite_location_name &&
    normalize(current.location.lat) === normalize(userPreferencesWithHistory.preferences.favorite_lat) &&
    normalize(current.location.lon) === normalize(userPreferencesWithHistory.preferences.favorite_lon);

  useEffect(() => {
    getWeatherDataByIpAddress().then((res) => {
      setCurrent(res);
    });
  }, []);


  useEffect(() => {
    if (!userPreferencesWithHistory) {
      setHistory([]);
      return;
    }

    const loadHistoryData = async () => {
      const historyItems = userPreferencesWithHistory.history;

      // A. Fetch all history weather in parallel
      const historyPromises = historyItems.map(item => 
        getWeatherDataByCoordinates(item.lat, item.lon)
      );

      const resolvedHistory = await Promise.all(historyPromises);

      const prefData = userPreferencesWithHistory.preferences;

      const processedHistory = resolvedHistory.map(data => ({
        ...data,
        isFavorite: (
          normalize(data.location.lat) === normalize(prefData.favorite_lat || 0) &&
          normalize(data.location.lon) === normalize(prefData.favorite_lon || 0) &&
          data.location.name === (prefData.favorite_location_name)
        )
      }));

      setHistory(processedHistory);
      setHistoryLoaded(true);

    };

    const loadFavoriteData = async () => {
      if (userPreferencesWithHistory.preferences.favorite_lat && userPreferencesWithHistory.preferences.favorite_lon) {
        getWeatherDataByCoordinates(
          userPreferencesWithHistory.preferences.favorite_lat, 
          userPreferencesWithHistory.preferences.favorite_lon,
        ).then(data => {
          if (data.location.name) {
            data.isFavorite = true;
            setFavorite(data);
          }
        });
      }
    };

    const loadAllData = async () => {
      await loadFavoriteData();
      await loadHistoryData();
    };

    loadAllData();
    

  }, [userPreferencesWithHistory]);

// 2. Process History & Favorites when Preferences or Current changes
  useEffect(() => {
    // Guard: We need preferences to know WHAT history to fetch
      if (!userPreferencesWithHistory) return;
      if (!historyLoaded) return;
      if (!current) return;

      // C. Handle the "Current" location logic
    if (current) {
      console.log("op", history.length);
      const isCurrentInHistory = history.some(item => {
          console.log(
          "Comparing history item:",
           item.location.name, 
           item.location.lat, " ",
           item.location.lon, " ",
           "with current:", 
           current.location.name, " ", 
           current.location.lat, " ",
           current.location.lon
          );
        return normalize(item.location.lat) === normalize(current.location.lat) &&
        normalize(item.location.lon) === normalize(current.location.lon) &&
        item.location.name === current.location.name
      }
        
      );

      console.log("isCurrentInHistory", isCurrentInHistory);
      if (!isCurrentInHistory) {
        console.log("jao");
        // Add to local state
        history.push({ ...current, isFavorite: false });
        
        // Sync to backend
        addHistoryItem({
          user_id: userPreferencesWithHistory.preferences.user_id,
          location_name: current.location.name || "Unknown Location",
          lat: current.location.lat,
          lon: current.location.lon
        });
      }
    }

  
  }, [userPreferencesWithHistory, historyLoaded, current]);
  


function handleStarClick(entry: CurrentWeather) {
  //ako je vec omiljena, ukloni je
  if (entry.isFavorite && userPreferencesWithHistory) {
     userPreferencesWithHistory.preferences.favorite_lat = null;
     userPreferencesWithHistory.preferences.favorite_lon = null;
     userPreferencesWithHistory.preferences.favorite_location_name = null;
     syncUserPreferences(userPreferencesWithHistory);
     entry.isFavorite = false;
     setFavorite(null);

     history.forEach(item => {
       if (
        item.location.name === entry.location.name &&
        normalize(item.location.lat) === normalize(entry.location.lat) &&
        normalize(item.location.lon) === normalize(entry.location.lon)
       ) {
         item.isFavorite = false;
       }
      });


     toast.success("Successfully removed from favorites.");
     return;
  }

 

  //ako nije omiljena, postavi je kao omiljenu
  if (
    userPreferencesWithHistory?.preferences.favorite_lat
    &&  userPreferencesWithHistory?.preferences.favorite_lon
    && normalize(userPreferencesWithHistory.preferences.favorite_lat) !== normalize(entry.location.lat)
    && normalize(userPreferencesWithHistory.preferences.favorite_lon) !== normalize(entry.location.lon)
  ) {
    userPreferencesWithHistory.preferences.favorite_lat = entry.location.lat;
    userPreferencesWithHistory.preferences.favorite_lon = entry.location.lon;
    userPreferencesWithHistory.preferences.favorite_location_name = entry.location.name;
    entry.isFavorite = true;
    setFavorite(entry);
    toast.success("Successfully set as favorite.");
    syncUserPreferences(userPreferencesWithHistory);

    history.forEach(item => {
       if (
        item.location.name === entry.location.name &&
        normalize(item.location.lat) === normalize(entry.location.lat) &&
        normalize(item.location.lon) === normalize(entry.location.lon)
       ) {
         item.isFavorite = true;
       }
      });


    return;
  }
 
  if (!userPreferencesWithHistory) {
    toast.error("User preferences not loaded.");
    return;
  }

  if (!userPreferencesWithHistory?.preferences.favorite_lat
    && !userPreferencesWithHistory?.preferences.favorite_lon) {
      userPreferencesWithHistory.preferences.favorite_lat = entry.location.lat;
      userPreferencesWithHistory.preferences.favorite_lon = entry.location.lon;
      userPreferencesWithHistory.preferences.favorite_location_name = entry.location.name;
      syncUserPreferences(userPreferencesWithHistory);
        entry.isFavorite = true;

        history.forEach(item => {
       if (
        item.location.name === entry.location.name &&
        normalize(item.location.lat) === normalize(entry.location.lat) &&
        normalize(item.location.lon) === normalize(entry.location.lon)
       ) {
         item.isFavorite = true;
       }
      });

      setFavorite(entry);
      toast.success("Successfully set as favorite.");
    }
  
}

  return (
    <div style={styles.main_home}>
      <MapView />
      <SidePanel       
        current={current ? { ...current, isFavorite: !!isCurrentFav } : null}
        userPreferencesWithHistory={userPreferencesWithHistory}
        onStarClick={handleStarClick}
        favorite={favorite}
        history={history}
        onHistoryChange={setHistory}
      />  
    </div> 
  );
}



const styles = {
  main_home: {
    flex: 1,
    display: "flex",
    overflow: "hidden",
  },
} as const;