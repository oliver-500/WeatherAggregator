import { useEffect, useState } from 'react';
import  { getWeatherDataByCoordinates, getWeatherDataByIpAddress }  from '../api/weather';

import 'leaflet/dist/leaflet.css';
import toast from 'react-hot-toast';
import SidePanel from '../components/SidePanel';
import MapView from '../components/MapView';
import type { CurrentWeather } from '../model/CurrentWeather';
import { addHistoryItem } from '../api/user';
import type { UserPreferencesWithHistory } from '../model/UserPreferencesWithLocationHistory';
import type { LocationOption } from '../model/LocationOption';


type HomeProps = {
  userPreferencesWithHistory: UserPreferencesWithHistory | null;
  syncUserPreferences: (prefs: UserPreferencesWithHistory | null) => void;
  currentSelectedLocationOption: LocationOption | null;
  setCurrentSelectedLocationOption : React.Dispatch<React.SetStateAction<LocationOption | null>>
  locationHistory: CurrentWeather[];
  setLocationHistory: React.Dispatch<React.SetStateAction<CurrentWeather[]>>
};

export default function Home({ 
  userPreferencesWithHistory, 
  syncUserPreferences,
  currentSelectedLocationOption,
  setCurrentSelectedLocationOption,
  setLocationHistory,
  locationHistory
}: HomeProps) {

  const [current, setCurrent] = useState<CurrentWeather | null>(null);
  const [isCurrentLoaded, setIsCurrentLoaded] = useState<boolean>(false);
  
  const [favorite, setFavorite] =  useState<CurrentWeather|null>(null);

  const normalize = (num: number) => num.toFixed(2);

  const isCurrentFav = current &&
    userPreferencesWithHistory?.preferences.favorite_lat && 
    userPreferencesWithHistory?.preferences.favorite_lon &&
    userPreferencesWithHistory?.preferences.favorite_location_name &&
    current.location.name === userPreferencesWithHistory?.preferences.favorite_location_name &&
    normalize(current.location.lat) === normalize(userPreferencesWithHistory.preferences.favorite_lat) &&
    normalize(current.location.lon) === normalize(userPreferencesWithHistory.preferences.favorite_lon);


  useEffect(() => {
    if(!userPreferencesWithHistory || !currentSelectedLocationOption) {
      return 
    }

    if (currentSelectedLocationOption.lat !== null && currentSelectedLocationOption.lon !== null) {
      let lat = currentSelectedLocationOption.lat;
      let lon = currentSelectedLocationOption.lon;

      console.log(lat + " "  + lon);
      const isNewLocationAlreadyInHistory = userPreferencesWithHistory.history.some(item => {
        return normalize(item.lat) === normalize(lat) &&
          normalize(item.lon) === normalize(lon)
      });
    
      if (isNewLocationAlreadyInHistory) {
        return;
      }
          
      addHistoryItem({
        user_id: userPreferencesWithHistory.preferences.user_id,
        location_name: currentSelectedLocationOption.location_name || "Unknown Location",
        lat: lat,
        lon: lon,
      }).then(() => {
        console.log("uspjesno");
        setLocationHistory(prev => [
          ...prev, 
          { 
            ...currentSelectedLocationOption.current_weather, 
            isFavorite: false 
          } as CurrentWeather // Tell TS: "Trust me, this is a CurrentWeather object"
        ]);
      });
    }
  }, [currentSelectedLocationOption])

  useEffect(() => {
    if (!userPreferencesWithHistory) {
      return;
    }
    setFavorite(null);

    const fetchCurrentData = async () => {
      try {
        const res = await getWeatherDataByIpAddress();
        setCurrent(res);

        setCurrentSelectedLocationOption({
          current_weather: res,
          location_name: res.location.name,
          state: res.location.state_region_province_or_entity,
          country: res.location.country,
          lat: res.location.lat,
          lon: res.location.lon
        });
      } catch (err: any) {
        console.error("Failed to fetch weather", err);
      } finally {    
        setIsCurrentLoaded(true);
      }
    };

    fetchCurrentData();

    const loadFavoriteData = async () => {
      if (userPreferencesWithHistory.preferences.favorite_lat && userPreferencesWithHistory.preferences.favorite_lon) {
        await getWeatherDataByCoordinates(
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

    const loadHistoryData = async () => {
      const historyItems = userPreferencesWithHistory.history;
      const historyPromises = historyItems.map(async item => 
        await getWeatherDataByCoordinates(item.lat, item.lon)
      );
      const resolvedHistory = await Promise.all(historyPromises);

      const prefData = userPreferencesWithHistory.preferences;
      const processedHistory = resolvedHistory.map(data => {
        console.log(data.location.name)
        return ({
          ...data,
          isFavorite: (
            normalize(data.location.lat) === normalize(prefData.favorite_lat || 0) &&
            normalize(data.location.lon) === normalize(prefData.favorite_lon || 0)
          )
        });
      });
      console.log("Processed history:", processedHistory);
      setLocationHistory(processedHistory);
    };

    const loadAllData = async () => {
      console.log("Loading favorite data...");
      await loadFavoriteData();
      console.log("Loading favorite data done.");
      console.log("Loading history data...");
      await loadHistoryData();
      console.log("Loading history data done.");
    };

    loadAllData();
  }, [userPreferencesWithHistory]);



  useEffect(() => {
    if (!userPreferencesWithHistory || !current) return;
  
    const isCurrentAlreadyInHistory = userPreferencesWithHistory.history.some(item => {
      return normalize(item.lat) === normalize(current.location.lat) &&
        normalize(item.lon) === normalize(current.location.lon)
    });

    if (isCurrentAlreadyInHistory) {
      return;
    }

    // 5. Update State and Backend
   
    
    addHistoryItem({
      user_id: userPreferencesWithHistory.preferences.user_id,
      location_name: current.location.name || "Unknown Location",
      lat: current.location.lat,
      lon: current.location.lon
    }).then(() => {
      console.log("uspjesno dodan item u historiju");
       setLocationHistory(prev => [...prev, { ...current, isFavorite: false }]);
    })

    // Note: history is removed from dependencies to prevent the loop
  }, [userPreferencesWithHistory, current]);


function handleStarClick(entry: CurrentWeather) {
  //ako je vec omiljena, ukloni je
  if (entry.isFavorite && userPreferencesWithHistory) {
     userPreferencesWithHistory.preferences.favorite_lat = null;
     userPreferencesWithHistory.preferences.favorite_lon = null;
     userPreferencesWithHistory.preferences.favorite_location_name = null;
     syncUserPreferences(userPreferencesWithHistory);
     entry.isFavorite = false;
     setFavorite(null);

     locationHistory.forEach(item => {
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

    locationHistory.forEach(item => {
       if (
        item.location.name === entry.location.name &&
        normalize(item.location.lat) === normalize(entry.location.lat) &&
        normalize(item.location.lon) === normalize(entry.location.lon)
       ) {
         item.isFavorite = true;
       }
       else item.isFavorite = false;
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

        locationHistory.forEach(item => {
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
      <MapView 
      current={current} 
      userPreferencesWithHistory={userPreferencesWithHistory} 
      isCurrentLoaded={isCurrentLoaded} 
      currentSelectedLocationOption={currentSelectedLocationOption}
      setCurrentSelectedLocationOption={setCurrentSelectedLocationOption}
      />
      <SidePanel       
        current={current ? { ...current, isFavorite: !!isCurrentFav } : null}
        userPreferencesWithHistory={userPreferencesWithHistory}
        onStarClick={handleStarClick}
        favorite={favorite}
        history={locationHistory}
        setCurrentSelectedLocationOption={setCurrentSelectedLocationOption}
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