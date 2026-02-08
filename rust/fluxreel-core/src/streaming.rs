use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamingProtocol {
    RTMP,
    RTSP,
    SRT,
    WebRTC,
    HLS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamingQuality {
    Low,      // 480p, 1Mbps
    Medium,   // 720p, 2.5Mbps
    High,     // 1080p, 5Mbps
    Ultra,    // 1080p60, 8Mbps
    Custom { bitrate: u32, resolution: (u32, u32), fps: u32 },
}

#[pyclass]
#[derive(Clone)]
pub struct StreamingConfig {
    pub protocol: StreamingProtocol,
    pub quality: StreamingQuality,
    #[pyo3(get, set)]
    pub url: String,
    #[pyo3(get, set)]
    pub stream_key: String,
    #[pyo3(get, set)]
    pub reconnect: bool,
    #[pyo3(get, set)]
    pub reconnect_delay: u32,  // seconds
}

#[pymethods]
impl StreamingConfig {
    #[new]
    fn new(url: String, stream_key: String) -> Self {
        Self {
            protocol: StreamingProtocol::RTMP,
            quality: StreamingQuality::High,
            url,
            stream_key,
            reconnect: true,
            reconnect_delay: 5,
        }
    }

    fn set_protocol(&mut self, protocol: String) {
        self.protocol = match protocol.as_str() {
            "rtmp" => StreamingProtocol::RTMP,
            "rtsp" => StreamingProtocol::RTSP,
            "srt" => StreamingProtocol::SRT,
            "webrtc" => StreamingProtocol::WebRTC,
            "hls" => StreamingProtocol::HLS,
            _ => StreamingProtocol::RTMP,
        };
    }

    fn set_quality(&mut self, quality: String) {
        self.quality = match quality.as_str() {
            "low" => StreamingQuality::Low,
            "medium" => StreamingQuality::Medium,
            "high" => StreamingQuality::High,
            "ultra" => StreamingQuality::Ultra,
            _ => StreamingQuality::High,
        };
    }

    fn set_custom_quality(&mut self, bitrate: u32, width: u32, height: u32, fps: u32) {
        self.quality = StreamingQuality::Custom {
            bitrate,
            resolution: (width, height),
            fps,
        };
    }
}

#[pyclass]
pub struct Streamer {
    pub config: StreamingConfig,
    pub is_streaming: bool,
    pub frame_count: u64,
}

#[pymethods]
impl Streamer {
    #[new]
    fn new(config: StreamingConfig) -> Self {
        Self {
            config,
            is_streaming: false,
            frame_count: 0,
        }
    }

    fn start(&mut self) -> PyResult<()> {
        println!("Starting stream to: {}", self.config.url);
        println!("Stream key: {}", self.config.stream_key);
        self.is_streaming = true;
        self.frame_count = 0;
        // Actual streaming implementation would connect here
        Ok(())
    }

    fn stop(&mut self) -> PyResult<()> {
        println!("Stopping stream");
        self.is_streaming = false;
        // Actual streaming implementation would disconnect here
        Ok(())
    }

    fn send_frame(&mut self, _frame_data: Vec<u8>) -> PyResult<()> {
        if !self.is_streaming {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Stream is not active"
            ));
        }
        self.frame_count += 1;
        // Actual frame sending would happen here
        Ok(())
    }

    fn get_stats(&self) -> PyResult<(u64, bool, u32, u64)> {
        Ok((
            self.frame_count,
            self.is_streaming,
            0,  // bitrate - would be calculated from actual stream
            0,  // dropped_frames
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct RTMPStreamer {
    pub config: StreamingConfig,
    #[pyo3(get, set)]
    pub server_url: String,
    #[pyo3(get, set)]
    pub app_name: String,
}

#[pymethods]
impl RTMPStreamer {
    #[new]
    fn new(server_url: String, stream_key: String) -> Self {
        let mut config = StreamingConfig::new(server_url.clone(), stream_key);
        config.set_protocol("rtmp".to_string());
        
        Self {
            config,
            server_url,
            app_name: "live".to_string(),
        }
    }

    fn connect(&mut self) -> PyResult<()> {
        let full_url = format!("rtmp://{}/{}", self.server_url, self.app_name);
        println!("Connecting to RTMP server: {}", full_url);
        // RTMP connection would be established here
        Ok(())
    }

    fn disconnect(&mut self) -> PyResult<()> {
        println!("Disconnecting from RTMP server");
        Ok(())
    }
}

/// Create RTMP stream URL
#[pyfunction]
pub fn create_rtmp_url(server: &str, app: &str, stream_key: &str) -> String {
    format!("rtmp://{}/{}/{}", server, app, stream_key)
}

/// Validate RTMP URL format
#[pyfunction]
pub fn validate_rtmp_url(url: &str) -> bool {
    url.starts_with("rtmp://") || url.starts_with("rtmps://")
}

