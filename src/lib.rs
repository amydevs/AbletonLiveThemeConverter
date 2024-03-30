use common::{HexColor, ValueWrapper};

pub mod common;
pub mod live10;
pub mod live11;
pub mod live12;
pub mod util;

impl Into<live10::Ableton> for live11::Ableton {
    fn into(self) -> live10::Ableton {
        live10::Ableton {
            major_version: Some("5".to_owned()),
            minor_version: Some("10.0_377".to_owned()),
            schema_change_count: Some("1".to_owned()),
            creator: self.creator,
            revision: self.revision,
            skin_manager: self.theme.and_then(|v| Some(v.into())),
        }
    }
}

impl Into<live10::SkinManager> for live11::Theme {
    fn into(self) -> live10::SkinManager {
        live10::SkinManager {
            control_foreground: self.control_foreground.and_then(|v| Some(v.into())),
            text_disabled: self.text_disabled.and_then(|v| Some(v.into())),
            control_disabled: self.control_disabled.and_then(|v| Some(v.into())),
            meter_background: self.meter_background.and_then(|v| Some(v.into())),
            surface_highlight: self.surface_highlight.and_then(|v| Some(v.into())),
            surface_area: self.surface_area.and_then(|v| Some(v.into())),
            desktop: self.desktop.and_then(|v| Some(v.into())),
            view_check_control_enabled_on: self
                .view_check_control_enabled_on
                .and_then(|v| Some(v.into())),
            scrollbar_inner_handle: self.scrollbar_inner_handle.and_then(|v| Some(v.into())),
            scrollbar_inner_track: self.scrollbar_inner_track.and_then(|v| Some(v.into())),
            scrollbar_outer_handle: self.scrollbar_outer_handle.and_then(|v| Some(v.into())),
            scrollbar_outer_track: self.scrollbar_outer_track.and_then(|v| Some(v.into())),
            scrollbar_lcd_handle: self.scrollbar_lcd_handle.and_then(|v| Some(v.into())),
            scrollbar_lcd_track: self.scrollbar_lcd_track.and_then(|v| Some(v.into())),
            detail_view_background: self.detail_view_background.and_then(|v| Some(v.into())),
            preferences_tab: self.preferences_tab.and_then(|v| Some(v.into())),
            selection_frame: self.selection_frame.and_then(|v| Some(v.into())),
            control_background: self.control_background.and_then(|v| Some(v.into())),
            control_fill_handle: self.control_fill_handle.and_then(|v| Some(v.into())),
            chosen_default: self.chosen_default.and_then(|v| Some(v.into())),
            chosen_record: self.chosen_record.and_then(|v| Some(v.into())),
            chosen_pre_listen: self.chosen_pre_listen.and_then(|v| Some(v.into())),
            implicit_arm: self.implicit_arm.and_then(|v| Some(v.into())),
            range_default: self.range_default.and_then(|v| Some(v.into())),
            range_disabled: self.range_disabled.and_then(|v| Some(v.into())),
            range_disabled_off: self.range_disabled_off.and_then(|v| Some(v.into())),
            learn_midi: self.learn_midi.and_then(|v| Some(v.into())),
            learn_key: self.learn_key.and_then(|v| Some(v.into())),
            learn_macro: self.learn_macro.and_then(|v| Some(v.into())),
            range_edit_field: self.range_edit_field.and_then(|v| Some(v.into())),
            range_edit_field_2: self.range_edit_field_2.and_then(|v| Some(v.into())),
            bipol_reset: self.bipol_reset.and_then(|v| Some(v.into())),
            chosen_alternative: self.chosen_alternative.and_then(|v| Some(v.into())),
            chosen_alert: self.chosen_alert.and_then(|v| Some(v.into())),
            chosen_play: self.chosen_play.and_then(|v| Some(v.into())),
            clip_1: self.clip_1.and_then(|v| Some(v.into())),
            clip_2: self.clip_2.and_then(|v| Some(v.into())),
            clip_3: self.clip_3.and_then(|v| Some(v.into())),
            clip_4: self.clip_4.and_then(|v| Some(v.into())),
            clip_5: self.clip_5.and_then(|v| Some(v.into())),
            clip_6: self.clip_6.and_then(|v| Some(v.into())),
            clip_7: self.clip_7.and_then(|v| Some(v.into())),
            clip_8: self.clip_8.and_then(|v| Some(v.into())),
            clip_9: self.clip_9.and_then(|v| Some(v.into())),
            clip_10: self.clip_10.and_then(|v| Some(v.into())),
            clip_11: self.clip_11.and_then(|v| Some(v.into())),
            clip_12: self.clip_12.and_then(|v| Some(v.into())),
            clip_13: self.clip_13.and_then(|v| Some(v.into())),
            clip_14: self.clip_14.and_then(|v| Some(v.into())),
            clip_15: self.clip_15.and_then(|v| Some(v.into())),
            clip_16: self.clip_16.and_then(|v| Some(v.into())),
            clip_text: self.clip_text.and_then(|v| Some(v.into())),
            clip_border: self.clip_border.and_then(|v| Some(v.into())),
            selection_background: self.selection_background.and_then(|v| Some(v.into())),
            standby_selection_background: self
                .standby_selection_background
                .and_then(|v| Some(v.into())),
            selection_foreground: self.selection_foreground.and_then(|v| Some(v.into())),
            standby_selection_foreground: self
                .standby_selection_foreground
                .and_then(|v| Some(v.into())),
            surface_background: self.surface_background.and_then(|v| Some(v.into())),
            automation_color: self.automation_color.and_then(|v| Some(v.into())),
            automation_grid: self.automation_grid.and_then(|v| Some(v.into())),
            loop_color: self.loop_color.and_then(|v| Some(v.into())),
            off_grid_loop_color: self.off_grid_loop_color.and_then(|v| Some(v.into())),
            arrangement_ruler_markings: self
                .arrangement_ruler_markings
                .and_then(|v| Some(v.into())),
            detail_view_ruler_markings: self
                .detail_view_ruler_markings
                .and_then(|v| Some(v.into())),
            shadow_dark: self.shadow_dark.and_then(|v| Some(v.into())),
            shadow_light: self.shadow_light.and_then(|v| Some(v.into())),
            display_background: self.display_background.and_then(|v| Some(v.into())),
            ableton_color: self.ableton_color.and_then(|v| Some(v.into())),
            waveform_color: self.waveform_color.and_then(|v| Some(v.into())),
            velocity_color: self.velocity_color.and_then(|v| Some(v.into())),
            alert: self.alert.and_then(|v| Some(v.into())),
            control_on_foreground: self.control_on_foreground.and_then(|v| Some(v.into())),
            control_off_foreground: self.control_off_foreground.and_then(|v| Some(v.into())),
            control_on_disabled_foreground: self
                .control_on_disabled_foreground
                .and_then(|v| Some(v.into())),
            control_off_disabled_foreground: self
                .control_off_disabled_foreground
                .and_then(|v| Some(v.into())),
            control_on_alternative_foreground: self
                .control_on_alternative_foreground
                .and_then(|v| Some(v.into())),
            control_text_back: self.control_text_back.and_then(|v| Some(v.into())),
            control_contrast_frame: self.control_contrast_frame.and_then(|v| Some(v.into())),
            control_selection_frame: self.control_selection_frame.and_then(|v| Some(v.into())),
            control_contrast_transport: self
                .control_contrast_transport
                .and_then(|v| Some(v.into())),
            progress: self.progress.and_then(|v| Some(v.into())),
            progress_text: self.progress_text.and_then(|v| Some(v.into())),
            transport_progress: self.transport_progress.and_then(|v| Some(v.into())),
            clip_slot_button: self.clip_slot_button.and_then(|v| Some(v.into())),
            browser_bar: self.browser_bar.and_then(|v| Some(v.into())),
            browser_bar_overlay_hint_text_color: self
                .browser_bar_overlay_hint_text_color
                .and_then(|v| Some(v.into())),
            browser_disabled_item: self.browser_disabled_item.and_then(|v| Some(v.into())),
            browser_sample_waveform: self.browser_sample_waveform.and_then(|v| Some(v.into())),
            automation_disabled: self.automation_disabled.and_then(|v| Some(v.into())),
            automation_mouse_over: self.automation_mouse_over.and_then(|v| Some(v.into())),
            midi_note_max_velocity: self.midi_note_max_velocity.and_then(|v| Some(v.into())),
            retro_display_background: self.retro_display_background.and_then(|v| Some(v.into())),
            retro_display_background_line: self
                .retro_display_background_line
                .and_then(|v| Some(v.into())),
            retro_display_foreground: self.retro_display_foreground.and_then(|v| Some(v.into())),
            retro_display_foreground_2: self.retro_display_foreground_2.and_then(|v| Some(v.into())),
            retro_display_foreground_disabled: self
                .retro_display_foreground_disabled
                .and_then(|v| Some(v.into())),
            retro_display_green: self.retro_display_green.and_then(|v| Some(v.into())),
            retro_display_red: self.retro_display_red.and_then(|v| Some(v.into())),
            retro_display_handle_1: self.retro_display_handle_1.and_then(|v| Some(v.into())),
            retro_display_handle_2: self.retro_display_handle_2.and_then(|v| Some(v.into())),
            retro_display_scale_text: self.retro_display_scale_text.and_then(|v| Some(v.into())),
            threshold_line_color: self.threshold_line_color.and_then(|v| Some(v.into())),
            gain_reduction_line_color: self.gain_reduction_line_color.and_then(|v| Some(v.into())),
            input_curve_color: self.input_curve_color.and_then(|v| Some(v.into())),
            input_curve_outline_color: self.input_curve_outline_color.and_then(|v| Some(v.into())),
            output_curve_color: self.output_curve_color.and_then(|v| Some(v.into())),
            output_curve_outline_color: self
                .output_curve_outline_color
                .and_then(|v| Some(v.into())),
            spectrum_default_color: self.spectrum_default_color.and_then(|v| Some(v.into())),
            spectrum_alternative_color: self
                .spectrum_alternative_color
                .and_then(|v| Some(v.into())),
            spectrum_grid_lines: self.spectrum_grid_lines.and_then(|v| Some(v.into())),
            operator_1: self.operator_1.and_then(|v| Some(v.into())),
            operator_2: self.operator_2.and_then(|v| Some(v.into())),
            operator_3: self.operator_3.and_then(|v| Some(v.into())),
            operator_4: self.operator_4.and_then(|v| Some(v.into())),
            drum_rack_scroller_1: self.drum_rack_scroller_1.and_then(|v| Some(v.into())),
            drum_rack_scroller_2: self.drum_rack_scroller_2.and_then(|v| Some(v.into())),
            filled_drum_rack_pad: self.filled_drum_rack_pad.and_then(|v| Some(v.into())),
            surface_area_focus: self.surface_area_focus.and_then(|v| Some(v.into())),
            freeze_color: self.freeze_color.and_then(|v| Some(v.into())),
            grid_label: self.grid_label.and_then(|v| Some(v.into())),
            grid_line_base: self.grid_line_base.and_then(|v| Some(v.into())),
            arranger_grid_tiles: self.arranger_grid_tiles.and_then(|v| Some(v.into())),
            detail_grid_tiles: self.detail_grid_tiles.and_then(|v| Some(v.into())),
            grid_guideline: self.grid_guideline.and_then(|v| Some(v.into())),
            off_grid_guideline: self.off_grid_guideline.and_then(|v| Some(v.into())),
            tree_column_head_background: self
                .tree_column_head_background
                .and_then(|v| Some(v.into())),
            tree_column_head_foreground: self
                .tree_column_head_foreground
                .and_then(|v| Some(v.into())),
            tree_column_head_selected: self.tree_column_head_selected.and_then(|v| Some(v.into())),
            tree_column_head_focus: self.tree_column_head_focus.and_then(|v| Some(v.into())),
            tree_column_head_control: self.tree_column_head_control.and_then(|v| Some(v.into())),
            tree_row_category_foreground: self
                .tree_row_category_foreground
                .and_then(|v| Some(v.into())),
            tree_row_category_background: self
                .tree_row_category_background
                .and_then(|v| Some(v.into())),
            search_indication: self.search_indication.and_then(|v| Some(v.into())),
            search_indication_standby: self.search_indication_standby.and_then(|v| Some(v.into())),
            key_zone_background: self.key_zone_background.and_then(|v| Some(v.into())),
            key_zone_crossfade_ramp: self.key_zone_crossfade_ramp.and_then(|v| Some(v.into())),
            velocity_zone_background: self.velocity_zone_background.and_then(|v| Some(v.into())),
            velocity_zone_crossfade_ramp: self
                .velocity_zone_crossfade_ramp
                .and_then(|v| Some(v.into())),
            selector_zone_background: self.selector_zone_background.and_then(|v| Some(v.into())),
            selector_zone_crossfade_ramp: self
                .selector_zone_crossfade_ramp
                .and_then(|v| Some(v.into())),
            view_check_control_enabled_off: self
                .view_check_control_enabled_off
                .and_then(|v| Some(v.into())),
            view_check_control_disabled_on: self
                .view_check_control_disabled_on
                .and_then(|v| Some(v.into())),
            view_check_control_disabled_off: self
                .view_check_control_disabled_off
                .and_then(|v| Some(v.into())),
            default_blend_factor: self.default_blend_factor.and_then(|v| Some(v.into())),
            icon_blend_factor: self.icon_blend_factor.and_then(|v| Some(v.into())),
            clip_blend_factor: self.clip_blend_factor.and_then(|v| Some(v.into())),
            note_border_standby_blend_factor: self
                .note_border_standby_blend_factor
                .and_then(|v| Some(v.into())),
            retro_display_blend_factor: self
                .retro_display_blend_factor
                .and_then(|v| Some(v.into())),
            check_control_not_checked_blend_factor: self
                .check_control_not_checked_blend_factor
                .and_then(|v| Some(v.into())),
            mix_surface_area_blend_factor: self
                .mix_surface_area_blend_factor
                .and_then(|v| Some(v.into())),
            text_frame_segment_blend_factor: self
                .text_frame_segment_blend_factor
                .and_then(|v| Some(v.into())),
            velocity_editor_foreground_selected_blend_factor: Some(ValueWrapper {
                value: 0.6000000238,
            }),
            note_disabled_selected_blend_factor: self
                .note_disabled_selected_blend_factor
                .and_then(|v| Some(v.into())),
            background_clip: self.background_clip.and_then(|v| Some(v.into())),
            background_clip_frame: self.background_clip_frame.and_then(|v| Some(v.into())),
            warper_time_bar_ruler_background: self
                .warper_time_bar_ruler_background
                .and_then(|v| Some(v.into())),
            warper_time_bar_marker_background: self
                .warper_time_bar_marker_background
                .and_then(|v| Some(v.into())),
            min_velocity_note_blend_factor: self
                .min_velocity_note_blend_factor
                .and_then(|v| Some(v.into())),
            striped_background_shade_factor: self
                .striped_background_shade_factor
                .and_then(|v| Some(v.into())),
            automation_lane_header_alpha: Some(ValueWrapper { value: 60 }),
            automation_lane_clip_body_alpha: Some(ValueWrapper { value: 60 }),
            non_editable_automation_alpha: self
                .non_editable_automation_alpha
                .and_then(|v| Some(v.into())),
            disabled_context_menu_icon_alpha: self
                .disabled_context_menu_icon_alpha
                .and_then(|v| Some(v.into())),
            bipolar_poti_triangle: self.bipolar_poti_triangle.and_then(|v| Some(v.into())),
            poti: self.poti.and_then(|v| Some(v.into())),
            deactivated_poti: self.deactivated_poti.and_then(|v| Some(v.into())),
            poti_needle: self.poti_needle.and_then(|v| Some(v.into())),
            deactivated_poti_needle: self.deactivated_poti_needle.and_then(|v| Some(v.into())),
            transport_off_background: self.transport_off_background.and_then(|v| Some(v.into())),
            transport_off_disabled_foreground: self
                .transport_off_disabled_foreground
                .and_then(|v| Some(v.into())),
            transport_selection_background: self
                .transport_selection_background
                .and_then(|v| Some(v.into())),
            modulation: self.modulation.and_then(|v| Some(v.into())),
            modulation_disabled: self.modulation_disabled.and_then(|v| Some(v.into())),
            modulation_mouse_over: self.modulation_mouse_over.and_then(|v| Some(v.into())),
            automation_transform_tool_frame: self
                .automation_transform_tool_frame
                .and_then(|v| Some(v.into())),
            automation_transform_tool_frame_active: self
                .automation_transform_tool_frame_active
                .and_then(|v| Some(v.into())),
            automation_transform_tool_handle: self
                .automation_transform_tool_handle
                .and_then(|v| Some(v.into())),
            automation_transform_tool_handle_active: self
                .automation_transform_tool_handle_active
                .and_then(|v| Some(v.into())),
        }
    }
}

