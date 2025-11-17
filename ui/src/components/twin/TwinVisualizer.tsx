import React, { useEffect, useRef, useState } from 'react';

interface DigitalTwin {
  id: string;
  name: string;
  twin_type: string;
  configuration: any;
  created_at: string;
  updated_at: string;
}

interface SensorData {
  id: string;
  twin_id: string;
  sensor_id: string;
  sensor_name: string;
  value: number | string | boolean;
  unit?: string;
  timestamp: string;
}

interface TwinVisualizerProps {
  twin: DigitalTwin;
  sensorData: SensorData[];
}

const TwinVisualizer: React.FC<TwinVisualizerProps> = ({
  twin,
  sensorData
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [visualizationType, setVisualizationType] = useState<'2d' | '3d'>('2d');
  const [isFullscreen, setIsFullscreen] = useState<boolean>(false);

  useEffect(() => {
    if (!canvasRef.current) return;
    
    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    
    if (!ctx) return;
    
    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Draw based on twin type
    switch (twin.twin_type) {
      case 'industrial':
        drawIndustrialTwin(ctx, canvas, twin, sensorData);
        break;
      case 'infrastructure':
        drawInfrastructureTwin(ctx, canvas, twin, sensorData);
        break;
      case 'process':
        drawProcessTwin(ctx, canvas, twin, sensorData);
        break;
      default:
        drawDefaultTwin(ctx, canvas, twin, sensorData);
    }
  }, [twin, sensorData, visualizationType]);

  const drawIndustrialTwin = (
    ctx: CanvasRenderingContext2D, 
    canvas: HTMLCanvasElement, 
    twin: DigitalTwin, 
    sensorData: SensorData[]
  ) => {
    // Example visualization for industrial twin
    // This is a placeholder - in a real application, you would have more sophisticated visualizations
    
    const width = canvas.width;
    const height = canvas.height;
    
    // Draw background
    ctx.fillStyle = '#f0f0f0';
    ctx.fillRect(0, 0, width, height);
    
    // Draw machine outline
    ctx.strokeStyle = '#333';
    ctx.lineWidth = 2;
    ctx.strokeRect(width * 0.1, height * 0.2, width * 0.8, height * 0.6);
    
    // Draw title
    ctx.fillStyle = '#000';
    ctx.font = 'bold 16px Arial';
    ctx.textAlign = 'center';
    ctx.fillText(twin.name, width / 2, height * 0.1);
    
    // Draw sensors
    const sensorRadius = 15;
    const sensorPositions = [
      { x: width * 0.2, y: height * 0.3 },
      { x: width * 0.5, y: height * 0.3 },
      { x: width * 0.8, y: height * 0.3 },
      { x: width * 0.2, y: height * 0.7 },
      { x: width * 0.5, y: height * 0.7 },
      { x: width * 0.8, y: height * 0.7 }
    ];
    
    sensorData.slice(0, 6).forEach((sensor, index) => {
      if (index >= sensorPositions.length) return;
      
      const pos = sensorPositions[index];
      
      // Determine color based on value
      let color = '#4CAF50'; // Default green
      
      if (typeof sensor.value === 'number') {
        if (sensor.value > 90) color = '#F44336'; // Red for high values
        else if (sensor.value > 70) color = '#FFC107'; // Yellow for medium values
      } else if (typeof sensor.value === 'boolean') {
        color = sensor.value ? '#4CAF50' : '#F44336'; // Green for true, red for false
      }
      
      // Draw sensor circle
      ctx.beginPath();
      ctx.arc(pos.x, pos.y, sensorRadius, 0, Math.PI * 2);
      ctx.fillStyle = color;
      ctx.fill();
      ctx.stroke();
      
      // Draw sensor name
      ctx.fillStyle = '#000';
      ctx.font = '12px Arial';
      ctx.textAlign = 'center';
      ctx.fillText(sensor.sensor_name, pos.x, pos.y - sensorRadius - 5);
      
      // Draw sensor value
      const valueText = typeof sensor.value === 'boolean' 
        ? (sensor.value ? 'ON' : 'OFF')
        : `${sensor.value}${sensor.unit ? ' ' + sensor.unit : ''}`;
      
      ctx.fillText(valueText, pos.x, pos.y + 5);
    });
  };

  const drawInfrastructureTwin = (
    ctx: CanvasRenderingContext2D, 
    canvas: HTMLCanvasElement, 
    twin: DigitalTwin, 
    sensorData: SensorData[]
  ) => {
    // Placeholder for infrastructure twin visualization
    const width = canvas.width;
    const height = canvas.height;
    
    // Draw background
    ctx.fillStyle = '#e6f7ff';
    ctx.fillRect(0, 0, width, height);
    
    // Draw building outline
    ctx.strokeStyle = '#0066cc';
    ctx.lineWidth = 2;
    ctx.strokeRect(width * 0.2, height * 0.2, width * 0.6, height * 0.6);
    
    // Draw roof
    ctx.beginPath();
    ctx.moveTo(width * 0.2, height * 0.2);
    ctx.lineTo(width * 0.5, height * 0.1);
    ctx.lineTo(width * 0.8, height * 0.2);
    ctx.closePath();
    ctx.stroke();
    
    // Draw title
    ctx.fillStyle = '#000';
    ctx.font = 'bold 16px Arial';
    ctx.textAlign = 'center';
    ctx.fillText(twin.name, width / 2, height * 0.9);
    
    // Draw some sensor indicators
    // Implementation similar to industrial twin but with different layout
  };

  const drawProcessTwin = (
    ctx: CanvasRenderingContext2D, 
    canvas: HTMLCanvasElement, 
    twin: DigitalTwin, 
    sensorData: SensorData[]
  ) => {
    // Placeholder for process twin visualization
    const width = canvas.width;
    const height = canvas.height;
    
    // Draw background
    ctx.fillStyle = '#f9f9f9';
    ctx.fillRect(0, 0, width, height);
    
    // Draw process flow
    ctx.strokeStyle = '#666';
    ctx.lineWidth = 3;
    
    // Draw flow line
    ctx.beginPath();
    ctx.moveTo(width * 0.1, height * 0.5);
    ctx.lineTo(width * 0.9, height * 0.5);
    ctx.stroke();
    
    // Draw process nodes
    const nodePositions = [
      { x: width * 0.2, y: height * 0.5 },
      { x: width * 0.4, y: height * 0.5 },
      { x: width * 0.6, y: height * 0.5 },
      { x: width * 0.8, y: height * 0.5 }
    ];
    
    nodePositions.forEach((pos, index) => {
      ctx.beginPath();
      ctx.arc(pos.x, pos.y, 15, 0, Math.PI * 2);
      ctx.fillStyle = index < sensorData.length ? '#4CAF50' : '#ccc';
      ctx.fill();
      ctx.stroke();
      
      if (index < sensorData.length) {
        // Draw sensor name and value
        ctx.fillStyle = '#000';
        ctx.font = '12px Arial';
        ctx.textAlign = 'center';
        ctx.fillText(sensorData[index].sensor_name, pos.x, pos.y - 25);
        ctx.fillText(String(sensorData[index].value), pos.x, pos.y + 30);
      }
    });
    
    // Draw title
    ctx.fillStyle = '#000';
    ctx.font = 'bold 16px Arial';
    ctx.textAlign = 'center';
    ctx.fillText(twin.name, width / 2, height * 0.1);
  };

  const drawDefaultTwin = (
    ctx: CanvasRenderingContext2D, 
    canvas: HTMLCanvasElement, 
    twin: DigitalTwin, 
    sensorData: SensorData[]
  ) => {
    // Default visualization for unknown twin types
    const width = canvas.width;
    const height = canvas.height;
    
    // Draw background
    ctx.fillStyle = '#f5f5f5';
    ctx.fillRect(0, 0, width, height);
    
    // Draw title
    ctx.fillStyle = '#000';
    ctx.font = 'bold 16px Arial';
    ctx.textAlign = 'center';
    ctx.fillText(`${twin.name} (${twin.twin_type})`, width / 2, height * 0.1);
    
    // Draw message
    ctx.fillText('Custom visualization not available', width / 2, height * 0.5);
  };

  const toggleFullscreen = () => {
    setIsFullscreen(!isFullscreen);
  };

  return (
    <div className={`twin-visualizer ${isFullscreen ? 'fullscreen' : ''}`}>
      <div className="visualizer-header">
        <h3>Twin Visualization</h3>
        <div className="visualizer-controls">
          <select
            value={visualizationType}
            onChange={(e) => setVisualizationType(e.target.value as '2d' | '3d')}
            className="visualization-type-selector"
          >
            <option value="2d">2D View</option>
            <option value="3d">3D View</option>
          </select>
          <button 
            onClick={toggleFullscreen}
            className="fullscreen-button"
            title={isFullscreen ? 'Exit Fullscreen' : 'Enter Fullscreen'}
          >
            {isFullscreen ? '⤓' : '⤢'}
          </button>
        </div>
      </div>
      
      <div className="visualizer-canvas-container">
        <canvas 
          ref={canvasRef} 
          width={800} 
          height={500}
          className="twin-canvas"
        />
        {visualizationType === '3d' && (
          <div className="visualization-not-available">
            <p>3D visualization is not available in this version.</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default TwinVisualizer;