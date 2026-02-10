import type { CurrentWeather } from "./CurrentWeather";

export interface LocationOption {
  location_name: string | null;
  state: string | null;
  country: string | null;
  lat: number | null;
  lon: number | null;
  current_weather: CurrentWeather |null;
}