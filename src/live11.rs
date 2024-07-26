use crate::common::{HexColor, Meter, ValueWrapper};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "PascalCase")]
pub struct Theme {
    pub control_foreground: Option<HexColor>,
    pub text_disabled: Option<HexColor>,
    pub control_disabled: Option<HexColor>,
    pub meter_background: Option<HexColor>,
    pub surface_highlight: Option<HexColor>,
    pub surface_area: Option<HexColor>,
    pub surface_area_foreground: Option<HexColor>,
    pub desktop: Option<HexColor>,
    pub view_check_control_enabled_on: Option<HexColor>,
    pub scrollbar_inner_handle: Option<HexColor>,
    pub scrollbar_inner_track: Option<HexColor>,
    pub scrollbar_outer_handle: Option<HexColor>,
    pub scrollbar_outer_track: Option<HexColor>,
    #[serde(rename = "ScrollbarLCDHandle")]
    pub scrollbar_lcd_handle: Option<HexColor>,
    #[serde(rename = "ScrollbarLCDTrack")]
    pub scrollbar_lcd_track: Option<HexColor>,
    pub detail_view_background: Option<HexColor>,
    pub preferences_tab: Option<HexColor>,
    pub selection_frame: Option<HexColor>,
    pub control_background: Option<HexColor>,
    pub control_fill_handle: Option<HexColor>,
    pub chosen_default: Option<HexColor>,
    pub chosen_record: Option<HexColor>,
    pub chosen_pre_listen: Option<HexColor>,
    pub implicit_arm: Option<HexColor>,
    pub range_default: Option<HexColor>,
    pub range_disabled: Option<HexColor>,
    pub range_disabled_off: Option<HexColor>,
    pub learn_midi: Option<HexColor>,
    pub learn_key: Option<HexColor>,
    pub learn_macro: Option<HexColor>,
    pub range_edit_field: Option<HexColor>,
    pub range_edit_field_2: Option<HexColor>,
    pub bipol_reset: Option<HexColor>,
    pub chosen_alternative: Option<HexColor>,
    pub chosen_alert: Option<HexColor>,
    pub chosen_play: Option<HexColor>,
    pub clip_1: Option<HexColor>,
    pub clip_2: Option<HexColor>,
    pub clip_3: Option<HexColor>,
    pub clip_4: Option<HexColor>,
    pub clip_5: Option<HexColor>,
    pub clip_6: Option<HexColor>,
    pub clip_7: Option<HexColor>,
    pub clip_8: Option<HexColor>,
    pub clip_9: Option<HexColor>,
    pub clip_10: Option<HexColor>,
    pub clip_11: Option<HexColor>,
    pub clip_12: Option<HexColor>,
    pub clip_13: Option<HexColor>,
    pub clip_14: Option<HexColor>,
    pub clip_15: Option<HexColor>,
    pub clip_16: Option<HexColor>,
    pub clip_text: Option<HexColor>,
    pub clip_border: Option<HexColor>,
    pub scene_contrast: Option<HexColor>,
    pub selection_background: Option<HexColor>,
    pub standby_selection_background: Option<HexColor>,
    pub selection_foreground: Option<HexColor>,
    pub standby_selection_foreground: Option<HexColor>,
    pub selection_background_contrast: Option<HexColor>,
    pub surface_background: Option<HexColor>,
    pub take_lane_track_highlighted: Option<HexColor>,
    pub take_lane_track_not_highlighted: Option<HexColor>,
    pub automation_color: Option<HexColor>,
    pub automation_grid: Option<HexColor>,
    pub loop_color: Option<HexColor>,
    pub off_grid_loop_color: Option<HexColor>,
    pub arrangement_ruler_markings: Option<HexColor>,
    pub detail_view_ruler_markings: Option<HexColor>,
    pub shadow_dark: Option<HexColor>,
    pub shadow_light: Option<HexColor>,
    pub display_background: Option<HexColor>,
    pub ableton_color: Option<HexColor>,
    pub waveform_color: Option<HexColor>,
    pub dimmed_waveform_color: Option<HexColor>,
    pub velocity_color: Option<HexColor>,
    pub velocity_selected_or_hovered: Option<HexColor>,
    pub note_probability: Option<HexColor>,
    pub alert: Option<HexColor>,
    pub control_on_foreground: Option<HexColor>,
    pub control_off_foreground: Option<HexColor>,
    pub control_on_disabled_foreground: Option<HexColor>,
    pub control_off_disabled_foreground: Option<HexColor>,
    pub control_on_alternative_foreground: Option<HexColor>,
    pub control_text_back: Option<HexColor>,
    pub control_contrast_frame: Option<HexColor>,
    pub control_selection_frame: Option<HexColor>,
    pub control_contrast_transport: Option<HexColor>,
    pub progress: Option<HexColor>,
    pub progress_text: Option<HexColor>,
    pub transport_progress: Option<HexColor>,
    pub clip_slot_button: Option<HexColor>,
    pub browser_bar: Option<HexColor>,
    pub browser_bar_overlay_hint_text_color: Option<HexColor>,
    pub browser_disabled_item: Option<HexColor>,
    pub browser_sample_waveform: Option<HexColor>,
    pub automation_disabled: Option<HexColor>,
    pub automation_mouse_over: Option<HexColor>,
    pub midi_note_max_velocity: Option<HexColor>,
    pub retro_display_background: Option<HexColor>,
    pub retro_display_background_line: Option<HexColor>,
    pub retro_display_foreground: Option<HexColor>,
    pub retro_display_foreground_2: Option<HexColor>,
    pub retro_display_foreground_disabled: Option<HexColor>,
    pub retro_display_green: Option<HexColor>,
    pub retro_display_red: Option<HexColor>,
    pub retro_display_handle_1: Option<HexColor>,
    pub retro_display_handle_2: Option<HexColor>,
    pub retro_display_scale_text: Option<HexColor>,
    pub retro_display_title: Option<HexColor>,
    pub threshold_line_color: Option<HexColor>,
    pub gain_reduction_line_color: Option<HexColor>,
    pub input_curve_color: Option<HexColor>,
    pub input_curve_outline_color: Option<HexColor>,
    pub output_curve_color: Option<HexColor>,
    pub output_curve_outline_color: Option<HexColor>,
    pub spectrum_default_color: Option<HexColor>,
    pub spectrum_alternative_color: Option<HexColor>,
    pub spectrum_grid_lines: Option<HexColor>,
    pub operator_1: Option<HexColor>,
    pub operator_2: Option<HexColor>,
    pub operator_3: Option<HexColor>,
    pub operator_4: Option<HexColor>,
    pub drum_rack_scroller_1: Option<HexColor>,
    pub drum_rack_scroller_2: Option<HexColor>,
    pub filled_drum_rack_pad: Option<HexColor>,
    pub surface_area_focus: Option<HexColor>,
    pub freeze_color: Option<HexColor>,
    pub grid_label: Option<HexColor>,
    pub grid_line_base: Option<HexColor>,
    pub arranger_grid_tiles: Option<HexColor>,
    pub detail_grid_tiles: Option<HexColor>,
    pub grid_guideline: Option<HexColor>,
    pub off_grid_guideline: Option<HexColor>,
    pub tree_column_head_background: Option<HexColor>,
    pub tree_column_head_foreground: Option<HexColor>,
    pub tree_column_head_selected: Option<HexColor>,
    pub tree_column_head_focus: Option<HexColor>,
    pub tree_column_head_control: Option<HexColor>,
    pub tree_row_category_foreground: Option<HexColor>,
    pub tree_row_category_background: Option<HexColor>,
    pub search_indication: Option<HexColor>,
    pub search_indication_standby: Option<HexColor>,
    pub key_zone_background: Option<HexColor>,
    pub key_zone_crossfade_ramp: Option<HexColor>,
    pub velocity_zone_background: Option<HexColor>,
    pub velocity_zone_crossfade_ramp: Option<HexColor>,
    pub selector_zone_background: Option<HexColor>,
    pub selector_zone_crossfade_ramp: Option<HexColor>,
    pub view_check_control_enabled_off: Option<HexColor>,
    pub view_check_control_disabled_on: Option<HexColor>,
    pub view_check_control_disabled_off: Option<HexColor>,
    pub default_blend_factor: Option<ValueWrapper<f32>>,
    pub icon_blend_factor: Option<ValueWrapper<f32>>,
    pub clip_blend_factor: Option<ValueWrapper<f32>>,
    pub note_border_standby_blend_factor: Option<ValueWrapper<f32>>,
    pub retro_display_blend_factor: Option<ValueWrapper<f32>>,
    pub check_control_not_checked_blend_factor: Option<ValueWrapper<f32>>,
    pub mix_surface_area_blend_factor: Option<ValueWrapper<f32>>,
    pub text_frame_segment_blend_factor: Option<ValueWrapper<f32>>,
    pub note_disabled_selected_blend_factor: Option<ValueWrapper<f32>>,
    pub background_clip: Option<HexColor>,
    pub background_clip_frame: Option<HexColor>,
    pub warper_time_bar_ruler_background: Option<HexColor>,
    pub warper_time_bar_marker_background: Option<HexColor>,
    pub min_velocity_note_blend_factor: Option<ValueWrapper<f32>>,
    pub striped_background_shade_factor: Option<ValueWrapper<f32>>,
    pub non_editable_automation_alpha: Option<ValueWrapper<i32>>,
    pub disabled_context_menu_icon_alpha: Option<ValueWrapper<i32>>,
    pub clip_border_alpha: Option<ValueWrapper<i32>>,
    pub scroll_bar_alpha: Option<ValueWrapper<i32>>,
    pub scroll_bar_on_hover_alpha: Option<ValueWrapper<i32>>,
    pub scroll_bar_background_alpha: Option<ValueWrapper<i32>>,
    pub inaudible_take_lightness: Option<ValueWrapper<f64>>,
    pub inaudible_take_saturation: Option<ValueWrapper<f64>>,
    pub inaudible_take_name_lightness: Option<ValueWrapper<f64>>,
    pub inaudible_take_name_saturation: Option<ValueWrapper<f64>>,
    pub automation_lane_clip_body_lightness: Option<ValueWrapper<f64>>,
    pub automation_lane_clip_body_saturation: Option<ValueWrapper<f64>>,
    pub automation_lane_header_lightness: Option<ValueWrapper<f64>>,
    pub automation_lane_header_saturation: Option<ValueWrapper<f64>>,
    pub take_lane_header_lightness: Option<ValueWrapper<f64>>,
    pub take_lane_header_saturation: Option<ValueWrapper<f64>>,
    pub take_lane_header_name_lightness: Option<ValueWrapper<f64>>,
    pub take_lane_header_name_saturation: Option<ValueWrapper<f64>>,
    pub automation_lane_header_name_lightness: Option<ValueWrapper<f64>>,
    pub automation_lane_header_name_saturation: Option<ValueWrapper<f64>>,
    pub clip_contrast_color_adjustment: Option<ValueWrapper<f32>>,
    pub bipolar_poti_triangle: Option<HexColor>,
    pub poti: Option<HexColor>,
    pub deactivated_poti: Option<HexColor>,
    pub poti_needle: Option<HexColor>,
    pub deactivated_poti_needle: Option<HexColor>,
    pub transport_off_background: Option<HexColor>,
    pub transport_off_disabled_foreground: Option<HexColor>,
    pub transport_selection_background: Option<HexColor>,
    pub modulation: Option<HexColor>,
    pub modulation_disabled: Option<HexColor>,
    pub modulation_mouse_over: Option<HexColor>,
    pub automation_transform_tool_frame: Option<HexColor>,
    pub automation_transform_tool_frame_active: Option<HexColor>,
    pub automation_transform_tool_handle: Option<HexColor>,
    pub automation_transform_tool_handle_active: Option<HexColor>,
    pub muted_audition_clip: Option<HexColor>,
    pub linked_track_hover: Option<HexColor>,
    pub expression_lane_header_highlight: Option<HexColor>,
    pub zoom_pan_handle: Option<HexColor>,
    pub standard_vu_meter: Option<Meter>,
    pub overload_vu_meter: Option<Meter>,
    pub disabled_vu_meter: Option<Meter>,
    pub headphones_vu_meter: Option<Meter>,
    pub sends_only_vu_meter: Option<Meter>,
    pub bipolar_gain_reduction_vu_meter: Option<Meter>,
    pub orange_vu_meter: Option<Meter>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Ableton {
    #[serde(rename = "@MajorVersion")]
    pub major_version: Option<String>,
    #[serde(rename = "@MinorVersion")]
    pub minor_version: Option<String>,
    #[serde(rename = "@SchemaChangeCount")]
    pub schema_change_count: Option<String>,
    #[serde(rename = "@Creator")]
    pub creator: Option<String>,
    #[serde(rename = "@Revision")]
    pub revision: Option<String>,
    #[serde(rename = "Theme")]
    pub theme: Option<Theme>,
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Ableton")]
    pub type AbletonJsType;
}
impl Tsify for Ableton {
    type JsType = AbletonJsType;
    const DECL: &'static str = "export interface Ableton {\n    \"@MajorVersion?\": string;\n    \"@MinorVersion?\": string;\n    \"@SchemaChangeCount?\": string;\n    \"@Creator?\": string;\n    \"@Revision?\": string;\n    Theme?: Theme;\n}";
}
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = "export interface Ableton {\n    \"@MajorVersion?\": string;\n    \"@MinorVersion?\": string;\n    \"@SchemaChangeCount?\": string;\n    \"@Creator?\": string;\n    \"@Revision?\": string;\n    Theme?: Theme;\n}";

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use super::Ableton;
    #[test]
    fn ableton() {
        let ableton: Ableton = from_str(include_str!("../test_themes/blank_11.ask")).unwrap();
        assert_eq!(ableton.major_version, Some("5".to_string()));
    }
}
