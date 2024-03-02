import { LRLanguage, LanguageSupport } from '@codemirror/language';
import { EditorState } from '@codemirror/state';
import { parser } from '@geeks-tracker/parser';
import { EditorView, minimalSetup } from 'codemirror';
import { useRef } from 'react';

const commandLanguage = LRLanguage.define({ name: 'geeks-tracker', parser });
const forceSingleLine = EditorState.transactionFilter.of(tr => (tr.newDoc.lines > 1 ? [] : [tr]));

function editor() {
  const state = EditorState.create({
    doc: '',
    extensions: [minimalSetup, forceSingleLine, new LanguageSupport(commandLanguage, [])],
  });
  return state;
}

// WIP
export function CommandInput() {
  const ref = useRef<EditorView>();
  const init = (elem: HTMLElement) => {
    if (ref.current != null) {
      return;
    }
    ref.current = new EditorView({ state: editor(), parent: elem });
  };
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
