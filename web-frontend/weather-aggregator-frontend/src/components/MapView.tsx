import { MapContainer, TileLayer, Marker, Popup, useMap, ZoomControl } from 'react-leaflet';
import 'leaflet/dist/leaflet.css';
import L from 'leaflet';

import markerIcon from 'leaflet/dist/images/marker-icon.png';
import markerShadow from 'leaflet/dist/images/marker-shadow.png';
import type { UserPreferencesWithHistory } from '../model/UserPreferencesWithLocationHistory';
import type { CurrentWeather } from '../model/CurrentWeather';
import { useEffect } from 'react';
import type { LocationOption } from '../model/LocationOption';



let DefaultIcon = L.icon({
    iconUrl: markerIcon,
    shadowUrl: markerShadow,
    iconSize: [25, 41],
    iconAnchor: [12, 41]
});
L.Marker.prototype.options.icon = DefaultIcon;

interface MapProps {
  current: CurrentWeather | null;
  isCurrentLoaded: boolean;
  userPreferencesWithHistory?: UserPreferencesWithHistory | null;
  currentSelectedLocationOption: LocationOption | null;
  setCurrentSelectedLocationOption: (entry: LocationOption) => void;
}


export default function MapView({
  current,
  userPreferencesWithHistory,
  isCurrentLoaded,
  currentSelectedLocationOption,
  setCurrentSelectedLocationOption
} : MapProps
  
) {

  if (!isCurrentLoaded) {
    return <div style={styles.container}>Loading Map...</div>;
  }

  const initialPosition: [number, number] = [current?.location.lat || 0, current?.location.lon || 0];


  return (
    <div style={styles.container}>


  <div style={styles.infoPanel} className="custom-scrollbar">
        <h3 style={{ margin: 0 }}>{currentSelectedLocationOption?.current_weather?.location.name || ""}</h3>
        <p style={{ fontSize: '0.8rem', color: '#666' }}>
          {currentSelectedLocationOption?.current_weather?.location.state_region_province_or_entity || ""},&nbsp; 
          {currentSelectedLocationOption?.current_weather?.location.country || ""}
        </p>
        
        
        
        <div>
          <strong>Temperature: </strong> 
          {userPreferencesWithHistory?.preferences.unit_system === "IMPERIAL" 
            ? `${currentSelectedLocationOption?.current_weather?.weather.temp_imperial ?? ""} °F`
          : `${currentSelectedLocationOption?.current_weather?.weather.temp_metric ?? ""} °C`
          }
          <br />
          <small>Feels like: {userPreferencesWithHistory?.preferences.unit_system === "IMPERIAL" 
            ? `${currentSelectedLocationOption?.current_weather?.weather.temp_feelslike_imperial ?? ""} °F`
          : `${currentSelectedLocationOption?.current_weather?.weather.temp_feelslike_metric ?? ""} °C`
          }</small>
        </div>

        <div>
          <strong>Condition:</strong> {currentSelectedLocationOption?.current_weather?.weather.condition || ""}
        </div>

        <div>
          <strong>Humidity:</strong> {currentSelectedLocationOption?.current_weather?.weather.humidity || ""}%
        </div>

        <div>
          <strong>Wind:</strong> {currentSelectedLocationOption?.current_weather?.wind.speed_metric || ""} km/h 
          ({currentSelectedLocationOption?.current_weather?.wind.direction || ""})
        </div>

        <div>
          <strong>Pressure:</strong> {currentSelectedLocationOption?.current_weather?.weather.pressure_metric || ""} hPa
        </div>
        
        {/* Add more info here; the panel will scroll if it gets too tall */}
      </div>

      <MapContainer 
        center={initialPosition} 
        zoom={12} 
        scrollWheelZoom={false} 
        style={{ height: '100%', width: '100%' }}
        zoomControl={false}
      >
        <ZoomControl position="bottomright" />
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
        <RecenterMap   lat={currentSelectedLocationOption?.current_weather?.location.lat || 0} lon={currentSelectedLocationOption?.current_weather?.location.lon || 0} />
        <Marker position={[currentSelectedLocationOption?.current_weather?.location.lat || current?.location.lat || 0,
        currentSelectedLocationOption?.current_weather?.location.lon || current?.location.lon || 0]}>
          <Popup>Selected Location</Popup>
        </Marker>
      </MapContainer>
    </div>
  );
}

const styles = {
  container: {
    flex: 1,
    backgroundColor: "#e5e5e5",
    position: "relative",
    width: "100%",
    height: "100%",
  },
  infoPanel: {
    position: 'absolute',
    top: '20px',
    left: '20px',
    zIndex: 1000, // Must be higher than Leaflet (typically < 1000)
    width: '280px',
    maxHeight: 'calc(100% - 40px)', // Ensures it doesn't leave the map area
    backgroundColor: 'rgba(255, 255, 255, 0.85)', // Glassmorphism/Transparent look
    backdropFilter: 'blur(4px)',
    padding: '15px',
    borderRadius: '8px',
    boxShadow: '0 4px 12px rgba(0,0,0,0.15)',
    overflowY: 'auto', // Adds scrollbar if content is too long
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
    pointerEvents: 'auto', // Important so you can scroll/click inside it
  }
} as const;

function RecenterMap({ lat, lon }: { lat: number; lon: number;}) {
  const map = useMap();

  useEffect(() => {

    if (lat === 0 || lon === 0) {
        return;
    }
    // .setView moves the map. .flyTo adds a smooth animation.
    map.flyTo([lat, lon], map.getZoom(), {
      duration: 1.5, // seconds
    });
  }, [lat, lon, map]);

  return null;
}