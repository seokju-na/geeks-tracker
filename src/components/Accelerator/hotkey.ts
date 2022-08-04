import { getOSType, OSType } from '../../utils/osType';
import { KeyboardShortcut } from './types';

export function createHotkey(shortcut: KeyboardShortcut[]) {
  const keys = shortcut.map(x => {
    if (x === 'CmdOrCtrl') {
      return getOSType() === 'macos' ? 'command' : 'ctrl';
    }
    return x.toLowerCase();
  });

  return keys.join('+');
}

export function formatHotkey(shortcut: KeyboardShortcut[]) {
  const keys = shortcut.map(x => {
    const transform = transforms.find(t => t[0] === x);
    return transform?.[1][getOSType()] ?? x;
  });
  return keys.join('+');
}

const transforms: Array<[string, Partial<Record<OSType, string>>]> = [
  [
    'CmdOrCtrl',
    {
      macos: '⌘',
      linux: 'Ctrl',
      windows: 'Ctrl',
    },
  ],
  ['Ctrl', { macos: '⌃' }],
  ['Alt', { macos: '⌥' }], // alternative to option
  ['Shift', { macos: '⇧' }],
  ['Delete', { macos: '⌫' }],
];
