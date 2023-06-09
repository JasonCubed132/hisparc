use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NameNumber {
    pub name: String,
    pub number: u32,
}

#[derive(Deserialize, Debug)]
pub struct Scintillator {
    pub alpha: Option<f32>,
    pub beta: Option<f32>,
    pub height: Option<f32>,
    pub radius: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct StationInfo {
    pub active: bool,
    pub altitude: Option<f32>,
    pub cluster: String,
    pub country: String,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub name: String,
    pub number: u32,
    pub scintillators: Vec<Scintillator>,
    pub subcluster: String,
}

#[derive(Deserialize, Debug)]
pub struct StationConfig {
    pub coinctime: f32,
    pub delay_check: f32,
    pub delay_error: f32,
    pub delay_screen: f32,
    pub detnum: f32,
    pub gps_altitude: f32,
    pub gps_latitude: f32,
    pub gps_longitude: f32,
    pub mas_ch1_adc_gain: f32,
    pub mas_ch1_adc_offset: f32,
    pub mas_ch1_comp_gain: f32,
    pub mas_ch1_comp_offset: f32,
    pub mas_ch1_current: f32,
    pub mas_ch1_gain_neg: f32,
    pub mas_ch1_gain_pos: f32,
    pub mas_ch1_inttime: f32,
    pub mas_ch1_offset_neg: f32,
    pub mas_ch1_offset_pos: f32,
    pub mas_ch1_thres_high: f32,
    pub mas_ch1_thres_low: f32,
    pub mas_ch1_voltage: f32,
    pub mas_ch2_adc_gain: f32,
    pub mas_ch2_adc_offset: f32,
    pub mas_ch2_comp_gain: f32,
    pub mas_ch2_comp_offset: f32,
    pub mas_ch2_current: f32,
    pub mas_ch2_gain_neg: f32,
    pub mas_ch2_gain_pos: f32,
    pub mas_ch2_inttime: f32,
    pub mas_ch2_offset_neg: f32,
    pub mas_ch2_offset_pos: f32,
    pub mas_ch2_thres_high: f32,
    pub mas_ch2_thres_low: f32,
    pub mas_ch2_voltage: f32,
    pub mas_common_offset: f32,
    pub mas_comp_thres_high: f32,
    pub mas_comp_thres_low: f32,
    pub mas_internal_voltage: f32,
    pub mas_max_voltage: f32,
    pub mas_reset: bool,
    pub mas_version: String,
    pub postcoinctime: f32,
    pub precoinctime: f32,
    pub reduce_data: bool,
    pub slv_ch1_adc_gain: f32,
    pub slv_ch1_adc_offset: f32,
    pub slv_ch1_comp_gain: f32,
    pub slv_ch1_comp_offset: f32,
    pub slv_ch1_current: f32,
    pub slv_ch1_gain_neg: f32,
    pub slv_ch1_gain_pos: f32,
    pub slv_ch1_inttime: f32,
    pub slv_ch1_offset_neg: f32,
    pub slv_ch1_offset_pos: f32,
    pub slv_ch1_thres_high: f32,
    pub slv_ch1_thres_low: f32,
    pub slv_ch1_voltage: f32,
    pub slv_ch2_adc_gain: f32,
    pub slv_ch2_adc_offset: f32,
    pub slv_ch2_comp_gain: f32,
    pub slv_ch2_comp_offset: f32,
    pub slv_ch2_current: f32,
    pub slv_ch2_gain_neg: f32,
    pub slv_ch2_gain_pos: f32,
    pub slv_ch2_inttime: f32,
    pub slv_ch2_offset_neg: f32,
    pub slv_ch2_offset_pos: f32,
    pub slv_ch2_thres_high: f32,
    pub slv_ch2_thres_low: f32,
    pub slv_ch2_voltage: f32,
    pub slv_common_offset: f32,
    pub slv_comp_thres_high: f32,
    pub slv_comp_thres_low: f32,
    pub slv_internal_voltage: f32,
    pub slv_max_voltage: f32,
    pub slv_reset: bool,
    pub slv_version: String,
    pub spare_bytes: f32,
    pub startmode: bool,
    pub summary: f32,
    pub timestamp: String,
    pub trig_and_or: bool,
    pub trig_external: f32,
    pub trig_high_signals: f32,
    pub trig_low_signals: f32,
    pub use_filter: bool,
    pub use_filter_threshold: bool,
}
