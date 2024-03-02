import { LanguageSupport, LRLanguage } from '@codemirror/language';
import { parser } from '@geeks-tracker/command';
import { EditorSelection, EditorState } from '@codemirror/state';
import { keymap, placeholder } from '@codemirror/view';
import type { Command } from '@geeks-tracker/core';
import { runCommand } from './bridges';
import { EditorView } from 'codemirror';

function parseCommand(text: string): Command | null {
  try {
    const tree = parser.configure({ strict: true }).parse(text);
    let command: Command | null = null;
    tree.iterate({
      enter: node => {
        switch (node.type.name) {
          case 'NewCommand':
            break;
          case 'SetCommand':
            break;
          default:
            break;
        }
      },
    });
  } catch {
    return null;
  }
}

export function clearEditor(view: EditorView) {
  view.dispatch({
    changes: { from: 0, to: view.state.doc.toString().length, insert: '' },
  });
}

export function selectEditorAll(view: EditorView) {
  const end = view.state.doc.toString().length;
  view.dispatch({
    selection: EditorSelection.create([EditorSelection.range(0, end)]),
  });
}

export function isEditorEmpty(view: EditorView) {
  return view.state.doc.length === 0;
}

const commandLanguage = new LanguageSupport(LRLanguage.define({ name: 'geeks-tracker', parser }), []);
const forceSingleLine = EditorState.transactionFilter.of(tr => (tr.newDoc.lines > 1 ? [] : [tr]));
const submit = keymap.of([
  {
    key: 'Enter',
    run: view => {
      const command = parseCommand(view.state.doc.toString());
      if (command != null) {
        runCommand(command);
        clearEditor(view);
      }
      return true;
    },
  },
]);

export function createEditor(
  elem: HTMLElement,
  options?: {
    placeholder?: string;
  }
) {
  const state = EditorState.create({
    doc: '',
    extensions: [forceSingleLine, placeholder(options?.placeholder ?? 'Input command...'), commandLanguage, submit],
  });
  const view = new EditorView({ state, parent: elem });
  return view;
}
