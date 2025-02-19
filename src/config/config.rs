use crate::color::Color;
use crate::config::utils::{editor_from_env, get_bool, get_color, get_input, get_string, open_git_config};
use crate::config::Theme;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub struct Config {
	pub theme: Theme,
	pub auto_select_next: bool,
	pub comment_char: String,
	pub editor: String,
	pub input_abort: String,
	pub input_action_break: String,
	pub input_action_drop: String,
	pub input_action_edit: String,
	pub input_action_fixup: String,
	pub input_action_pick: String,
	pub input_action_reword: String,
	pub input_action_squash: String,
	pub input_confirm_no: String,
	pub input_confirm_yes: String,
	pub input_edit: String,
	pub input_force_abort: String,
	pub input_force_rebase: String,
	pub input_help: String,
	pub input_move_down: String,
	pub input_move_down_step: String,
	pub input_move_left: String,
	pub input_move_right: String,
	pub input_move_selection_down: String,
	pub input_move_selection_up: String,
	pub input_move_up: String,
	pub input_move_up_step: String,
	pub input_open_in_external_editor: String,
	pub input_rebase: String,
	pub input_show_commit: String,
	pub input_toggle_visual_mode: String,
}

impl Config {
	pub fn new() -> Result<Self, String> {
		let git_config = open_git_config()?;
		Ok(Config {
			theme: Theme {
				color_foreground: get_color(&git_config, "interactive-rebase-tool.foregroundColor", Color::White)?,
				color_background: get_color(&git_config, "interactive-rebase-tool.backgroundColor", Color::Default)?,
				color_selected_background: get_color(
					&git_config,
					"interactive-rebase-tool.selectedBackgroundColor",
					Color::try_from("35,35,40").unwrap(),
				)?,
				color_indicator: get_color(&git_config, "interactive-rebase-tool.indicatorColor", Color::Cyan)?,
				color_action_break: get_color(&git_config, "interactive-rebase-tool.breakColor", Color::White)?,
				color_action_drop: get_color(&git_config, "interactive-rebase-tool.dropColor", Color::Red)?,
				color_action_edit: get_color(&git_config, "interactive-rebase-tool.editColor", Color::Blue)?,
				color_action_exec: get_color(&git_config, "interactive-rebase-tool.execColor", Color::White)?,
				color_action_fixup: get_color(&git_config, "interactive-rebase-tool.fixupColor", Color::Magenta)?,
				color_action_pick: get_color(&git_config, "interactive-rebase-tool.pickColor", Color::Green)?,
				color_action_reword: get_color(&git_config, "interactive-rebase-tool.rewordColor", Color::Yellow)?,
				color_action_squash: get_color(&git_config, "interactive-rebase-tool.squashColor", Color::Cyan)?,
				color_diff_add: get_color(&git_config, "interactive-rebase-tool.diffAddColor", Color::Green)?,
				color_diff_change: get_color(&git_config, "interactive-rebase-tool.diffChangeColor", Color::Yellow)?,
				color_diff_remove: get_color(&git_config, "interactive-rebase-tool.diffRemoveColor", Color::Red)?,
				character_vertical_spacing: get_string(
					&git_config,
					"interactive-rebase-tool.verticalSpacingCharacter",
					"~",
				)?,
			},
			auto_select_next: get_bool(&git_config, "interactive-rebase-tool.autoSelectNext", false)?,
			comment_char: get_string(&git_config, "core.commentChar", "#")?,
			editor: get_string(&git_config, "core.editor", editor_from_env().as_str())?,
			input_abort: get_input(&git_config, "interactive-rebase-tool.inputAbort", "q")?,
			input_action_break: get_input(&git_config, "interactive-rebase-tool.inputActionBreak", "b")?,
			input_action_drop: get_input(&git_config, "interactive-rebase-tool.inputActionDrop", "d")?,
			input_action_edit: get_input(&git_config, "interactive-rebase-tool.inputActionEdit", "e")?,
			input_action_fixup: get_input(&git_config, "interactive-rebase-tool.inputActionFixup", "f")?,
			input_action_pick: get_input(&git_config, "interactive-rebase-tool.inputActionPick", "p")?,
			input_action_reword: get_input(&git_config, "interactive-rebase-tool.inputActionReword", "r")?,
			input_action_squash: get_input(&git_config, "interactive-rebase-tool.inputActionSquash", "s")?,
			input_confirm_no: get_input(&git_config, "interactive-rebase-tool.inputConfirmNo", "n")?,
			input_confirm_yes: get_input(&git_config, "interactive-rebase-tool.inputConfirmYes", "y")?,
			input_edit: get_input(&git_config, "interactive-rebase-tool.inputEdit", "E")?,
			input_force_abort: get_input(&git_config, "interactive-rebase-tool.inputForceAbort", "Q")?,
			input_force_rebase: get_input(&git_config, "interactive-rebase-tool.inputForceRebase", "W")?,
			input_help: get_input(&git_config, "interactive-rebase-tool.inputHelp", "?")?,
			input_move_down: get_input(&git_config, "interactive-rebase-tool.inputMoveDown", "Down")?,
			input_move_left: get_input(&git_config, "interactive-rebase-tool.inputMoveLeft", "Left")?,
			input_move_right: get_input(&git_config, "interactive-rebase-tool.inputMoveRight", "Right")?,
			input_move_up_step: get_input(&git_config, "interactive-rebase-tool.inputMoveStepUp", "PageUp")?,
			input_move_down_step: get_input(&git_config, "interactive-rebase-tool.inputMoveStepDown", "PageDown")?,
			input_move_selection_down: get_input(&git_config, "interactive-rebase-tool.inputMoveSelectionDown", "j")?,
			input_move_selection_up: get_input(&git_config, "interactive-rebase-tool.inputMoveSelectionUp", "k")?,
			input_move_up: get_input(&git_config, "interactive-rebase-tool.inputMoveUp", "Up")?,
			input_open_in_external_editor: get_input(
				&git_config,
				"interactive-rebase-tool.inputOpenInExternalEditor",
				"!",
			)?,
			input_rebase: get_input(&git_config, "interactive-rebase-tool.inputRebase", "w")?,
			input_show_commit: get_input(&git_config, "interactive-rebase-tool.inputShowCommit", "c")?,
			input_toggle_visual_mode: get_input(&git_config, "interactive-rebase-tool.inputToggleVisualMode", "v")?,
		})
	}
}
