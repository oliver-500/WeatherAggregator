import { weatherApi } from './client';


const getWeatherData = async () => {
    console.log("Target URL:", import.meta.env.VITE_WEATHER_AGGREGATOR_API_URL);
    try {
      const response = await weatherApi.get('/current_weather_by_coordinates', {
    params: { lat: 45, lon: 1 }
  });
    console.log(response.data);
      // 3. Update your React state with the result
      return response.data;
    } catch (error) {
      console.error("Microservice is grumpy today:", error);
    }
  };

export default getWeatherData;