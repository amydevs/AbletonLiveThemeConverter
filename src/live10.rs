use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueWrapper<T> {
    #[serde(rename = "@Value")]
    pub value: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct RGBA {
    pub r: ValueWrapper<u8>,
    pub g: ValueWrapper<u8>,
    pub b: ValueWrapper<u8>,
    pub alpha: ValueWrapper<u8>,
}

impl Default for RGBA {
    fn default() -> Self {
        RGBA {
            r: ValueWrapper { value: 0 },
            g: ValueWrapper { value: 0 },
            b: ValueWrapper { value: 0 },
            alpha: ValueWrapper { value: 255 },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkinManager {
    pub control_foreground: Option<RGBA>,
    pub text_disabled: Option<RGBA>,
    pub control_disabled: Option<RGBA>,
    pub meter_background: Option<RGBA>,
    pub surface_highlight: Option<RGBA>,
    pub surface_area: Option<RGBA>,
    pub desktop: Option<RGBA>,
    pub view_check_control_enabled_on: Option<RGBA>,
    pub scrollbar_inner_handle: Option<RGBA>,
    pub scrollbar_inner_track: Option<RGBA>,
    pub scrollbar_outer_handle: Option<RGBA>,
    pub scrollbar_outer_track: Option<RGBA>,
    pub scrollbar_lcd_handle: Option<RGBA>,
    pub scrollbar_lcd_track: Option<RGBA>,
    pub detail_view_background: Option<RGBA>,
    pub preferences_tab: Option<RGBA>,
    pub selection_frame: Option<RGBA>,
    pub control_background: Option<RGBA>,
    pub control_fill_handle: Option<RGBA>,
    pub chosen_default: Option<RGBA>,
    pub chosen_record: Option<RGBA>,
    pub chosen_pre_listen: Option<RGBA>,
    pub implicit_arm: Option<RGBA>,
    pub range_default: Option<RGBA>,
    pub range_disabled: Option<RGBA>,
    pub range_disabled_off: Option<RGBA>,
    pub learn_midi: Option<RGBA>,
    pub learn_key: Option<RGBA>,
    pub learn_macro: Option<RGBA>,
    pub range_edit_field: Option<RGBA>,
    pub range_edit_field2: Option<RGBA>,
    pub bipol_reset: Option<RGBA>,
    pub chosen_alternative: Option<RGBA>,
    pub chosen_alert: Option<RGBA>,
    pub chosen_play: Option<RGBA>,
    pub clip1: Option<RGBA>,
    pub clip2: Option<RGBA>,
    pub clip3: Option<RGBA>,
    pub clip4: Option<RGBA>,
    pub clip5: Option<RGBA>,
    pub clip6: Option<RGBA>,
    pub clip7: Option<RGBA>,
    pub clip8: Option<RGBA>,
    pub clip9: Option<RGBA>,
    pub clip10: Option<RGBA>,
    pub clip11: Option<RGBA>,
    pub clip12: Option<RGBA>,
    pub clip13: Option<RGBA>,
    pub clip14: Option<RGBA>,
    pub clip15: Option<RGBA>,
    pub clip16: Option<RGBA>,
    pub clip_text: Option<RGBA>,
    pub clip_border: Option<RGBA>,
    pub selection_background: Option<RGBA>,
    pub standby_selection_background: Option<RGBA>,
    pub selection_foreground: Option<RGBA>,
    pub standby_selection_foreground: Option<RGBA>,
    pub surface_background: Option<RGBA>,
    pub automation_color: Option<RGBA>,
    pub automation_grid: Option<RGBA>,
    pub loop_color: Option<RGBA>,
    pub off_grid_loop_color: Option<RGBA>,
    pub arrangement_ruler_markings: Option<RGBA>,
    pub detail_view_ruler_markings: Option<RGBA>,
    pub shadow_dark: Option<RGBA>,
    pub shadow_light: Option<RGBA>,
    pub display_background: Option<RGBA>,
    pub ableton_color: Option<RGBA>,
    pub waveform_color: Option<RGBA>,
    pub velocity_color: Option<RGBA>,
    pub alert: Option<RGBA>,
    pub control_on_foreground: Option<RGBA>,
    pub control_off_foreground: Option<RGBA>,
    pub control_on_disabled_foreground: Option<RGBA>,
    pub control_off_disabled_foreground: Option<RGBA>,
    pub control_on_alternative_foreground: Option<RGBA>,
    pub control_text_back: Option<RGBA>,
    pub control_contrast_frame: Option<RGBA>,
    pub control_selection_frame: Option<RGBA>,
    pub control_contrast_transport: Option<RGBA>,
    pub progress: Option<RGBA>,
    pub progress_text: Option<RGBA>,
    pub transport_progress: Option<RGBA>,
    pub clip_slot_button: Option<RGBA>,
    pub browser_bar: Option<RGBA>,
    pub browser_bar_overlay_hint_text_color: Option<RGBA>,
    pub browser_disabled_item: Option<RGBA>,
    pub browser_sample_waveform: Option<RGBA>,
    pub automation_disabled: Option<RGBA>,
    pub automation_mouse_over: Option<RGBA>,
    pub midi_note_max_velocity: Option<RGBA>,
    pub retro_display_background: Option<RGBA>,
    pub retro_display_background_line: Option<RGBA>,
    pub retro_display_foreground: Option<RGBA>,
    pub retro_display_foreground2: Option<RGBA>,
    pub retro_display_foreground_disabled: Option<RGBA>,
    pub retro_display_green: Option<RGBA>,
    pub retro_display_red: Option<RGBA>,
    pub retro_display_handle1: Option<RGBA>,
    pub retro_display_handle2: Option<RGBA>,
    pub retro_display_scale_text: Option<RGBA>,
    pub threshold_line_color: Option<RGBA>,
    pub gain_reduction_line_color: Option<RGBA>,
    pub input_curve_color: Option<RGBA>,
    pub input_curve_outline_color: Option<RGBA>,
    pub output_curve_color: Option<RGBA>,
    pub output_curve_outline_color: Option<RGBA>,
    pub spectrum_default_color: Option<RGBA>,
    pub spectrum_alternative_color: Option<RGBA>,
    pub spectrum_grid_lines: Option<RGBA>,
    pub operator1: Option<RGBA>,
    pub operator2: Option<RGBA>,
    pub operator3: Option<RGBA>,
    pub operator4: Option<RGBA>,
    pub drum_rack_scroller1: Option<RGBA>,
    pub drum_rack_scroller2: Option<RGBA>,
    pub filled_drum_rack_pad: Option<RGBA>,
    pub surface_area_focus: Option<RGBA>,
    pub freeze_color: Option<RGBA>,
    pub grid_label: Option<RGBA>,
    pub grid_line_base: Option<RGBA>,
    pub arranger_grid_tiles: Option<RGBA>,
    pub detail_grid_tiles: Option<RGBA>,
    pub grid_guideline: Option<RGBA>,
    pub off_grid_guideline: Option<RGBA>,
    pub tree_column_head_background: Option<RGBA>,
    pub tree_column_head_foreground: Option<RGBA>,
    pub tree_column_head_selected: Option<RGBA>,
    pub tree_column_head_focus: Option<RGBA>,
    pub tree_column_head_control: Option<RGBA>,
    pub tree_row_category_foreground: Option<RGBA>,
    pub tree_row_category_background: Option<RGBA>,
    pub search_indication: Option<RGBA>,
    pub search_indication_standby: Option<RGBA>,
    pub key_zone_background: Option<RGBA>,
    pub key_zone_crossfade_ramp: Option<RGBA>,
    pub velocity_zone_background: Option<RGBA>,
    pub velocity_zone_crossfade_ramp: Option<RGBA>,
    pub selector_zone_background: Option<RGBA>,
    pub selector_zone_crossfade_ramp: Option<RGBA>,
    pub view_check_control_enabled_off: Option<RGBA>,
    pub view_check_control_disabled_on: Option<RGBA>,
    pub view_check_control_disabled_off: Option<RGBA>,
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
    pub background_clip: Option<RGBA>,
    pub background_clip_frame: Option<RGBA>,
    pub warper_time_bar_ruler_background: Option<RGBA>,
    pub warper_time_bar_marker_background: Option<RGBA>,
    pub min_velocity_note_blend_factor: Option<ValueWrapper<f32>>,
    pub striped_background_shade_factor: Option<ValueWrapper<f32>>,
    pub automation_lane_header_alpha: Option<ValueWrapper<u8>>,
    pub automation_lane_clip_body_alpha: Option<ValueWrapper<u8>>,
    pub non_editable_automation_alpha: Option<ValueWrapper<u8>>,
    pub disabled_context_menu_icon_alpha: Option<ValueWrapper<u8>>,
    pub bipolar_poti_triangle: Option<RGBA>,
    pub poti: Option<RGBA>,
    pub deactivated_poti: Option<RGBA>,
    pub poti_needle: Option<RGBA>,
    pub deactivated_poti_needle: Option<RGBA>,
    pub transport_off_background: Option<RGBA>,
    pub transport_off_disabled_foreground: Option<RGBA>,
    pub transport_selection_background: Option<RGBA>,
    pub modulation: Option<RGBA>,
    pub modulation_disabled: Option<RGBA>,
    pub modulation_mouse_over: Option<RGBA>,
    pub automation_transform_tool_frame: Option<RGBA>,
    pub automation_transform_tool_frame_active: Option<RGBA>,
    pub automation_transform_tool_handle: Option<RGBA>,
    pub automation_transform_tool_handle_active: Option<RGBA>,
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

    use super::{ValueWrapper, RGBA, Ableton};
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
        let rgba: RGBA = from_str(
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
        assert_eq!(rgba.alpha.value, 4);
    }
    #[test]
    fn ableton() {
        let ableton: Ableton = from_str(include_str!("../test_themes/blank_10.ask")).unwrap();
        assert_eq!(ableton.major_version, Some("5".to_string()));
    }
}