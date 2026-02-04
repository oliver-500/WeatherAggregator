import type { GetUserPreferencesRequest } from '../model/requests/GetUserPreferencesRequest';
import { userApi } from './client';
import type { UpdateUserPreferencesRequest } from '../model/requests/UpdateUserPreferencesRequest';
import type { AddHistoryItemRequest } from '../model/requests/AddHistoryItemRequest';


export const getUserPreferencesWithHistory = async (req: GetUserPreferencesRequest) => {
    try {
      const response = await userApi.post('/preferences', req);
      console.log(response.data);
      return response.data;
    } catch (error) {
      console.error("Microservice is grumpy today:", error);
    }
};


export const updateUserPreferencesWithHistory = async (req: UpdateUserPreferencesRequest) => {
    try {
      await userApi.put('/preferences', req);
    } catch (error) {
      console.error("Microservice is grumpy today:", error);
    }
};


export const addHistoryItem = async (req: AddHistoryItemRequest) => {
    try {
      await userApi.post('/history', req);
    } catch (error) {
      console.error("Microservice is grumpy today:", error);
    }   
  }