import { weatherApi } from './client';


export const getWeatherDataByCoordinates = async (lat: number, lon: number) => {
    try {
      const response = await weatherApi.get('/current_weather_by_coordinates', {
        params: { lat, lon }
      });
    console.log(response.data);
      // 3. Update your React state with the result
      return response.data;
    } catch (error) {
      console.error("Microservice is grumpy today:", error);
    }
  };

export const getWeatherDataByIpAddress = async () => {
  try {
    const response = await weatherApi.get('/current_weather_by_ip_address');
  console.log(response.data);
    // 3. Update your React state with the result
    return response.data;
  } catch (error) {
    console.error("Microservice is grumpy today:", error);
  }
  
}
