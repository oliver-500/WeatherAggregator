

import type { CurrentWeather } from "../../model/CurrentWeather";
import type { LocationOption } from "../../model/LocationOption";
import type { UserPreferencesWithHistory } from "../../model/UserPreferencesWithLocationHistory";


import WeatherEntry from "./WeatherEntry";


type Props = {
  onStarClick: (entry: CurrentWeather) => void;
  userPreferencesWithHistory?: UserPreferencesWithHistory | null;
  favorite: CurrentWeather | null;
  history: CurrentWeather[];
  setCurrentSelectedLocationOption : React.Dispatch<React.SetStateAction<LocationOption | null>>;
};


export default function HistoryLocations(
    {
      onStarClick,
      userPreferencesWithHistory,
      history,
      setCurrentSelectedLocationOption
     }: Props
) {





  return (
  <section style={styles.section}>
    <h3>History</h3>

    {history.length > 0 ? (
      history.map((item, index) => {
   
        
        // Added 'return' here because you are using curly braces { }
        return (
          <div 
            key={index} 
           
            style={{
              ...styles.weatherCardWrapper,
            }} 
            className="weather-card-wrapper" 
            onClick={() => 3} // Replaced '3' with a placeholder function
          >
            <WeatherEntry
              weather={item}
              userPreferencesWithHistory={userPreferencesWithHistory}
              onStarClick={onStarClick}
              setCurrentSelectedLocationOption={setCurrentSelectedLocationOption}
            />

           
          </div>
        );
      })
    ) : (
      <div style={styles.errorMessage}>
        <p>No history locations.</p>
        <span style={{ fontSize: '12px', color: '#888' }}>
          Use the search bar to add locations.
        </span>
      </div>
    )}
  </section>
);
};

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
  weatherCardWrapper: {
    position: "relative",
    cursor: "pointer",
    
    transition: "transform 0.2s ease",
    overflow: "hidden",
    borderRadius: "3px",
    display: "block", // Ensures it wraps the content correctly
  },
  // Base state for the overlay
  cardOverlay: {
    position: "absolute",
    top: 0,
    left: 0,
    width: "100%",
    height: "100%",
    backgroundColor: "rgba(0, 0, 0, 0.15)",
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    opacity: 0, // Hidden by default
    transition: "opacity 0.3s ease",
    backdropFilter: "blur(2px)",
    pointerEvents: "none", // Allows clicks to pass through if needed
  },
  overlayIcon: {
    fontSize: "2rem",
    color: "white",
    marginBottom: "8px",
  },
  overlayText: {
    color: "white",
    fontWeight: 600,
    fontSize: "0.9rem",
    textTransform: "uppercase",
    letterSpacing: "1px",
  },
} as const;