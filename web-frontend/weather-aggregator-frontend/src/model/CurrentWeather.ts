export interface CurrentWeather {
  isFavorite?: boolean;
  provider: string;
  location: Location;
  weather: Weather;
  wind: Wind;
  /** Represented as a number in TS (handles i64) */
  observed_at_timestamp?: number | null;
}

export interface Location {
  name?: string | null;
  country?: string | null;
  lat: number;
  lon: number;
  state_region_province_or_entity?: string | null;
}

export interface Wind {
  speed_metric?: number | null;
  speed_imperial?: number | null;
  gust_metric?: number | null;
  gust_imperial?: number | null;
  direction?: string | null;
  degrees?: number | null;
}

export interface Weather {
  temp_metric: number;
  temp_imperial: number;
  temp_feelslike_metric?: number | null;
  temp_feelslike_imperial?: number | null;
  humidity?: number | null;
  pressure_metric?: number | null;
  pressure_imperial?: number | null;
  condition?: string | null;
}