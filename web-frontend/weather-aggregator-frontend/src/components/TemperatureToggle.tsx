 export const TemperatureToggle = ({ unit, onChange }: { unit: string; onChange: (newUnit: string) => void  }) => (
  <div style={styles.toggleContainer} onClick={() => onChange(unit === 'C' ? 'F' : 'C')}>
    <div style={{...styles.pill, backgroundColor: unit === 'C' ? 'aliceblue' : 'transparent'}}>°C</div>
    <div style={{...styles.pill, backgroundColor: unit === 'F' ? 'aliceblue' : 'transparent'}}>°F</div>
  </div>
);


const styles = {
   toggleContainer: {
    display: 'flex',
    cursor: 'pointer',
    background: 'rgba(255, 255, 255, 0.1)',
    borderRadius: '20px',
    padding: '2px',
    border: '1px solid rgba(255, 255, 255, 0.2)',
    marginLeft: '10px'
  },
  pill: {
    padding: '4px 10px',
    borderRadius: '18px',
    fontSize: '12px',
    fontWeight: 'bold',
    transition: 'all 0.3s ease',
    color: 'black'
  }

} as const;