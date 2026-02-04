import type { CurrentWeather } from "../../model/CurrentWeather";
import type { UserPreferencesWithHistory } from "../../model/UserPreferencesWithLocationHistory";

import WeatherEntry from "./WeatherEntry";

type Props = {
  current: CurrentWeather | null;
  onStarClick: (entry: CurrentWeather) => void;
  userPreferencesWithHistory?: UserPreferencesWithHistory | null;
  favorite: CurrentWeather | null;
};

export default function CurrentLocation({
    current,
    onStarClick,
    userPreferencesWithHistory
}: Props) {
  return (
    <section style={styles.section}>
      <h3>Current Location</h3>
      {current ? (
        <WeatherEntry
          weather={current}
          userPreferencesWithHistory={userPreferencesWithHistory}
          onStarClick={onStarClick}
        />
      ) : (
      <div style={styles.errorMessage}>
        <p>Current location could not be determined.</p>
        <span style={{ fontSize: '12px', color: '#888' }}>
          Please check your connection.
        </span>
      </div>
    )}
</section>
  );
}


const styles = {
  section: {
    padding: "16px",
    borderBottom: "1px solid #eee",
  },
  entry: {
    display: "flex",
    justifyContent: "space-between",
    marginTop: "8px",
  },
  errorMessage: {
    display: "flex",
    flexDirection: "column" as "column",
    alignItems: "center",
    justifyContent: "center",
    padding: "40px",
    backgroundColor: "#f9f9f9",
    borderRadius: "12px",
    border: "1px dashed #ccc",
    textAlign: "center" as "center",
    color: "#555",
    margin: "20px 0",
 },
};

