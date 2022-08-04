import hotkeys from 'hotkeys-js';
import { Children, cloneElement, ReactElement, useEffect } from 'react';
import { usePreservedCallback } from '../../hooks/usePreservedCallback';
import { NonEmptyArray } from '../../utils/array';
import { noop } from '../../utils/noop';
import { createHotkey, formatHotkey } from './hotkey';
import { KeyboardShortcut } from './types';

interface Props {
  shortcut: NonEmptyArray<KeyboardShortcut>;
  onKeyPress?: () => void;
  children: ReactElement;
}

/** Register accelerator with shortcut */
export function Accelerator({ shortcut, onKeyPress = noop, children }: Props) {
  if (shortcut.length === 0) {
    throw new Error('Accelerator key cannot be empty');
  }

  const handleKeyPress = usePreservedCallback(onKeyPress);
  const hotkey = createHotkey(shortcut);
  const hotkeyLabel = formatHotkey(shortcut);

  const child = Children.only(children);
  const cloned = cloneElement(Children.only(children), {
    ...child.props,
    onClick: (...args: unknown[]) => {
      child.props.onClick?.(...args);
      handleKeyPress();
    },
    'aria-label': `${child.props['aria-label'] ?? ''} (${hotkeyLabel})`.trimStart(),
  });

  const disabled = cloned.props.disabled ?? false;

  useEffect(() => {
    if (disabled) {
      hotkeys.unbind(hotkey, handleKeyPress);
      return;
    }

    hotkeys(hotkey, handleKeyPress);
    return () => {
      hotkeys.unbind(hotkey, handleKeyPress);
    };
  }, [disabled, hotkey, handleKeyPress]);

  return <>{cloned}</>;
}
