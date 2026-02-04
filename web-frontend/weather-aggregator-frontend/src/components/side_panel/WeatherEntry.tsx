
import type { CurrentWeather } from "../../model/CurrentWeather";
import type { UserPreferencesWithHistory } from "../../model/UserPreferencesWithLocationHistory";


type Props = {
  weather: CurrentWeather;
  userPreferencesWithHistory?: UserPreferencesWithHistory | null;
  onStarClick: (entry: CurrentWeather) => void;
};

export default function WeatherEntry({
  weather,
  userPreferencesWithHistory,
  onStarClick,

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
        <span>{ userPreferencesWithHistory?.preferences.unit_system === "METRIC" ? weather.weather.temp_metric : weather.weather.temp_imperial}°</span>

        <button
          onClick={() => onStarClick(weather)}
          style={styles.star}
          title={"Set as favorite" + (weather.isFavorite ? " (currently favorite)" : "")}
        >
          {
          weather.isFavorite ? "★" : "☆"
          }
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
};