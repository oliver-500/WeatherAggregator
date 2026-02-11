import React, { useEffect, useState } from 'react';
import { registerUser, loginUser, logoutUser as logoutUserAuth } from '../api/auth';
import isEmail from 'validator/lib/isEmail';
import type { UserInfo } from '../model/UserInfo';
import type { RegisterUserRequest } from '../model/requests/RegisterUserRequest';
import type { LoginUserRequest } from '../model/requests/LoginUserRequest.ts';
import toast from 'react-hot-toast';
import { TemperatureToggle } from './TemperatureToggle.tsx';
import type { UserPreferencesWithHistory } from '../model/UserPreferencesWithLocationHistory.ts';
import type { RegistrationResponse } from '../model/responses/RegistrationResponse.ts';
import { TextField, Autocomplete } from '@mui/material';
import { getWeatherDataByCityName, getWeatherDataByCoordinates } from '../api/weather.ts';
import type { LocationOption } from '../model/LocationOption.ts';
import type { CurrentWeather } from '../model/CurrentWeather.ts';

type TopBarProps = {
  user_info?: UserInfo | null;
  onUnitChange: (newSystem: "METRIC" | "IMPERIAL") => void;
  userPreferencesWithHistory?: UserPreferencesWithHistory | null;
  setUserInfo: (userInfo: UserInfo | null) => void;
  setUserPreferencesWithHistory(userPreferencesWithHistory: UserPreferencesWithHistory | null): void;
  initializeUserRelatedInfo(isReinitialization: boolean): Promise<void>;
  setCurrentSelectedLocationOption: (locationOption: LocationOption) => void;
  setLocationHistory: React.Dispatch<React.SetStateAction<CurrentWeather[]>>;
};

