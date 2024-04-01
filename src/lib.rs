use common::{Color, HexColor, ValueWrapper};

pub mod common;
pub mod live10;
pub mod live11;
pub mod live12;
pub mod util;

impl Into<live10::Ableton> for live11::Ableton {
    fn into(self) -> live10::Ableton {
        live10::Ableton {
            major_version: Some("5".to_owned()),
            minor_version: Some("10.0_373".to_owned()),
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
            retro_display_foreground_2: self
                .retro_display_foreground_2
                .and_then(|v| Some(v.into())),
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
            minor_version: Some("11.0_432".to_owned()),
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
            retro_display_foreground_2: self
                .retro_display_foreground_2
                .and_then(|v| Some(v.into())),
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
            zoom_pan_handle: self.surface_area.and_then(|v| Some(v.into())),
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

impl Into<live11::Ableton> for live12::Ableton {
    fn into(self) -> live11::Ableton {
        live11::Ableton {
            major_version: Some("5".to_owned()),
            minor_version: Some("11.0_432".to_owned()),
            schema_change_count: Some("3".to_owned()),
            creator: self.creator,
            revision: self.revision,
            theme: self.theme.and_then(|v| Some(v.into())),
        }
    }
}

impl Into<live11::Theme> for live12::Theme {
    fn into(self) -> live11::Theme {
        live11::Theme {
            control_foreground: self.control_foreground,
            text_disabled: self.text_disabled,
            control_disabled: self.control_disabled,
            meter_background: self.meter_background,
            surface_highlight: self.surface_highlight,
            surface_area: self.surface_area,
            surface_area_foreground: Some(HexColor {
                value: Color::default(),
            }),
            desktop: self.desktop,
            view_check_control_enabled_on: self.view_check_control_enabled_on,
            scrollbar_inner_handle: self.scrollbar_inner_handle,
            scrollbar_inner_track: self.scrollbar_inner_track,
            // going to use inner track and handle instead
            scrollbar_outer_handle: self.scrollbar_inner_handle,
            scrollbar_outer_track: self.scrollbar_inner_track,
            scrollbar_lcd_handle: self.scrollbar_lcd_handle,
            scrollbar_lcd_track: self.scrollbar_lcd_track,
            detail_view_background: self.detail_view_background,
            preferences_tab: self.preferences_tab,
            selection_frame: self.selection_frame,
            control_background: self.control_background,
            control_fill_handle: self.control_fill_handle,
            chosen_default: self.chosen_default,
            chosen_record: self.chosen_record,
            chosen_pre_listen: self.chosen_pre_listen,
            implicit_arm: self.implicit_arm,
            range_default: self.range_default,
            range_disabled: self.range_disabled,
            range_disabled_off: self.range_disabled_off,
            learn_midi: self.learn_midi,
            learn_key: self.learn_key,
            learn_macro: self.learn_macro,
            range_edit_field: self.range_edit_field,
            range_edit_field_2: self.range_edit_field_2,
            bipol_reset: self.bipol_reset,
            chosen_alternative: self.chosen_alternative,
            chosen_alert: self.chosen_alert,
            chosen_play: self.chosen_play,
            clip_1: self.clip_1,
            clip_2: self.clip_2,
            clip_3: self.clip_3,
            clip_4: self.clip_4,
            clip_5: self.clip_5,
            clip_6: self.clip_6,
            clip_7: self.clip_7,
            clip_8: self.clip_8,
            clip_9: self.clip_9,
            clip_10: self.clip_10,
            clip_11: self.clip_11,
            clip_12: self.clip_12,
            clip_13: self.clip_13,
            clip_14: self.clip_14,
            clip_15: self.clip_15,
            clip_16: self.clip_16,
            clip_text: self.clip_text,
            clip_border: self.clip_border,
            scene_contrast: self.scene_contrast,
            selection_background: self.selection_background,
            standby_selection_background: self.standby_selection_background,
            selection_foreground: self.selection_foreground,
            standby_selection_foreground: self.standby_selection_foreground,
            selection_background_contrast: self.selection_background_contrast,
            surface_background: self.surface_background,
            take_lane_track_highlighted: self.take_lane_track_highlighted,
            take_lane_track_not_highlighted: self.take_lane_track_not_highlighted,
            automation_color: self.automation_color,
            automation_grid: self.automation_grid,
            loop_color: self.loop_color,
            off_grid_loop_color: self.off_grid_loop_color,
            arrangement_ruler_markings: self.arrangement_ruler_markings,
            detail_view_ruler_markings: self.detail_view_ruler_markings,
            shadow_dark: self.shadow_dark,
            shadow_light: self.shadow_light,
            display_background: self.display_background,
            ableton_color: self.ableton_color,
            waveform_color: self.waveform_color,
            dimmed_waveform_color: self.dimmed_waveform_color,
            velocity_color: self.velocity_color,
            velocity_selected_or_hovered: self.velocity_selected_or_hovered,
            note_probability: self.note_probability,
            alert: self.alert,
            control_on_foreground: self.control_on_foreground,
            control_off_foreground: self.control_off_foreground,
            control_on_disabled_foreground: self.control_on_disabled_foreground,
            control_off_disabled_foreground: self.control_off_disabled_foreground,
            control_on_alternative_foreground: self.control_on_alternative_foreground,
            control_text_back: self.control_text_back,
            control_contrast_frame: self.control_contrast_frame,
            control_selection_frame: self.control_selection_frame,
            control_contrast_transport: self.control_contrast_transport,
            progress: self.progress,
            progress_text: self.progress_text,
            transport_progress: self.transport_progress,
            clip_slot_button: self.clip_slot_button,
            browser_bar: self.browser_bar,
            browser_bar_overlay_hint_text_color: self.browser_bar_overlay_hint_text_color,
            browser_disabled_item: self.browser_disabled_item,
            browser_sample_waveform: self.browser_sample_waveform,
            automation_disabled: self.automation_disabled,
            automation_mouse_over: self.automation_mouse_over,
            midi_note_max_velocity: self.midi_note_max_velocity,
            retro_display_background: self.retro_display_background,
            retro_display_background_line: self.retro_display_background_line,
            retro_display_foreground: self.retro_display_foreground,
            retro_display_foreground_2: self.retro_display_foreground_2,
            retro_display_foreground_disabled: self.retro_display_foreground_disabled,
            retro_display_green: self.retro_display_green,
            retro_display_red: self.retro_display_red,
            retro_display_handle_1: self.retro_display_handle_1,
            retro_display_handle_2: self.retro_display_handle_2,
            retro_display_scale_text: self.retro_display_scale_text,
            retro_display_title: self.retro_display_title,
            threshold_line_color: self.threshold_line_color,
            gain_reduction_line_color: self.gain_reduction_line_color,
            input_curve_color: self.input_curve_color,
            input_curve_outline_color: self.input_curve_outline_color,
            output_curve_color: self.output_curve_color,
            output_curve_outline_color: self.output_curve_outline_color,
            spectrum_default_color: self.spectrum_default_color,
            spectrum_alternative_color: self.spectrum_alternative_color,
            spectrum_grid_lines: self.spectrum_grid_lines,
            operator_1: self.operator_1,
            operator_2: self.operator_2,
            operator_3: self.operator_3,
            operator_4: self.operator_4,
            drum_rack_scroller_1: self.drum_rack_scroller_1,
            drum_rack_scroller_2: self.drum_rack_scroller_2,
            filled_drum_rack_pad: self.filled_drum_rack_pad,
            surface_area_focus: self.surface_area_focus,
            freeze_color: self.freeze_color,
            grid_label: self.grid_label,
            grid_line_base: self.grid_line_base,
            arranger_grid_tiles: self.arranger_grid_tiles,
            detail_grid_tiles: self.detail_grid_tiles,
            grid_guideline: self.grid_guideline,
            off_grid_guideline: self.off_grid_guideline,
            tree_column_head_background: self.tree_column_head_background,
            tree_column_head_foreground: self.tree_column_head_foreground,
            tree_column_head_selected: self.tree_column_head_selected,
            tree_column_head_focus: self.tree_column_head_focus,
            tree_column_head_control: self.tree_column_head_control,
            tree_row_category_foreground: self.tree_row_category_foreground,
            tree_row_category_background: self.tree_row_category_background,
            search_indication: self.search_indication,
            // the standby color is going to be the same
            search_indication_standby: self.search_indication,
            key_zone_background: self.key_zone_background,
            key_zone_crossfade_ramp: self.key_zone_crossfade_ramp,
            velocity_zone_background: self.velocity_zone_background,
            velocity_zone_crossfade_ramp: self.velocity_zone_crossfade_ramp,
            selector_zone_background: self.selector_zone_background,
            selector_zone_crossfade_ramp: self.selector_zone_crossfade_ramp,
            view_check_control_enabled_off: self.view_check_control_enabled_off,
            view_check_control_disabled_on: self.view_check_control_disabled_on,
            view_check_control_disabled_off: self.view_check_control_disabled_off,
            default_blend_factor: self.default_blend_factor,
            icon_blend_factor: self.icon_blend_factor,
            clip_blend_factor: self.clip_blend_factor,
            note_border_standby_blend_factor: self.note_border_standby_blend_factor,
            retro_display_blend_factor: self.retro_display_blend_factor,
            check_control_not_checked_blend_factor: self.check_control_not_checked_blend_factor,
            mix_surface_area_blend_factor: self.mix_surface_area_blend_factor,
            text_frame_segment_blend_factor: self.text_frame_segment_blend_factor,
            note_disabled_selected_blend_factor: self.note_disabled_selected_blend_factor,
            background_clip: self.background_clip,
            background_clip_frame: self.background_clip_frame,
            warper_time_bar_ruler_background: self.warper_time_bar_ruler_background,
            warper_time_bar_marker_background: self.warper_time_bar_marker_background,
            min_velocity_note_blend_factor: self.min_velocity_note_blend_factor,
            striped_background_shade_factor: self.striped_background_shade_factor,
            non_editable_automation_alpha: self.non_editable_automation_alpha,
            disabled_context_menu_icon_alpha: self.disabled_context_menu_icon_alpha,
            clip_border_alpha: self.clip_border_alpha,
            scroll_bar_alpha: self.scroll_bar_alpha,
            scroll_bar_on_hover_alpha: self.scroll_bar_on_hover_alpha,
            scroll_bar_background_alpha: self.scroll_bar_background_alpha,
            inaudible_take_lightness: self.inaudible_take_lightness,
            inaudible_take_saturation: self.inaudible_take_saturation,
            inaudible_take_name_lightness: self.inaudible_take_name_lightness,
            inaudible_take_name_saturation: self.inaudible_take_name_saturation,
            automation_lane_clip_body_lightness: self.automation_lane_clip_body_lightness,
            automation_lane_clip_body_saturation: self.automation_lane_clip_body_saturation,
            automation_lane_header_lightness: self.automation_lane_header_lightness,
            automation_lane_header_saturation: self.automation_lane_header_saturation,
            take_lane_header_lightness: self.take_lane_header_lightness,
            take_lane_header_saturation: self.take_lane_header_saturation,
            take_lane_header_name_lightness: self.take_lane_header_name_lightness,
            take_lane_header_name_saturation: self.take_lane_header_name_saturation,
            automation_lane_header_name_lightness: self.automation_lane_header_name_lightness,
            automation_lane_header_name_saturation: self.automation_lane_header_name_saturation,
            clip_contrast_color_adjustment: self.clip_contrast_color_adjustment,
            bipolar_poti_triangle: self.bipolar_poti_triangle,
            poti: self.poti,
            deactivated_poti: self.deactivated_poti,
            poti_needle: self.poti_needle,
            deactivated_poti_needle: self.deactivated_poti_needle,
            transport_off_background: self.transport_off_background,
            transport_off_disabled_foreground: self.transport_off_disabled_foreground,
            transport_selection_background: self.transport_selection_background,
            modulation: self.modulation,
            modulation_disabled: self.modulation_disabled,
            modulation_mouse_over: self.modulation_mouse_over,
            automation_transform_tool_frame: self.automation_transform_tool_frame,
            automation_transform_tool_frame_active: self.automation_transform_tool_frame_active,
            automation_transform_tool_handle: self.automation_transform_tool_handle,
            automation_transform_tool_handle_active: self.automation_transform_tool_handle_active,
            muted_audition_clip: self.muted_audition_clip,
            linked_track_hover: self.linked_track_hover,
            expression_lane_header_highlight: self.expression_lane_header_highlight,
            // the pan handle disappears in 12
            zoom_pan_handle: self.scrollbar_inner_handle,
            standard_vu_meter: self.standard_vu_meter,
            overload_vu_meter: self.overload_vu_meter,
            disabled_vu_meter: self.disabled_vu_meter,
            headphones_vu_meter: self.headphones_vu_meter,
            sends_only_vu_meter: self.sends_only_vu_meter,
            bipolar_gain_reduction_vu_meter: self.bipolar_gain_reduction_vu_meter,
            orange_vu_meter: self.orange_vu_meter,
        }
    }
}

impl Into<live12::Ableton> for live11::Ableton {
    fn into(self) -> live12::Ableton {
        live12::Ableton {
            major_version: Some("5".to_owned()),
            minor_version: Some("12.0_12047".to_owned()),
            schema_change_count: Some("2".to_owned()),
            creator: self.creator,
            revision: self.revision,
            theme: self.theme.and_then(|v| Some(v.into())),
        }
    }
}

impl Into<live12::Theme> for live11::Theme {
    fn into(self) -> live12::Theme {
        live12::Theme {
            control_foreground: self.control_foreground,
            text_disabled: self.text_disabled,
            control_disabled: self.control_disabled,
            meter_background: self.meter_background,
            surface_highlight: self.surface_highlight,
            surface_area: self.surface_area,
            desktop: self.desktop,
            view_check_control_enabled_on: self.view_check_control_enabled_on,
            scrollbar_inner_handle: self.scrollbar_inner_handle,
            scrollbar_inner_track: self.scrollbar_inner_track,
            scrollbar_lcd_handle: self.scrollbar_lcd_handle,
            scrollbar_lcd_track: self.scrollbar_lcd_track,
            // using scrollbar outer handle
            scrollbar_mixer_show_on_scroll_handle: Some(HexColor {
                value: Color {
                    a: 0x66,
                    ..Default::default()
                },
            }),
            detail_view_background: self.detail_view_background,
            preferences_tab: self.preferences_tab,
            selection_frame: self.selection_frame,
            control_background: self.control_background,
            control_fill_handle: self.control_fill_handle,
            chosen_default: self.chosen_default,
            chosen_record: self.chosen_record,
            chosen_pre_listen: self.chosen_pre_listen,
            implicit_arm: self.implicit_arm,
            range_default: self.range_default,
            range_disabled: self.range_disabled,
            range_disabled_off: self.range_disabled_off,
            learn_midi: self.learn_midi,
            learn_key: self.learn_key,
            learn_macro: self.learn_macro,
            range_edit_field: self.range_edit_field,
            range_edit_field_2: self.range_edit_field_2,
            bipol_reset: self.bipol_reset,
            chosen_alternative: self.chosen_alternative,
            chosen_alert: self.chosen_alert,
            chosen_play: self.chosen_play,
            clip_1: self.clip_1,
            clip_2: self.clip_2,
            clip_3: self.clip_3,
            clip_4: self.clip_4,
            clip_5: self.clip_5,
            clip_6: self.clip_6,
            clip_7: self.clip_7,
            clip_8: self.clip_8,
            clip_9: self.clip_9,
            clip_10: self.clip_10,
            clip_11: self.clip_11,
            clip_12: self.clip_12,
            clip_13: self.clip_13,
            clip_14: self.clip_14,
            clip_15: self.clip_15,
            clip_16: self.clip_16,
            clip_text: self.clip_text,
            clip_border: self.clip_border,
            scene_contrast: self.scene_contrast,
            selection_background: self.selection_background,
            standby_selection_background: self.standby_selection_background,
            selection_foreground: self.selection_foreground,
            standby_selection_foreground: self.standby_selection_foreground,
            selection_background_contrast: self.selection_background_contrast,
            surface_background: self.surface_background,
            take_lane_track_highlighted: self.take_lane_track_highlighted,
            take_lane_track_not_highlighted: self.take_lane_track_not_highlighted,
            automation_color: self.automation_color,
            automation_grid: self.automation_grid,
            loop_color: self.loop_color,
            off_grid_loop_color: self.off_grid_loop_color,
            arrangement_ruler_markings: self.arrangement_ruler_markings,
            detail_view_ruler_markings: self.detail_view_ruler_markings,
            shadow_dark: self.shadow_dark,
            shadow_light: self.shadow_light,
            display_background: self.display_background,
            ableton_color: self.ableton_color,
            waveform_color: self.waveform_color,
            dimmed_waveform_color: self.dimmed_waveform_color,
            velocity_color: self.velocity_color,
            velocity_selected_or_hovered: self.velocity_selected_or_hovered,
            note_probability: self.note_probability,
            alert: self.alert,
            control_on_foreground: self.control_on_foreground,
            control_off_foreground: self.control_off_foreground,
            control_on_disabled_foreground: self.control_on_disabled_foreground,
            control_off_disabled_foreground: self.control_off_disabled_foreground,
            control_on_alternative_foreground: self.control_on_alternative_foreground,
            control_text_back: self.control_text_back,
            control_contrast_frame: self.control_contrast_frame,
            control_selection_frame: self.control_selection_frame,
            control_contrast_transport: self.control_contrast_transport,
            // These will emulate the old theme behaviour
            view_control_on: self.view_check_control_enabled_on,
            view_control_off: self.transport_off_background,
            progress: self.progress,
            progress_text: self.progress_text,
            transport_progress: self.transport_progress,
            clip_slot_button: self.clip_slot_button,
            browser_bar: self.browser_bar,
            browser_bar_overlay_hint_text_color: self.browser_bar_overlay_hint_text_color,
            browser_disabled_item: self.browser_disabled_item,
            browser_sample_waveform: self.browser_sample_waveform,
            automation_disabled: self.automation_disabled,
            automation_mouse_over: self.automation_mouse_over,
            midi_note_max_velocity: self.midi_note_max_velocity,
            retro_display_background: self.retro_display_background,
            retro_display_background_line: self.retro_display_background_line,
            retro_display_foreground: self.retro_display_foreground,
            retro_display_foreground_2: self.retro_display_foreground_2,
            retro_display_foreground_disabled: self.retro_display_foreground_disabled,
            retro_display_green: self.retro_display_green,
            retro_display_red: self.retro_display_red,
            retro_display_handle_1: self.retro_display_handle_1,
            retro_display_handle_2: self.retro_display_handle_2,
            retro_display_scale_text: self.retro_display_scale_text,
            retro_display_title: self.retro_display_title,
            threshold_line_color: self.threshold_line_color,
            gain_reduction_line_color: self.gain_reduction_line_color,
            input_curve_color: self.input_curve_color,
            input_curve_outline_color: self.input_curve_outline_color,
            output_curve_color: self.output_curve_color,
            output_curve_outline_color: self.output_curve_outline_color,
            spectrum_default_color: self.spectrum_default_color,
            spectrum_alternative_color: self.spectrum_alternative_color,
            spectrum_grid_lines: self.spectrum_grid_lines,
            operator_1: self.operator_1,
            operator_2: self.operator_2,
            operator_3: self.operator_3,
            operator_4: self.operator_4,
            drum_rack_scroller_1: self.drum_rack_scroller_1,
            drum_rack_scroller_2: self.drum_rack_scroller_2,
            filled_drum_rack_pad: self.filled_drum_rack_pad,
            surface_area_focus: self.surface_area_focus,
            freeze_color: self.freeze_color,
            grid_label: self.grid_label,
            grid_line_base: self.grid_line_base,
            arranger_grid_tiles: self.arranger_grid_tiles,
            detail_grid_tiles: self.detail_grid_tiles,
            grid_guideline: self.grid_guideline,
            off_grid_guideline: self.off_grid_guideline,
            tree_column_head_background: self.tree_column_head_background,
            tree_column_head_foreground: self.tree_column_head_foreground,
            tree_column_head_selected: self.tree_column_head_selected,
            tree_column_head_focus: self.tree_column_head_focus,
            tree_column_head_control: self.tree_column_head_control,
            tree_row_category_foreground: self.tree_row_category_foreground,
            tree_row_category_background: self.tree_row_category_background,
            // this will emulate the little buttons that exist on the mixer
            browser_tag_background: self.control_text_back,
            search_indication: self.search_indication,
            key_zone_background: self.key_zone_background,
            key_zone_crossfade_ramp: self.key_zone_crossfade_ramp,
            velocity_zone_background: self.velocity_zone_background,
            velocity_zone_crossfade_ramp: self.velocity_zone_crossfade_ramp,
            selector_zone_background: self.selector_zone_background,
            selector_zone_crossfade_ramp: self.selector_zone_crossfade_ramp,
            view_check_control_enabled_off: self.view_check_control_enabled_off,
            view_check_control_disabled_on: self.view_check_control_disabled_on,
            view_check_control_disabled_off: self.view_check_control_disabled_off,
            default_blend_factor: self.default_blend_factor,
            icon_blend_factor: self.icon_blend_factor,
            clip_blend_factor: self.clip_blend_factor,
            note_border_standby_blend_factor: self.note_border_standby_blend_factor,
            retro_display_blend_factor: self.retro_display_blend_factor,
            check_control_not_checked_blend_factor: self.check_control_not_checked_blend_factor,
            mix_surface_area_blend_factor: self.mix_surface_area_blend_factor,
            text_frame_segment_blend_factor: self.text_frame_segment_blend_factor,
            note_disabled_selected_blend_factor: self.note_disabled_selected_blend_factor,
            background_clip: self.background_clip,
            background_clip_frame: self.background_clip_frame,
            warper_time_bar_ruler_background: self.warper_time_bar_ruler_background,
            warper_time_bar_marker_background: self.warper_time_bar_marker_background,
            min_velocity_note_blend_factor: self.min_velocity_note_blend_factor,
            striped_background_shade_factor: self.striped_background_shade_factor,
            non_editable_automation_alpha: self.non_editable_automation_alpha,
            disabled_context_menu_icon_alpha: self.disabled_context_menu_icon_alpha,
            clip_border_alpha: self.clip_border_alpha,
            scroll_bar_alpha: self.scroll_bar_alpha,
            scroll_bar_on_hover_alpha: self.scroll_bar_on_hover_alpha,
            scroll_bar_background_alpha: self.scroll_bar_background_alpha,
            inaudible_take_lightness: self.inaudible_take_lightness,
            inaudible_take_saturation: self.inaudible_take_saturation,
            inaudible_take_name_lightness: self.inaudible_take_name_lightness,
            inaudible_take_name_saturation: self.inaudible_take_name_saturation,
            automation_lane_clip_body_lightness: self.automation_lane_clip_body_lightness,
            automation_lane_clip_body_saturation: self.automation_lane_clip_body_saturation,
            automation_lane_header_lightness: self.automation_lane_header_lightness,
            automation_lane_header_saturation: self.automation_lane_header_saturation,
            take_lane_header_lightness: self.take_lane_header_lightness,
            take_lane_header_saturation: self.take_lane_header_saturation,
            take_lane_header_name_lightness: self.take_lane_header_name_lightness,
            take_lane_header_name_saturation: self.take_lane_header_name_saturation,
            automation_lane_header_name_lightness: self.automation_lane_header_name_lightness,
            automation_lane_header_name_saturation: self.automation_lane_header_name_saturation,
            clip_contrast_color_adjustment: self.clip_contrast_color_adjustment,
            // Have no idea what this is, default to 50
            session_slot_oklab_l_compensation_factor: Some(ValueWrapper { value: 50.0 }),
            bipolar_poti_triangle: self.bipolar_poti_triangle,
            poti: self.poti,
            deactivated_poti: self.deactivated_poti,
            poti_needle: self.poti_needle,
            deactivated_poti_needle: self.deactivated_poti_needle,
            piano_black_key: Some(HexColor {
                value: Color {
                    r: 0x31,
                    g: 0x31,
                    b: 0x31,
                    a: 0xff,
                },
            }),
            piano_white_key: Some(HexColor {
                value: Color {
                    r: 0xe0,
                    g: 0xe0,
                    b: 0xe0,
                    a: 0xff,
                },
            }),
            piano_key_separator: Some(HexColor {
                value: Color {
                    r: 0x4f,
                    g: 0x4f,
                    b: 0x4f,
                    a: 0xff,
                },
            }),
            transport_off_background: self.transport_off_background,
            transport_off_disabled_foreground: self.transport_off_disabled_foreground,
            // using the background and the foreground the same
            transport_off_foreground: self.control_off_foreground,
            transport_selection_background: self.transport_selection_background,
            modulation: self.modulation,
            modulation_disabled: self.modulation_disabled,
            modulation_mouse_over: self.modulation_mouse_over,
            automation_transform_tool_frame: self.automation_transform_tool_frame,
            automation_transform_tool_frame_active: self.automation_transform_tool_frame_active,
            automation_transform_tool_handle: self.automation_transform_tool_handle,
            automation_transform_tool_handle_active: self.automation_transform_tool_handle_active,
            muted_audition_clip: self.muted_audition_clip,
            linked_track_hover: self.linked_track_hover,
            expression_lane_header_highlight: self.expression_lane_header_highlight,
            // use defaults
            deactivated_clip_header: Some(HexColor {
                value: Color {
                    r: 0x99,
                    g: 0x99,
                    b: 0x99,
                    a: 0xdf,
                },
            }),
            deactivated_clip_header_foreground: Some(HexColor {
                value: Color {
                    r: 0x39,
                    g: 0x39,
                    b: 0x39,
                    a: 0xff,
                },
            }),
            // this is a toggle, use chosen default
            scale_awareness: self.chosen_default,
            standard_vu_meter: self.standard_vu_meter,
            overload_vu_meter: self.overload_vu_meter,
            disabled_vu_meter: self.disabled_vu_meter,
            headphones_vu_meter: self.headphones_vu_meter,
            sends_only_vu_meter: self.sends_only_vu_meter,
            bipolar_gain_reduction_vu_meter: self.bipolar_gain_reduction_vu_meter,
            orange_vu_meter: self.orange_vu_meter,
            // the zoom pan handle is now apart of the same thing
            main_view_focus_indicator: self.zoom_pan_handle,
            midi_editor_black_key_background: Some(HexColor {
                value: Color {
                    r: 0x0a,
                    g: 0x0a,
                    b: 0x0a,
                    a: 0x19,
                },
            }),
            midi_editor_background_white_key_separator: Some(HexColor {
                value: Color {
                    r: 0x14,
                    g: 0x14,
                    b: 0x14,
                    a: 0x19,
                },
            }),
            // there are 3 of them now, not much we can do but just reuse
            range_edit_field_3: self.range_edit_field_2,
            // hovers willbe the base color
            scrollbar_inner_handle_hover: self.scrollbar_inner_handle,
            scrollbar_inner_track_hover: self.scrollbar_inner_track,
            scrollbar_lcd_handle_hover: self.scrollbar_lcd_handle,
            scrollbar_lcd_track_hover: self.scrollbar_lcd_track,
            scrollbar_mixer_show_on_scroll_handle_hover: Some(HexColor {
                value: Color {
                    a: 0x4c,
                    ..Default::default()
                },
            }),
        }
    }
}
