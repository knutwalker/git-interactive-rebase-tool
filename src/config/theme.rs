use crate::config::utils::{get_color, get_string};
use crate::display::color::Color;
use git2::Config;

#[derive(Clone, Debug)]
pub(crate) struct Theme {
	pub(crate) color_foreground: Color,
	pub(crate) color_background: Color,
	pub(crate) color_selected_background: Color,
	pub(crate) color_indicator: Color,
	pub(crate) color_action_break: Color,
	pub(crate) color_action_drop: Color,
	pub(crate) color_action_edit: Color,
	pub(crate) color_action_exec: Color,
	pub(crate) color_action_fixup: Color,
	pub(crate) color_action_pick: Color,
	pub(crate) color_action_reword: Color,
	pub(crate) color_action_squash: Color,
	pub(crate) color_diff_add: Color,
	pub(crate) color_diff_change: Color,
	pub(crate) color_diff_remove: Color,
	pub(crate) color_diff_context: Color,
	pub(crate) color_diff_whitespace: Color,
	pub(crate) character_vertical_spacing: String,
}

impl Theme {
	pub(super) fn new(git_config: &Config) -> Result<Self, String> {
		Ok(Self {
			color_foreground: get_color(&git_config, "interactive-rebase-tool.foregroundColor", Color::Default)?,
			color_background: get_color(&git_config, "interactive-rebase-tool.backgroundColor", Color::Default)?,
			color_selected_background: get_color(
				&git_config,
				"interactive-rebase-tool.selectedBackgroundColor",
				Color::Index(237),
			)?,
			color_indicator: get_color(&git_config, "interactive-rebase-tool.indicatorColor", Color::LightCyan)?,
			color_action_break: get_color(&git_config, "interactive-rebase-tool.breakColor", Color::LightWhite)?,
			color_action_drop: get_color(&git_config, "interactive-rebase-tool.dropColor", Color::LightRed)?,
			color_action_edit: get_color(&git_config, "interactive-rebase-tool.editColor", Color::LightBlue)?,
			color_action_exec: get_color(&git_config, "interactive-rebase-tool.execColor", Color::LightWhite)?,
			color_action_fixup: get_color(&git_config, "interactive-rebase-tool.fixupColor", Color::LightMagenta)?,
			color_action_pick: get_color(&git_config, "interactive-rebase-tool.pickColor", Color::LightGreen)?,
			color_action_reword: get_color(&git_config, "interactive-rebase-tool.rewordColor", Color::LightYellow)?,
			color_action_squash: get_color(&git_config, "interactive-rebase-tool.squashColor", Color::LightCyan)?,
			color_diff_add: get_color(&git_config, "interactive-rebase-tool.diffAddColor", Color::LightGreen)?,
			color_diff_change: get_color(
				&git_config,
				"interactive-rebase-tool.diffChangeColor",
				Color::LightYellow,
			)?,
			color_diff_remove: get_color(&git_config, "interactive-rebase-tool.diffRemoveColor", Color::LightRed)?,
			color_diff_context: get_color(
				&git_config,
				"interactive-rebase-tool.diffContextColor",
				Color::LightWhite,
			)?,
			color_diff_whitespace: get_color(&git_config, "interactive-rebase-tool.diffWhitespace", Color::LightBlack)?,
			character_vertical_spacing: get_string(
				&git_config,
				"interactive-rebase-tool.verticalSpacingCharacter",
				"~",
			)?,
		})
	}
}