impl Into<live11::Ableton> for live10::Ableton {
    fn into(self) -> live11::Ableton {
        live11::Ableton {
            major_version: Some("5".to_owned()),
            minor_version: Some("11.0_436".to_owned()),
            schema_change_count: Some("3".to_owned()),
            creator: self.creator,
            revision: self.revision,
            theme: self.skin_manager.and_then(|v| Some(v.into())),
        }
    }
}

impl Into<live11::Theme> for live10::SkinManager {
    fn into(self) -> live11::Theme {
        live11::Theme {
            control_foreground: self.control_foreground.and_then(|v| Some(v.into())),
            text_disabled: self.text_disabled.and_then(|v| Some(v.into())),
            control_disabled: self.control_disabled.and_then(|v| Some(v.into())),
            meter_background: self.meter_background.and_then(|v| Some(v.into())),
            surface_highlight: self.surface_highlight.and_then(|v| Some(v.into())),
            surface_area: self.surface_area.and_then(|v| Some(v.into())),
            surface_area_foreground: self.surface_area.and_then(|v| Some(v.into())),
            desktop: self.desktop.and_then(|v| Some(v.into())),
            view_check_control_enabled_on: self
                .view_check_control_enabled_on
                .and_then(|v| Some(v.into())),
            scrollbar_inner_handle: self.scrollbar_inner_handle.and_then(|v| Some(v.into())),
            scrollbar_inner_track: self.scrollbar_inner_track.and_then(|v| Some(v.into())),
            scrollbar_outer_handle: self.scrollbar_outer_handle.and_then(|v| Some(v.into())),
            scrollbar_outer_track: self.scrollbar_outer_track.and_then(|v| Some(v.into())),
            scrollbar_lcd_handle: self.scrollbar_lcd_handle.and_then(|v| Some(v.into())),
            scrollbar_lcd_track: self.scrollbar_lcd_track.and_then(|v| Some(v.into())),
            detail_view_background: self.detail_view_background.and_then(|v| Some(v.into())),
            preferences_tab: self.preferences_tab.and_then(|v| Some(v.into())),
            selection_frame: self.selection_frame.and_then(|v| Some(v.into())),
            control_background: self.control_background.and_then(|v| Some(v.into())),
            control_fill_handle: self.control_fill_handle.and_then(|v| Some(v.into())),
            chosen_default: self.chosen_default.and_then(|v| Some(v.into())),
            chosen_record: self.chosen_record.and_then(|v| Some(v.into())),
            chosen_pre_listen: self.chosen_pre_listen.and_then(|v| Some(v.into())),
            implicit_arm: self.implicit_arm.and_then(|v| Some(v.into())),
            range_default: self.range_default.and_then(|v| Some(v.into())),
            range_disabled: self.range_disabled.and_then(|v| Some(v.into())),
            range_disabled_off: self.range_disabled_off.and_then(|v| Some(v.into())),
            learn_midi: self.learn_midi.and_then(|v| Some(v.into())),
            learn_key: self.learn_key.and_then(|v| Some(v.into())),
            learn_macro: self.learn_macro.and_then(|v| Some(v.into())),
            range_edit_field: self.range_edit_field.and_then(|v| Some(v.into())),
            range_edit_field_2: self.range_edit_field_2.and_then(|v| Some(v.into())),
            bipol_reset: self.bipol_reset.and_then(|v| Some(v.into())),
            chosen_alternative: self.chosen_alternative.and_then(|v| Some(v.into())),
            chosen_alert: self.chosen_alert.and_then(|v| Some(v.into())),
            chosen_play: self.chosen_play.and_then(|v| Some(v.into())),
            clip_1: self.clip_1.and_then(|v| Some(v.into())),
            clip_2: self.clip_2.and_then(|v| Some(v.into())),
            clip_3: self.clip_3.and_then(|v| Some(v.into())),
            clip_4: self.clip_4.and_then(|v| Some(v.into())),
            clip_5: self.clip_5.and_then(|v| Some(v.into())),
            clip_6: self.clip_6.and_then(|v| Some(v.into())),
            clip_7: self.clip_7.and_then(|v| Some(v.into())),
            clip_8: self.clip_8.and_then(|v| Some(v.into())),
            clip_9: self.clip_9.and_then(|v| Some(v.into())),
            clip_10: self.clip_10.and_then(|v| Some(v.into())),
            clip_11: self.clip_11.and_then(|v| Some(v.into())),
            clip_12: self.clip_12.and_then(|v| Some(v.into())),
            clip_13: self.clip_13.and_then(|v| Some(v.into())),
            clip_14: self.clip_14.and_then(|v| Some(v.into())),
            clip_15: self.clip_15.and_then(|v| Some(v.into())),
            clip_16: self.clip_16.and_then(|v| Some(v.into())),
            clip_text: self.clip_text.and_then(|v| Some(v.into())),
            clip_border: self.clip_border.and_then(|v| Some(v.into())),
            // We're going to keep the contrast the same as the surface.
            scene_contrast: self.surface_background.and_then(|v| Some(v.into())),
            selection_background: self.selection_background.and_then(|v| Some(v.into())),
            standby_selection_background: self
                .standby_selection_background
                .and_then(|v| Some(v.into())),
            selection_foreground: self.selection_foreground.and_then(|v| Some(v.into())),
            standby_selection_foreground: self
                .standby_selection_foreground
                .and_then(|v| Some(v.into())),
            // We're going to keep the contrast the same as the surface.
            selection_background_contrast: self.selection_background.and_then(|v| Some(v.into())),
            surface_background: self.surface_background.and_then(|v| Some(v.into())),
            // We're going to keep the take lanes the same as the surface.
            take_lane_track_highlighted: self.surface_highlight.and_then(|v| Some(v.into())),
            take_lane_track_not_highlighted: self.surface_background.and_then(|v| Some(v.into())),
            automation_color: self.automation_color.and_then(|v| Some(v.into())),
            automation_grid: self.automation_grid.and_then(|v| Some(v.into())),
            loop_color: self.loop_color.and_then(|v| Some(v.into())),
            off_grid_loop_color: self.off_grid_loop_color.and_then(|v| Some(v.into())),
            arrangement_ruler_markings: self
                .arrangement_ruler_markings
                .and_then(|v| Some(v.into())),
            detail_view_ruler_markings: self
                .detail_view_ruler_markings
                .and_then(|v| Some(v.into())),
            shadow_dark: self.shadow_dark.and_then(|v| Some(v.into())),
            shadow_light: self.shadow_light.and_then(|v| Some(v.into())),
            display_background: self.display_background.and_then(|v| Some(v.into())),
            ableton_color: self.ableton_color.and_then(|v| Some(v.into())),
            waveform_color: self.waveform_color.and_then(|v| Some(v.into())),
            // The dimmed color is going to be the same as the waveform color.
            dimmed_waveform_color: self.waveform_color.and_then(|v| Some(v.into())),
            velocity_color: self.velocity_color.and_then(|v| Some(v.into())),
            // The selected color is going to be the same as the velocity color.
            velocity_selected_or_hovered: self.velocity_color.and_then(|v| Some(v.into())),
            // This should be close
            note_probability: self.surface_background.and_then(|v| Some(v.into())),
            alert: self.alert.and_then(|v| Some(v.into())),
            control_on_foreground: self.control_on_foreground.and_then(|v| Some(v.into())),
            control_off_foreground: self.control_off_foreground.and_then(|v| Some(v.into())),
            control_on_disabled_foreground: self
                .control_on_disabled_foreground
                .and_then(|v| Some(v.into())),
            control_off_disabled_foreground: self
                .control_off_disabled_foreground
                .and_then(|v| Some(v.into())),
            control_on_alternative_foreground: self
                .control_on_alternative_foreground
                .and_then(|v| Some(v.into())),
            control_text_back: self.control_text_back.and_then(|v| Some(v.into())),
            control_contrast_frame: self.control_contrast_frame.and_then(|v| Some(v.into())),
            control_selection_frame: self.control_selection_frame.and_then(|v| Some(v.into())),
            control_contrast_transport: self
                .control_contrast_transport
                .and_then(|v| Some(v.into())),
            progress: self.progress.and_then(|v| Some(v.into())),
            progress_text: self.progress_text.and_then(|v| Some(v.into())),
            transport_progress: self.transport_progress.and_then(|v| Some(v.into())),
            clip_slot_button: self.clip_slot_button.and_then(|v| Some(v.into())),
            browser_bar: self.browser_bar.and_then(|v| Some(v.into())),
            browser_bar_overlay_hint_text_color: self
                .browser_bar_overlay_hint_text_color
                .and_then(|v| Some(v.into())),
            browser_disabled_item: self.browser_disabled_item.and_then(|v| Some(v.into())),
            browser_sample_waveform: self.browser_sample_waveform.and_then(|v| Some(v.into())),
            automation_disabled: self.automation_disabled.and_then(|v| Some(v.into())),
            automation_mouse_over: self.automation_mouse_over.and_then(|v| Some(v.into())),
            midi_note_max_velocity: self.midi_note_max_velocity.and_then(|v| Some(v.into())),
            retro_display_background: self.retro_display_background.and_then(|v| Some(v.into())),
            retro_display_background_line: self
                .retro_display_background_line
                .and_then(|v| Some(v.into())),
            retro_display_foreground: self.retro_display_foreground.and_then(|v| Some(v.into())),
            retro_display_foreground_2: self.retro_display_foreground_2.and_then(|v| Some(v.into())),
            retro_display_foreground_disabled: self
                .retro_display_foreground_disabled
                .and_then(|v| Some(v.into())),
            retro_display_green: self.retro_display_green.and_then(|v| Some(v.into())),
            retro_display_red: self.retro_display_red.and_then(|v| Some(v.into())),
            retro_display_handle_1: self.retro_display_handle_1.and_then(|v| Some(v.into())),
            retro_display_handle_2: self.retro_display_handle_2.and_then(|v| Some(v.into())),
            retro_display_scale_text: self.retro_display_scale_text.and_then(|v| Some(v.into())),
            // We're going to use the default text color here
            retro_display_title: self.control_foreground.and_then(|v| Some(v.into())),
            threshold_line_color: self.threshold_line_color.and_then(|v| Some(v.into())),
            gain_reduction_line_color: self.gain_reduction_line_color.and_then(|v| Some(v.into())),
            input_curve_color: self.input_curve_color.and_then(|v| Some(v.into())),
            input_curve_outline_color: self.input_curve_outline_color.and_then(|v| Some(v.into())),
            output_curve_color: self.output_curve_color.and_then(|v| Some(v.into())),
            output_curve_outline_color: self
                .output_curve_outline_color
                .and_then(|v| Some(v.into())),
            spectrum_default_color: self.spectrum_default_color.and_then(|v| Some(v.into())),
            spectrum_alternative_color: self
                .spectrum_alternative_color
                .and_then(|v| Some(v.into())),
            spectrum_grid_lines: self.spectrum_grid_lines.and_then(|v| Some(v.into())),
            operator_1: self.operator_1.and_then(|v| Some(v.into())),
            operator_2: self.operator_2.and_then(|v| Some(v.into())),
            operator_3: self.operator_3.and_then(|v| Some(v.into())),
            operator_4: self.operator_4.and_then(|v| Some(v.into())),
            drum_rack_scroller_1: self.drum_rack_scroller_1.and_then(|v| Some(v.into())),
            drum_rack_scroller_2: self.drum_rack_scroller_2.and_then(|v| Some(v.into())),
            filled_drum_rack_pad: self.filled_drum_rack_pad.and_then(|v| Some(v.into())),
            surface_area_focus: self.surface_area_focus.and_then(|v| Some(v.into())),
            freeze_color: self.freeze_color.and_then(|v| Some(v.into())),
            grid_label: self.grid_label.and_then(|v| Some(v.into())),
            grid_line_base: self.grid_line_base.and_then(|v| Some(v.into())),
            arranger_grid_tiles: self.arranger_grid_tiles.and_then(|v| Some(v.into())),
            detail_grid_tiles: self.detail_grid_tiles.and_then(|v| Some(v.into())),
            grid_guideline: self.grid_guideline.and_then(|v| Some(v.into())),
            off_grid_guideline: self.off_grid_guideline.and_then(|v| Some(v.into())),
            tree_column_head_background: self
                .tree_column_head_background
                .and_then(|v| Some(v.into())),
            tree_column_head_foreground: self
                .tree_column_head_foreground
                .and_then(|v| Some(v.into())),
            tree_column_head_selected: self.tree_column_head_selected.and_then(|v| Some(v.into())),
            tree_column_head_focus: self.tree_column_head_focus.and_then(|v| Some(v.into())),
            tree_column_head_control: self.tree_column_head_control.and_then(|v| Some(v.into())),
            tree_row_category_foreground: self
                .tree_row_category_foreground
                .and_then(|v| Some(v.into())),
            tree_row_category_background: self
                .tree_row_category_background
                .and_then(|v| Some(v.into())),
            search_indication: self.search_indication.and_then(|v| Some(v.into())),
            search_indication_standby: self.search_indication_standby.and_then(|v| Some(v.into())),
            key_zone_background: self.key_zone_background.and_then(|v| Some(v.into())),
            key_zone_crossfade_ramp: self.key_zone_crossfade_ramp.and_then(|v| Some(v.into())),
            velocity_zone_background: self.velocity_zone_background.and_then(|v| Some(v.into())),
            velocity_zone_crossfade_ramp: self
                .velocity_zone_crossfade_ramp
                .and_then(|v| Some(v.into())),
            selector_zone_background: self.selector_zone_background.and_then(|v| Some(v.into())),
            selector_zone_crossfade_ramp: self
                .selector_zone_crossfade_ramp
                .and_then(|v| Some(v.into())),
            view_check_control_enabled_off: self
                .view_check_control_enabled_off
                .and_then(|v| Some(v.into())),
            view_check_control_disabled_on: self
                .view_check_control_disabled_on
                .and_then(|v| Some(v.into())),
            view_check_control_disabled_off: self
                .view_check_control_disabled_off
                .and_then(|v| Some(v.into())),
            default_blend_factor: self.default_blend_factor.and_then(|v| Some(v.into())),
            icon_blend_factor: self.icon_blend_factor.and_then(|v| Some(v.into())),
            clip_blend_factor: self.clip_blend_factor.and_then(|v| Some(v.into())),
            note_border_standby_blend_factor: self
                .note_border_standby_blend_factor
                .and_then(|v| Some(v.into())),
            retro_display_blend_factor: self
                .retro_display_blend_factor
                .and_then(|v| Some(v.into())),
            check_control_not_checked_blend_factor: self
                .check_control_not_checked_blend_factor
                .and_then(|v| Some(v.into())),
            mix_surface_area_blend_factor: self
                .mix_surface_area_blend_factor
                .and_then(|v| Some(v.into())),
            text_frame_segment_blend_factor: self
                .text_frame_segment_blend_factor
                .and_then(|v| Some(v.into())),
            note_disabled_selected_blend_factor: self
                .note_disabled_selected_blend_factor
                .and_then(|v| Some(v.into())),
            background_clip: self.background_clip.and_then(|v| Some(v.into())),
            background_clip_frame: self.background_clip_frame.and_then(|v| Some(v.into())),
            warper_time_bar_ruler_background: self
                .warper_time_bar_ruler_background
                .and_then(|v| Some(v.into())),
            warper_time_bar_marker_background: self
                .warper_time_bar_marker_background
                .and_then(|v| Some(v.into())),
            min_velocity_note_blend_factor: self
                .min_velocity_note_blend_factor
                .and_then(|v| Some(v.into())),
            striped_background_shade_factor: self
                .striped_background_shade_factor
                .and_then(|v| Some(v.into())),
            non_editable_automation_alpha: self
                .non_editable_automation_alpha
                .and_then(|v| Some(v.into())),
            disabled_context_menu_icon_alpha: self
                .disabled_context_menu_icon_alpha
                .and_then(|v| Some(v.into())),
            clip_border_alpha: Some(ValueWrapper { value: 72 }),
            scroll_bar_alpha: Some(ValueWrapper { value: 100 }),
            scroll_bar_on_hover_alpha: Some(ValueWrapper { value: 255 }),
            scroll_bar_background_alpha: Some(ValueWrapper { value: 0 }),
            inaudible_take_lightness: Some(ValueWrapper { value: 0.4 }),
            inaudible_take_saturation: Some(ValueWrapper { value: 0.4 }),
            inaudible_take_name_lightness: Some(ValueWrapper { value: 0.75 }),
            inaudible_take_name_saturation: Some(ValueWrapper { value: 0.8 }),
            automation_lane_clip_body_lightness: Some(ValueWrapper { value: 0.3 }),
            automation_lane_clip_body_saturation: Some(ValueWrapper { value: 0.2 }),
            automation_lane_header_lightness: Some(ValueWrapper { value: 0.4 }),
            automation_lane_header_saturation: Some(ValueWrapper { value: 1.0 }),
            take_lane_header_lightness: Some(ValueWrapper { value: 0.4 }),
            take_lane_header_saturation: Some(ValueWrapper { value: 1.0 }),
            take_lane_header_name_lightness: Some(ValueWrapper { value: 0.85 }),
            take_lane_header_name_saturation: Some(ValueWrapper { value: 1.0 }),
            automation_lane_header_name_lightness: Some(ValueWrapper { value: 0.85 }),
            automation_lane_header_name_saturation: Some(ValueWrapper { value: 1.0 }),
            clip_contrast_color_adjustment: Some(ValueWrapper { value: 20.0 }),
            bipolar_poti_triangle: self.bipolar_poti_triangle.and_then(|v| Some(v.into())),
            poti: self.poti.and_then(|v| Some(v.into())),
            deactivated_poti: self.deactivated_poti.and_then(|v| Some(v.into())),
            poti_needle: self.poti_needle.and_then(|v| Some(v.into())),
            deactivated_poti_needle: self.deactivated_poti_needle.and_then(|v| Some(v.into())),
            transport_off_background: self.transport_off_background.and_then(|v| Some(v.into())),
            transport_off_disabled_foreground: self
                .transport_off_disabled_foreground
                .and_then(|v| Some(v.into())),
            transport_selection_background: self
                .transport_selection_background
                .and_then(|v| Some(v.into())),
            modulation: self.modulation.and_then(|v| Some(v.into())),
            modulation_disabled: self.modulation_disabled.and_then(|v| Some(v.into())),
            modulation_mouse_over: self.modulation_mouse_over.and_then(|v| Some(v.into())),
            automation_transform_tool_frame: self
                .automation_transform_tool_frame
                .and_then(|v| Some(v.into())),
            automation_transform_tool_frame_active: self
                .automation_transform_tool_frame_active
                .and_then(|v| Some(v.into())),
            automation_transform_tool_handle: self
                .automation_transform_tool_handle
                .and_then(|v| Some(v.into())),
            automation_transform_tool_handle_active: self
                .automation_transform_tool_handle_active
                .and_then(|v| Some(v.into())),
            // Using some defaults here onwards
            muted_audition_clip: Some(HexColor {
                value: common::Color {
                    r: 0xba,
                    g: 0xba,
                    b: 0xba,
                    a: 255,
                },
            }),
            // This might be a bit confusing... But I think it's fine.
            linked_track_hover: self.selection_background.and_then(|v| Some(v.into())),
            expression_lane_header_highlight: self.surface_highlight.and_then(|v| Some(v.into())),
            // This is the scrollbar on the same surface, should look good enough
            zoom_pan_handle: self.scrollbar_outer_handle.and_then(|v| Some(v.into())),
            standard_vu_meter: Some(common::Meter::default_standard_vu_meter()),
            overload_vu_meter: Some(common::Meter::default_overlad_vu_meter()),
            disabled_vu_meter: Some(common::Meter::default_disabled_vu_meter()),
            headphones_vu_meter: Some(common::Meter::default_headphones_vu_meter()),
            sends_only_vu_meter: Some(common::Meter::default_sends_only_vu_meter()),
            bipolar_gain_reduction_vu_meter: Some(
                common::Meter::default_bipolar_gain_reduction_vu_meter(),
            ),
            orange_vu_meter: Some(common::Meter::default_orange_vu_meter()),
        }
    }
}
