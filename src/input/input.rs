#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Input {
	Abort,
	ActionBreak,
	ActionDrop,
	ActionEdit,
	ActionFixup,
	ActionPick,
	ActionReword,
	ActionSquash,
	Backspace,
	Character(char),
	Delete,
	Edit,
	Enter,
	ForceAbort,
	ForceRebase,
	Help,
	MoveCursorDown,
	MoveCursorLeft,
	MoveCursorPageDown,
	MoveCursorPageUp,
	MoveCursorRight,
	MoveCursorUp,
	No,
	OpenInEditor,
	Other,
	Rebase,
	Resize,
	ShowCommit,
	SwapSelectedDown,
	SwapSelectedUp,
	ToggleVisualMode,
	Yes,
}
