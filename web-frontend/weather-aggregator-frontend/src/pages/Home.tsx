import { useEffect } from 'react';
import  getWeatherData  from '../api/weather';

export default function Home() {

    useEffect(() => {
        let isMounted = true;

        getWeatherData();

        return () => {
            isMounted = false; // Prevents updating state if component unmounts
         };
    }, []);

    return <div>Home</div>;
}