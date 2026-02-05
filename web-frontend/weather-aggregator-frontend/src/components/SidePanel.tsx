import CurrentLocation from "./side_panel/CurrentLocation";
import FavoriteLocation from "./side_panel/FavoriteLocation";
import HistoryLocations from "./side_panel/HistoryLocations";

import type { CurrentWeather } from "../model/CurrentWeather";
import type { UserPreferencesWithHistory } from "../model/UserPreferencesWithLocationHistory";

type Props = {
  current: CurrentWeather | null;
  onStarClick: (entry: CurrentWeather) => void;
  userPreferencesWithHistory?: UserPreferencesWithHistory | null;
  favorite: CurrentWeather | null;
  history: CurrentWeather[];

};

export default function SidePanel({
  current,
  userPreferencesWithHistory,
  onStarClick: handleStarClick,
  favorite: favorite,
  history: history,

}: Props) {

  return (
    <aside style={styles.container}>
      <CurrentLocation 
        current={current}
        userPreferencesWithHistory={userPreferencesWithHistory}
        favorite={favorite}
        onStarClick={handleStarClick}
      />
      <FavoriteLocation
        userPreferencesWithHistory={userPreferencesWithHistory}
        favorite={favorite}
        onStarClick={handleStarClick}
      />
      <HistoryLocations
        userPreferencesWithHistory={userPreferencesWithHistory}
        favorite={favorite}
        history={history}
        onStarClick={handleStarClick}

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