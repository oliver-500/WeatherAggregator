import axios from 'axios';

const commonConfig = {
  timeout: 5000,
  headers: { 'Content-Type': 'application/json' }
};

export const weatherApi = axios.create({
  ...commonConfig,
  baseURL: import.meta.env.VITE_WEATHER_AGGREGATOR_API_URL // /api/weather
});

export const userApi = axios.create({
  ...commonConfig,
  baseURL: import.meta.env.VITE_USER_IDENTITY_API_URL // /api/users
});

export const settingsApi = axios.create({
  ...commonConfig,
  baseURL: import.meta.env.VITE_USER_PREFERENCES_API_URL // /api/settings
});

