import CurrentLocation from "./side_panel/CurrentLocation";
import FavoriteLocation from "./side_panel/FavoriteLocation";
import HistoryLocations from "./side_panel/HistoryLocations";

import type { CurrentWeather } from "../model/CurrentWeather";
import type { UserPreferencesWithHistory } from "../model/UserPreferencesWithLocationHistory";
import type { LocationOption } from "../model/LocationOption";

type Props = {
  current: CurrentWeather | null;
  onStarClick: (entry: CurrentWeather) => void;
  userPreferencesWithHistory?: UserPreferencesWithHistory | null;
  favorite: CurrentWeather | null;
  history: CurrentWeather[];
  setCurrentSelectedLocationOption : React.Dispatch<React.SetStateAction<LocationOption | null>>;

};

export default function SidePanel({
  current,
  userPreferencesWithHistory,
  onStarClick: handleStarClick,
  favorite: favorite,
  history: history,
  setCurrentSelectedLocationOption

}: Props) {

  return (
    <aside style={styles.container}>
      <CurrentLocation 
        current={current}
        userPreferencesWithHistory={userPreferencesWithHistory}
        favorite={favorite}
        onStarClick={handleStarClick}
        setCurrentSelectedLocationOption={setCurrentSelectedLocationOption}
      />
      <FavoriteLocation
        userPreferencesWithHistory={userPreferencesWithHistory}
        favorite={favorite}
        onStarClick={handleStarClick}
        setCurrentSelectedLocationOption={setCurrentSelectedLocationOption}
      />
      <HistoryLocations
        userPreferencesWithHistory={userPreferencesWithHistory}
        favorite={favorite}
        history={history}
        onStarClick={handleStarClick}
        setCurrentSelectedLocationOption={setCurrentSelectedLocationOption}

      />
    </aside>
  );
}

const styles = {
  container: {
    width: "320px",
    display: "flex",
    flexDirection: "column",
    borderLeft: "1px solid #ddd",
    overflowY: "auto",
  },
} as const;