import type { LocationOption } from '../model/LocationOption';
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

export const getWeatherDataByCityName = async (locationName: string) => {
  try {
    const response = await weatherApi.get('/current_weather_by_location', {
      params: { location_name: locationName }
    });
  console.log(response.data);
    // 3. Update your React state with the result

    let locations = [] as LocationOption[];
    locations.push({
      current_weather: response.data,
      location_name: response.data.location.name,
      state: response.data.location.state_region_province_or_entity,
      country: response.data.location.country,
      lat: response.data.location.lat,
      lon: response.data.location.lon,
    });

    return locations;
  } catch (err: any) {

    if(err.status === 409) {
      const errorData = err.response?.data?.error;
    
      if (errorData?.code?.AMBIGUOUS_LOCATION_NAME_ERROR) {
        const locations: LocationOption[] = errorData.code.AMBIGUOUS_LOCATION_NAME_ERROR;
      
        return locations;

      }
      else {
        throw err;
      }
   }
   else if (err.status === 404) {
      return [];
   }

    else {
      throw err;
    }

    
    
  }
}