export default function TopBar({
    user_info, 
    onUnitChange,
    userPreferencesWithHistory,
    setUserInfo,
    setUserPreferencesWithHistory,
    initializeUserRelatedInfo,
    setCurrentSelectedLocationOption,
    setLocationHistory
  }: TopBarProps) {

  const [isModalOpen, setIsModalOpen] = useState(false);
  const [authMode, setAuthMode] = useState<'login' | 'register'>('login');

  const [inputValue, setInputValue] = useState(''); // What the user types
  const [options, setOptions] = useState<LocationOption[]>([]);     // Results from your API
  const [open, setOpen] = useState(false);
  const [isSearching, setIsSearching] = useState(false);

  const handleSearch = async () => {
    if (!inputValue) return;
    
    if(isSearching) return;
    setIsSearching(true);

    try {
      let res = await getWeatherDataByCityName(inputValue);
      if (res.length == 0) {
        toast.error("No locations found. Try a different spelling.")
      }
      else if (res.length == 1) {
        setCurrentSelectedLocationOption(res[0]);
        let result = res[0];
        let newInputValue = result.location_name + ", " + result.state + ", " + result.country;
        setInputValue(newInputValue)
      }
      else {
        setOptions(res);
        setOpen(true); // 2. Only show the suggestions AFTER the search is triggered
      } 
    } catch (error: any) {
    }
    
    setIsSearching(false);
  };
 
  const initialState = {
    email: 'pera@gmail.com',
    password: 'Pera2gmail.com',
    confirmPassword: 'Pera2gmail.com',
    usePrevouslySavedData: true
  };
  const [formData, setFormData] = useState(initialState);
  const [isSending, setIsSending] = useState(false);

  const isButtonEnabled = authMode === 'login' ? 
    (formData.email.length > 0 && formData.password.length > 0 && !isSending) :
    (formData.email.length > 0 && formData.password.length > 0 && formData.confirmPassword.length > 0 && !isSending);

  useEffect(() => {
    if (!isModalOpen) {
        setFormData(initialState);
    }
  }, [isModalOpen]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value
    });
  };

  const closeModal = () => {
    setIsModalOpen(false);
    setAuthMode('login');
    setFormData(initialState); 
  };

  const getWeatherDataForSelectedLocationOption = async (locationOption: LocationOption) => {
   
    console.log(locationOption.lat, locationOption.lon);
    if (locationOption.lat && locationOption.lon) {
      let result = await getWeatherDataByCoordinates(locationOption.lat, locationOption.lon);
      console.log("op");

      setCurrentSelectedLocationOption({
        current_weather: result,
        location_name: result.location.name,
        state: result.state_region_province_or_entity,
        country: result.location.country,
        lat: result.location.lat,
        lon: result.location.lon,
      });
      setOptions([]);
    }
   
  }

  const handleSubmit = async () => {
    if (isSending) return; // Prevent multiple submissions
    setIsSending(true);
    
    if (formData.password.length < 8) {
        toast.error("Password must be at least 8 characters."); 
        setIsSending(false);
        return;
    }

    if (!isEmail(formData.email)) {
        toast.error("Please enter a valid email address.");
        setIsSending(false);
        return;
    }

    if (authMode === 'register') {
      if (formData.password !== formData.confirmPassword) {
        toast.error("Passwords do not match.");
        setIsSending(false);
        return;
      }

      const requestData: RegisterUserRequest = {
        email: formData.email, 
        password: formData.password, 
        use_previously_saved_data: formData.usePrevouslySavedData 
      };

      try {
        let res: RegistrationResponse = await registerUser(requestData);
        setUserInfo( {
          email: res.user_email,
          user_id: res.user_id,
          user_type: res.user_type
        });

        setIsModalOpen(false);
        toast.success("Registration successful.");
        await initializeUserRelatedInfo(true);
        setAuthMode('login'); // Switch to login after successful registration
      } catch (err: any) {
        const statusCode = err.response?.status;

        if (statusCode === 409) {
          toast.error(err.response.data?.error?.message);
        } else if (statusCode === 400) {
          toast.error(err.response.data?.error?.message);
        } else {
          toast.error("Registration failed. Please try again.");
        }
      }
      setIsSending(false);
  
    } else {
      const requestData: LoginUserRequest = { 
        email: formData.email, 
        password: formData.password
      };
      try {
        await loginUser(requestData);
        setIsModalOpen(false);
        setIsSending(false);
        toast.success("Login successfull.");
        await initializeUserRelatedInfo(true);
        return;
      } catch (err: any) {
        
        const statusCode = err.status;

        if (statusCode === 400 || statusCode === 409 || statusCode === 401) {
          toast.error(err.message);
        } else {
         
          toast.error("Login failed. Please try again.");
        }
        setIsSending(false);
      }
    }
  };

  const logoutUser = async () => {
    try {
      await logoutUserAuth();
      setUserInfo(null);
      setUserPreferencesWithHistory(null);
      setLocationHistory([]);
      toast.success("Logout successfull.");
      await initializeUserRelatedInfo(true)
    } catch (err) {
      toast.error("Logout failed. Please try again.");
    }
  }

  return (
    <>
    <header style={styles.container}>
     
  
      <Autocomplete
        onBlur={() => {
          setInputValue(inputValue); 
        }}

        getOptionLabel={(option) => 
          `${option.location_name}, ${option.country}, ${option.state}`
        }
       
        open={open}
        onOpen={() => { if (options.length > 0) setOpen(true); }}
        onClose={() => setOpen(false)}
        options={options}
        onChange={(_event, newValue) => {
          console.log("User selected:", newValue);
          if (!newValue) return;
          getWeatherDataForSelectedLocationOption(newValue);
        }}
        style={styles.search}
        inputValue={inputValue}
        onInputChange={(_event, newInputValue, reason) => {
          if (reason === 'input') {
            setInputValue(newInputValue);
            if (!newInputValue) return;
          } else if (reason === 'reset') {
            // If the reason is 'reset' and we have a value, don't clear it
            if (newInputValue === '') return; 
            setInputValue(newInputValue);
          }
        }}
        renderInput={(params) => (
          <TextField {...params} 
          label="Location Name" 
          variant="outlined" 
          size="small" 
          
          onKeyDown={(e) => {
            if (e.key === 'Enter') {
              handleSearch();
            }
          }}

          />
        )}
      />
      
      <button onClick={handleSearch} style={{
              ...styles.button,
              ...( isSearching ? styles.buttonDisabled : {} )
              }}  disabled={isSearching}>
        Search
      </button>
 

      <div style={styles.right}>
        {user_info?.email ? (   
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
            <span>{user_info?.email}</span>
            <TemperatureToggle   
              unit={userPreferencesWithHistory?.preferences.unit_system === "METRIC" ? 'C' : 'F'}
              onChange={(newUnit: string) => {
                console.log(newUnit + " selected in TopBar");
                const system = newUnit === 'C' ? "METRIC" : "IMPERIAL";
                onUnitChange(system); // Just call the prop!
              }}
            >
            </TemperatureToggle>

            <button
              style={styles.button}
              onClick={() => logoutUser()}
            >
              Log out
            </button>
          </div>
        ) : (
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>     
           <TemperatureToggle   
            unit={userPreferencesWithHistory?.preferences.unit_system === "METRIC" ? 'C' : 'F'}
              onChange={(newUnit: string) => {
                console.log(newUnit + " selected in TopBar");
                const system = newUnit === 'C' ? "METRIC" : "IMPERIAL";
                onUnitChange(system); // Just call the prop!
              }}
           >            
           </TemperatureToggle>
            <button
              style={styles.button}
              onClick={() => setIsModalOpen(true)}
            >
              Sign In
            </button>
          </div>     
        )}
      </div>
    </header>

    {isModalOpen && (
        <div style={styles.overlay} >
          <div style={styles.modal} onClick={(e) => e.stopPropagation()}>
            <button style={styles.closeX} onClick={closeModal}>
                &times; 
            </button>
            <h3 style={styles.modalTitle}>
                {authMode === 'login' ? 'Sign In' : 'Create Account'}
            </h3>
            <input 
                type="email" 
                placeholder="Email" 
                style={styles.modalInput}
                value={formData.email}
                onChange={handleChange}
                maxLength={50}
                name="email"
                />
            <input 
                type="password" 
                placeholder="Password" 
                style={styles.modalInput} 
                value={formData.password} 
                onChange={handleChange}
                maxLength={20}
                name="password" />

            {/* Show extra field only for register */}
            {authMode === 'register' && (
                <input 
                    type="password" 
                    placeholder="Confirm Password" 
                    style={styles.modalInput} 
                    value={formData.confirmPassword}
                    maxLength={20}
                    onChange={handleChange} 
                    name="confirmPassword" />
            )}

            {authMode === 'register' && (
                <div style={styles.checkboxContainer}>
                    <input 
                    type="checkbox" 
                    id="specialAccess"
                    checked={formData.usePrevouslySavedData}
                    onChange={(e) => setFormData({...formData, usePrevouslySavedData: e.target.checked})}
                    />
                    <label htmlFor="specialAccess" style={styles.checkboxLabel}>
                      Use Previously Saved Data
                    </label>
                </div>
            )}

            {/* Dynamic Button */}
            <button style={{
              ...styles.button,
              ...( !isButtonEnabled ? styles.buttonDisabled : {} )
              }} onClick={handleSubmit} disabled={!isButtonEnabled}>
                {authMode === 'login' ? 'Login' : 'Sign Up'}
            </button>



           
            <br></br>
            {/* Dynamic Footer Link */}
            <p style={styles.footerText}>
                {authMode === 'login' ? (
                <>
                    Not registered? 
                    <span style={styles.link} onClick={() => setAuthMode('register')}> Register here</span>
                </>
                ) : (
                <>
                    Already have an account? 
                    <span style={styles.link} onClick={() => setAuthMode('login')}> Sign In here</span>
                </>
                )}
            </p>
          </div>
        </div>
      )}
    </>
  );
}

