import axios from 'axios';
import { logoutUser, refreshAccessToken } from './auth';

const commonConfig = {
  timeout: 5000,
  headers: { 'Content-Type': 'application/json' },
  withCredentials: true, // <--- THIS MUST BE TRUE
};

export const weatherApi = axios.create({
  ...commonConfig,
  baseURL: import.meta.env.VITE_WEATHER_AGGREGATOR_API_URL // /api/weather
});

export const authApi = axios.create({
  ...commonConfig,
  baseURL: import.meta.env.VITE_USER_IDENTITY_API_URL // /api/users
});

export const userApi = axios.create({
  ...commonConfig,
  baseURL: import.meta.env.VITE_USER_PREFERENCES_API_URL // /api/settings
});


userApi.interceptors.response.use(
  (response) => {
    console.log("Interceptor");
    return  response
  }, // Pass through successful requests
  async (error) => {
    console.log("Interceptor error");
    const originalRequest = error.config;

    // SCENARIO 1: Token Expired (401 + specific logic)
    if ((error.response?.status === 403 || error.response?.status === 400)  && !originalRequest._retry) {
      originalRequest._retry = true; // Prevents infinite loops

      try {
        // Attempt to refresh the token
        console.log("Attempting to refresh access token...");
        await refreshAccessToken(); 
        return userApi(originalRequest); // Retry the original request
      } catch (refreshError) {
        // Refresh token also failed/expired -> Force Login
        console.log("lol")
        await logoutUser();
      }
    }

    // SCENARIO 2: Token Missing (or 401 where refresh failed)
    if (error.response?.status === 401) {
       // Prompt user for login or redirect
       alert("Session expired. Please log in.");
       await logoutUser();
    }

    return Promise.reject(error);
  }
);
