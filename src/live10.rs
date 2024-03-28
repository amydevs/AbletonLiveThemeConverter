use serde::{Deserialize, Serialize};
use crate::common::{ValueWrapper, RGBAColor};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkinManager {
    pub control_foreground: Option<RGBAColor>,
    pub text_disabled: Option<RGBAColor>,
    pub control_disabled: Option<RGBAColor>,
    pub meter_background: Option<RGBAColor>,
    pub surface_highlight: Option<RGBAColor>,
    pub surface_area: Option<RGBAColor>,
    pub desktop: Option<RGBAColor>,
    pub view_check_control_enabled_on: Option<RGBAColor>,
    pub scrollbar_inner_handle: Option<RGBAColor>,
    pub scrollbar_inner_track: Option<RGBAColor>,
    pub scrollbar_outer_handle: Option<RGBAColor>,
    pub scrollbar_outer_track: Option<RGBAColor>,
    pub scrollbar_lcd_handle: Option<RGBAColor>,
    pub scrollbar_lcd_track: Option<RGBAColor>,
    pub detail_view_background: Option<RGBAColor>,
    pub preferences_tab: Option<RGBAColor>,
    pub selection_frame: Option<RGBAColor>,
    pub control_background: Option<RGBAColor>,
    pub control_fill_handle: Option<RGBAColor>,
    pub chosen_default: Option<RGBAColor>,
    pub chosen_record: Option<RGBAColor>,
    pub chosen_pre_listen: Option<RGBAColor>,
    pub implicit_arm: Option<RGBAColor>,
    pub range_default: Option<RGBAColor>,
    pub range_disabled: Option<RGBAColor>,
    pub range_disabled_off: Option<RGBAColor>,
    pub learn_midi: Option<RGBAColor>,
    pub learn_key: Option<RGBAColor>,
    pub learn_macro: Option<RGBAColor>,
    pub range_edit_field: Option<RGBAColor>,
    pub range_edit_field2: Option<RGBAColor>,
    pub bipol_reset: Option<RGBAColor>,
    pub chosen_alternative: Option<RGBAColor>,
    pub chosen_alert: Option<RGBAColor>,
    pub chosen_play: Option<RGBAColor>,
    pub clip1: Option<RGBAColor>,
    pub clip2: Option<RGBAColor>,
    pub clip3: Option<RGBAColor>,
    pub clip4: Option<RGBAColor>,
    pub clip5: Option<RGBAColor>,
    pub clip6: Option<RGBAColor>,
    pub clip7: Option<RGBAColor>,
    pub clip8: Option<RGBAColor>,
    pub clip9: Option<RGBAColor>,
    pub clip10: Option<RGBAColor>,
    pub clip11: Option<RGBAColor>,
    pub clip12: Option<RGBAColor>,
    pub clip13: Option<RGBAColor>,
    pub clip14: Option<RGBAColor>,
    pub clip15: Option<RGBAColor>,
    pub clip16: Option<RGBAColor>,
    pub clip_text: Option<RGBAColor>,
    pub clip_border: Option<RGBAColor>,
    pub selection_background: Option<RGBAColor>,
    pub standby_selection_background: Option<RGBAColor>,
    pub selection_foreground: Option<RGBAColor>,
    pub standby_selection_foreground: Option<RGBAColor>,
    pub surface_background: Option<RGBAColor>,
    pub automation_color: Option<RGBAColor>,
    pub automation_grid: Option<RGBAColor>,
    pub loop_color: Option<RGBAColor>,
    pub off_grid_loop_color: Option<RGBAColor>,
    pub arrangement_ruler_markings: Option<RGBAColor>,
    pub detail_view_ruler_markings: Option<RGBAColor>,
    pub shadow_dark: Option<RGBAColor>,
    pub shadow_light: Option<RGBAColor>,
    pub display_background: Option<RGBAColor>,
    pub ableton_color: Option<RGBAColor>,
    pub waveform_color: Option<RGBAColor>,
    pub velocity_color: Option<RGBAColor>,
    pub alert: Option<RGBAColor>,
    pub control_on_foreground: Option<RGBAColor>,
    pub control_off_foreground: Option<RGBAColor>,
    pub control_on_disabled_foreground: Option<RGBAColor>,
    pub control_off_disabled_foreground: Option<RGBAColor>,
    pub control_on_alternative_foreground: Option<RGBAColor>,
    pub control_text_back: Option<RGBAColor>,
    pub control_contrast_frame: Option<RGBAColor>,
    pub control_selection_frame: Option<RGBAColor>,
    pub control_contrast_transport: Option<RGBAColor>,
    pub progress: Option<RGBAColor>,
    pub progress_text: Option<RGBAColor>,
    pub transport_progress: Option<RGBAColor>,
    pub clip_slot_button: Option<RGBAColor>,
    pub browser_bar: Option<RGBAColor>,
    pub browser_bar_overlay_hint_text_color: Option<RGBAColor>,
    pub browser_disabled_item: Option<RGBAColor>,
    pub browser_sample_waveform: Option<RGBAColor>,
    pub automation_disabled: Option<RGBAColor>,
    pub automation_mouse_over: Option<RGBAColor>,
    pub midi_note_max_velocity: Option<RGBAColor>,
    pub retro_display_background: Option<RGBAColor>,
    pub retro_display_background_line: Option<RGBAColor>,
    pub retro_display_foreground: Option<RGBAColor>,
    pub retro_display_foreground2: Option<RGBAColor>,
    pub retro_display_foreground_disabled: Option<RGBAColor>,
    pub retro_display_green: Option<RGBAColor>,
    pub retro_display_red: Option<RGBAColor>,
    pub retro_display_handle1: Option<RGBAColor>,
    pub retro_display_handle2: Option<RGBAColor>,
    pub retro_display_scale_text: Option<RGBAColor>,
    pub threshold_line_color: Option<RGBAColor>,
    pub gain_reduction_line_color: Option<RGBAColor>,
    pub input_curve_color: Option<RGBAColor>,
    pub input_curve_outline_color: Option<RGBAColor>,
    pub output_curve_color: Option<RGBAColor>,
    pub output_curve_outline_color: Option<RGBAColor>,
    pub spectrum_default_color: Option<RGBAColor>,
    pub spectrum_alternative_color: Option<RGBAColor>,
    pub spectrum_grid_lines: Option<RGBAColor>,
    pub operator1: Option<RGBAColor>,
    pub operator2: Option<RGBAColor>,
    pub operator3: Option<RGBAColor>,
    pub operator4: Option<RGBAColor>,
    pub drum_rack_scroller1: Option<RGBAColor>,
    pub drum_rack_scroller2: Option<RGBAColor>,
    pub filled_drum_rack_pad: Option<RGBAColor>,
    pub surface_area_focus: Option<RGBAColor>,
    pub freeze_color: Option<RGBAColor>,
    pub grid_label: Option<RGBAColor>,
    pub grid_line_base: Option<RGBAColor>,
    pub arranger_grid_tiles: Option<RGBAColor>,
    pub detail_grid_tiles: Option<RGBAColor>,
    pub grid_guideline: Option<RGBAColor>,
    pub off_grid_guideline: Option<RGBAColor>,
    pub tree_column_head_background: Option<RGBAColor>,
    pub tree_column_head_foreground: Option<RGBAColor>,
    pub tree_column_head_selected: Option<RGBAColor>,
    pub tree_column_head_focus: Option<RGBAColor>,
    pub tree_column_head_control: Option<RGBAColor>,
    pub tree_row_category_foreground: Option<RGBAColor>,
    pub tree_row_category_background: Option<RGBAColor>,
    pub search_indication: Option<RGBAColor>,
    pub search_indication_standby: Option<RGBAColor>,
    pub key_zone_background: Option<RGBAColor>,
    pub key_zone_crossfade_ramp: Option<RGBAColor>,
    pub velocity_zone_background: Option<RGBAColor>,
    pub velocity_zone_crossfade_ramp: Option<RGBAColor>,
    pub selector_zone_background: Option<RGBAColor>,
    pub selector_zone_crossfade_ramp: Option<RGBAColor>,
    pub view_check_control_enabled_off: Option<RGBAColor>,
    pub view_check_control_disabled_on: Option<RGBAColor>,
    pub view_check_control_disabled_off: Option<RGBAColor>,
    pub default_blend_factor: Option<ValueWrapper<f32>>,
    pub icon_blend_factor: Option<ValueWrapper<f32>>,
    pub clip_blend_factor: Option<ValueWrapper<f32>>,
    pub note_border_standby_blend_factor: Option<ValueWrapper<f32>>,
    pub retro_display_blend_factor: Option<ValueWrapper<f32>>,
    pub check_control_not_checked_blend_factor: Option<ValueWrapper<f32>>,
    pub mix_surface_area_blend_factor: Option<ValueWrapper<f32>>,
    pub text_frame_segment_blend_factor: Option<ValueWrapper<f32>>,
    pub velocity_editor_foreground_selected_blend_factor: Option<ValueWrapper<f32>>,
    pub note_disabled_selected_blend_factor: Option<ValueWrapper<f32>>,
    pub background_clip: Option<RGBAColor>,
    pub background_clip_frame: Option<RGBAColor>,
    pub warper_time_bar_ruler_background: Option<RGBAColor>,
    pub warper_time_bar_marker_background: Option<RGBAColor>,
    pub min_velocity_note_blend_factor: Option<ValueWrapper<f32>>,
    pub striped_background_shade_factor: Option<ValueWrapper<f32>>,
    pub automation_lane_header_alpha: Option<ValueWrapper<u8>>,
    pub automation_lane_clip_body_alpha: Option<ValueWrapper<u8>>,
    pub non_editable_automation_alpha: Option<ValueWrapper<u8>>,
    pub disabled_context_menu_icon_alpha: Option<ValueWrapper<u8>>,
    pub bipolar_poti_triangle: Option<RGBAColor>,
    pub poti: Option<RGBAColor>,
    pub deactivated_poti: Option<RGBAColor>,
    pub poti_needle: Option<RGBAColor>,
    pub deactivated_poti_needle: Option<RGBAColor>,
    pub transport_off_background: Option<RGBAColor>,
    pub transport_off_disabled_foreground: Option<RGBAColor>,
    pub transport_selection_background: Option<RGBAColor>,
    pub modulation: Option<RGBAColor>,
    pub modulation_disabled: Option<RGBAColor>,
    pub modulation_mouse_over: Option<RGBAColor>,
    pub automation_transform_tool_frame: Option<RGBAColor>,
    pub automation_transform_tool_frame_active: Option<RGBAColor>,
    pub automation_transform_tool_handle: Option<RGBAColor>,
    pub automation_transform_tool_handle_active: Option<RGBAColor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ableton {
    #[serde(rename = "@MajorVersion")]
    major_version: Option<String>,
    #[serde(rename = "@MinorVersion")]
    minor_version: Option<String>,
    #[serde(rename = "@SchemaChangeCount")]
    schema_change_count: Option<String>,
    #[serde(rename = "@Creator")]
    creator: Option<String>,
    #[serde(rename = "@Revision")]
    revision: Option<String>,
    #[serde(rename = "SkinManager")]
    skin_manager: Option<SkinManager>,
}

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use super::{ValueWrapper, RGBAColor, Ableton};
    #[test]
    fn value_wrapper() {
        let rgba_value: ValueWrapper<u8> = from_str(
            r#"
            <Alpha Value="90"/>
            "#,
        ).unwrap();
        assert_eq!(rgba_value.value, 90);
    }
    #[test]
    fn rgba() {
        let rgba: RGBAColor = from_str(
            r#"
            <ControlForeground>
                <R Value="1"/>
                <G Value="2"/>
                <B Value="3"/>
                <Alpha Value="4"/>
            </ControlForeground>
            "#,
        ).unwrap();
        assert_eq!(rgba.r.value, 1);
        assert_eq!(rgba.g.value, 2);
        assert_eq!(rgba.b.value, 3);
        assert_eq!(rgba.a.value, 4);
    }
    #[test]
    fn ableton() {
        let ableton: Ableton = from_str(include_str!("../test_themes/blank_10.ask")).unwrap();
        assert_eq!(ableton.major_version, Some("5".to_string()));
    }
}