const styles = {
  container: {
    width: "100%",
    display: "flex",
    alignItems: "center",
    padding: "12px 16px",
    borderBottom: "1px solid #ddd",
    gap: "16px",
    boxSizing: "border-box" as "border-box",
    overflow: "hidden"
  },
  search: {
    flex: 1,
    padding: "8px 12px",
    fontSize: "16px",
  },
  right: {
    whiteSpace: "nowrap",
  },
  button: {
    backgroundColor: "aliceblue", // A vibrant "Action" color
    color: "black",
    padding: "10px 20px",
    borderRadius: "8px",
    border: "none",
    fontWeight: "600",
    fontSize: "14px",
    cursor: "pointer",
    transition: "all 0.2s ease", // Smooth transition for hover effects
    boxShadow: "0 2px 4px rgba(0, 0, 0, 0.1)",
    },
    overlay: {
    position: "fixed" as "fixed",
    top: 0,
    left: 0,
    width: "100vw",
    height: "100vh",
    backgroundColor: "rgba(0, 0, 0, 0.5)", // Darken the background
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    zIndex: 1000,
  },
  buttonDisabled: {
    backgroundColor: "#d1d5db", // A muted grey
    color: "#9ca3af",           // Faded text color
    cursor: "not-allowed",      // Shows the "circle-slash" icon
    boxShadow: "none",          // Remove depth to look "flat"
  },
  modal: {
    position: "relative" as "relative",
    backgroundColor: "white",
    padding: "40px 32px 32px 32px",
    borderRadius: "12px",
    display: "flex",
    flexDirection: "column" as "column",
    gap: "12px",
    width: "300px",
    boxShadow: "0 10px 25px rgba(0,0,0,0.2)",
  },
  modalInput: {
    padding: "10px",
    borderRadius: "6px",
    border: "1px solid #ddd",
  },
  loginSubmit: {
    backgroundColor: "aliceblue",
    color: "black",
    padding: "10px",
    border: "none",
    borderRadius: "6px",
    cursor: "pointer",
    fontWeight: "600",
    boxShadow: "0 2px 4px rgba(0, 0, 0, 0.1)",
  },
  closeBtn: {
    background: "none",
    border: "none",
    color: "#666",
    cursor: "pointer",
    fontSize: "12px",
    
  },
  modalTitle: {
    margin: "0 0 8px 0",       // Remove default top margin, add a little bottom space
    fontSize: "18px",          // Slightly larger than the button text
    fontWeight: "600",         // Match the button's thickness
    fontFamily: "inherit",     // Force it to use your app's font
    color: "#1a1a1a",          // A clean, near-black color
    textAlign: "center" as "center", // Optional: centers the title
  },
  closeX: {
    position: "absolute" as "absolute",
    top: "12px",
    right: "12px",
    background: "none",
    border: "none",
    fontSize: "20px",
    color: "#999",
    cursor: "pointer",
    lineHeight: "1",
    padding: "4px",
  },
  footerText: {
    marginTop: "8px",
    fontSize: "14px",
    textAlign: "center" as "center",
    color: "#666", // Subtle grey for the main text
    fontFamily: "inherit",
  },
  link: {
    color: "#007AFF", // Brand blue for the link
    fontWeight: "600",
    cursor: "pointer",
    marginLeft: "4px",
    textDecoration: "none",
  },
    checkboxContainer: {
    display: "flex",
    alignItems: "center",
    gap: "8px",
    marginTop: "8px",
    padding: "4px",
    },
    checkboxLabel: {
    fontSize: "14px",
    color: "#444",
    cursor: "pointer",
    fontFamily: "inherit",
    },
 
};