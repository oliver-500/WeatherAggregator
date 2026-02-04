import type { CurrentWeather } from "../../model/CurrentWeather";
import type { UserPreferencesWithHistory } from "../../model/UserPreferencesWithLocationHistory";
import WeatherEntry from "./WeatherEntry";

type Props = {
  onStarClick: (entry: CurrentWeather) => void;
  userPreferencesWithHistory?: UserPreferencesWithHistory | null;
  favorite: CurrentWeather | null;
};

export default function FavoriteLocation({
    userPreferencesWithHistory,
    onStarClick,
    favorite
}: Props) {

  return (
    <section style={styles.section}>
      <h3>Favorite</h3>
      {favorite ? (
        <WeatherEntry
          weather={favorite}
          userPreferencesWithHistory={userPreferencesWithHistory}
          onStarClick={onStarClick}
        />
      ) : (
        <div style={styles.errorMessage}>
          <p>No favorite location set.</p>
          <span style={{ fontSize: '12px', color: '#888' }}>
            Add a favorite location by clicking star icon next to one.
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
    padding: "10px",
    backgroundColor: "#f9f9f9",
    borderRadius: "12px",
    border: "1px dashed #ccc",
    textAlign: "center" as "center",
    color: "#555",
    margin: "20px 0",
  },
};