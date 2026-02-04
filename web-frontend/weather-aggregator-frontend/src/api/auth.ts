import type { RegisterUserRequest } from '../model/requests/RegisterUserRequest';
import type { LoginUserRequest } from '../model/requests/LoginUserRequest.ts';
import { authApi } from './client';
import type { GenericServiceError } from '../model/errors/GenericServiceError.ts';
import { ApiError } from '../model/errors/ApiError.ts';

export const registerUser = async (userRegistrationData : RegisterUserRequest) => {
    const response = await authApi.post('/register', userRegistrationData);
  return response.data;
};

export const loginUser = async (loginUserRequest : LoginUserRequest) => {
    try {
      const response = await authApi.post('/login', 
        loginUserRequest
    );


      // 3. Update your React state with the result
      return response.data;
    } catch (error: any) {
      let finalMessage = "An unexpected error occurred.";
      let isServerError = false;

      if (error.response) {
      // It's a 400/500 from the backend
          isServerError = true;
          const backendError = error.response.data as GenericServiceError;
          finalMessage = backendError?.error?.message || "Server Error";
      } else if (error.request) {
      // The server is dead/offline
          finalMessage = "Server is unreachable.";
      }

      const apiError = new ApiError(finalMessage, isServerError, error.response?.status, error);
      // Throw the specific message so the component can use it
      throw apiError;
    }
};

export const getUserInfo = async () => {
  try {
    const response = await authApi.get('/user_info');
    return response.data;
  } catch (error: any) {
    let finalMessage = "An unexpected error occurred.";
    let isServerError = false;

    if (error.response) {
    // It's a 400/500 from the backend
        isServerError = true;
        const backendError = error.response.data as GenericServiceError;
        finalMessage = backendError?.error?.message || "Server Error";
    } else if (error.request) {
    // The server is dead/offline
        finalMessage = "Server is unreachable.";
    }

    const apiError = new ApiError(finalMessage, isServerError, error.response?.status, error);
    // Throw the specific message so the component can use it
    throw apiError;
  }
};


export const refreshAccessToken = async () => {
  try {
    await authApi.get('/refresh');

  } catch (error: any) {
    let finalMessage = "An unexpected error occurred.";
    let isServerError = false;

    if (error.response) {
    // It's a 400/500 from the backend
        isServerError = true;
        const backendError = error.response.data as GenericServiceError;
        finalMessage = backendError?.error?.message || "Server Error";
    } else if (error.request) {
    // The server is dead/offline
        finalMessage = "Server is unreachable.";
    }

    const apiError = new ApiError(finalMessage, isServerError, error.response?.status, error);
    // Throw the specific message so the component can use it
    throw apiError;
  }
};


export const registerAnonymousUser = async () => {
  try {
    await authApi.get('/anonymous');

  } catch (error: any) {
    let finalMessage = "An unexpected error occurred.";
    let isServerError = false;

    if (error.response) {
    // It's a 400/500 from the backend
        isServerError = true;
        const backendError = error.response.data as GenericServiceError;
        finalMessage = backendError?.error?.message || "Server Error";
    } else if (error.request) {
    // The server is dead/offline
        finalMessage = "Server is unreachable.";
    }

    const apiError = new ApiError(finalMessage, isServerError, error.response?.status, error);
    // Throw the specific message so the component can use it
    throw apiError;
  }
};



export const logoutUser = async () => {
  authApi.post('/logout').then(() => {
    window.location.reload();
  }).catch((err: any) => {
    throw err;
  });
}