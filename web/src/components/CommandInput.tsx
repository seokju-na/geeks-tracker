import type { EditorView } from 'codemirror';
import { useRef } from 'react';
import { useSubscription } from 'observable-hooks';
import { appFocused$ } from '../events';
import { createEditor, isEditorEmpty, selectEditorAll } from '../editor';

// WIP
export function CommandInput() {
  const ref = useRef<EditorView>();
  const init = (elem: HTMLElement) => {
    if (ref.current == null) {
      ref.current = createEditor(elem);
    }
  };
  useSubscription(appFocused$, () => {
    const editor = ref.current;
    if (editor != null) {
      editor.focus();
      if (!isEditorEmpty(editor)) {
        selectEditorAll(editor);
      }
    }
  });
  return (
    <div
      ref={elem => {
        if (elem != null) {
          init(elem);
        }
      }}
      className="w-full h-[44px] outline-none pointer-events-none"
    />
  );
}
