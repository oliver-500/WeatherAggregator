
import type { CurrentWeather } from "../../model/CurrentWeather";
import type { UserPreferencesWithHistory } from "../../model/UserPreferencesWithLocationHistory";


import WeatherEntry from "./WeatherEntry";


type Props = {
  onStarClick: (entry: CurrentWeather) => void;
  userPreferencesWithHistory?: UserPreferencesWithHistory | null;
  favorite: CurrentWeather | null;
  history: CurrentWeather[];
  onHistoryChange?: (history: CurrentWeather[]) => void;
};


export default function HistoryLocations(
    {
      onStarClick,
      userPreferencesWithHistory,
      history,
     }: Props
) {



  // useEffect(() => {
  //   userPreferencesWithHistory?.history.forEach(item => {
  //     let res = getWeatherDataByCoordinates(item.lat, item.lon);

  //     res.then(data => {
  //       data.isFavorite = favorite?.location.name === data.location.name;
  //       onHistoryChange && onHistoryChange([...history, data]);
  //     });

  //   });

  // }, [userPreferencesWithHistory]);


  return (
  <section style={styles.section}>
    <h3>History</h3>

    {history.length > 0 ? (
      history.map((item, index) => (
        <div key={index} >
          <WeatherEntry
            weather={item}
            userPreferencesWithHistory={userPreferencesWithHistory}
            onStarClick={onStarClick}
          />
        </div>
      ))
    ) : (
      <div style={styles.errorMessage}>
        <p>No history locations.</p>
        <span style={{ fontSize: '12px', color: '#888' }}>
          Use the search bar to add locations.
        </span>
      </div>
    )} {/* <--- The parenthesis here was missing! */}
  </section>
  );
}

const styles = {
  section: {
    padding: "16px",
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