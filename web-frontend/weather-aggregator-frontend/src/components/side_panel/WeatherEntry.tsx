

import type { CurrentWeather } from "../../model/CurrentWeather";
import type { LocationOption } from "../../model/LocationOption";
import type { UserPreferencesWithHistory } from "../../model/UserPreferencesWithLocationHistory";


type Props = {
  weather: CurrentWeather;
  userPreferencesWithHistory?: UserPreferencesWithHistory | null;
  onStarClick: (entry: CurrentWeather) => void;
  setCurrentSelectedLocationOption : React.Dispatch<React.SetStateAction<LocationOption | null>>;
};

export default function WeatherEntry({
  weather,
  userPreferencesWithHistory,
  onStarClick,
  setCurrentSelectedLocationOption: setCurrentSelectedLocationOption

}: Props) {

  return (
    <div style={styles.entry}>
      <div>
        {weather.location.name}
        <small style={{ color: '#666' }}>, {weather.location.country}, {weather.location.state_region_province_or_entity}
        </small>
        <div> <i style={{ fontSize: '13px' }}>{weather.weather.condition}</i></div>
      </div>

      <div style={styles.right}>
        <span>{ userPreferencesWithHistory?.preferences.unit_system === "METRIC" ? weather.weather.temp_metric.toFixed(0) : weather.weather.temp_imperial.toFixed(0)}°</span>

        <button
          onClick={() => onStarClick(weather)}
          style={styles.star}
          title={"Set as favorite" + (weather.isFavorite ? " (currently favorite)" : "")}
        >
          {
          weather.isFavorite ? "★" : "☆"
          }
        </button>

        {/* Details Button (Three Dots) */}
        <button
          onClick={() => 
            setCurrentSelectedLocationOption({
              location_name: weather.location.name ?? null,
              state: weather.location.state_region_province_or_entity ?? null,
              country: weather.location.country ?? null,
              lat: weather.location.lat,
              lon: weather.location.lon,
              current_weather: weather,

            })
          }
          style={styles.iconButton}
          title="Show more details"
        >
          ⋮
        </button>

        
      </div>
    </div>
  );
}

const styles = {
  entry: {
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    marginTop: "8px",
  },
  right: {
    display: "flex",
    alignItems: "center",
    gap: "8px",
  },
  star: {
    background: "none",
    border: "none",
    fontSize: "18px",
    cursor: "pointer",
  },
  iconButton: {
    background: "none",
    border: "none",
    fontSize: "20px",
    cursor: "pointer",
    padding: "4px 8px",
    color: "#444",
    borderRadius: "4px",
    transition: "background 0.2s",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
  },
} as const;

