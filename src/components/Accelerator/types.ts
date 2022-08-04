type Modifiers = 'CmdOrCtrl' | 'Ctrl' | 'Alt' | 'Shift';
type SpecialKeys = 'Backspace' | 'Tab' | 'Enter' | 'Esc' | 'Up' | 'Down' | 'Left' | 'Right' | 'Delete';

export type KeyboardShortcut = Modifiers | SpecialKeys | string;
