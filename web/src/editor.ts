import { HighlightStyle, LanguageSupport, LRLanguage, syntaxHighlighting } from '@codemirror/language';
import { EditorSelection, EditorState } from '@codemirror/state';
import { drawSelection, highlightSpecialChars, keymap, placeholder } from '@codemirror/view';
import { parser } from '../../language';
import type { Command, TaskStatus } from '@geeks-tracker/core';
import { EditorView } from 'codemirror';
import { runCommand } from './bridges';
import ms, { type StringValue } from './ms';
import { tags } from '@lezer/highlight';

function parseCommand(text: string): Command | null {
  try {
    const tree = parser.configure({ strict: true }).parse(text);
    let command: Command | null = null;
    tree.iterate({
      enter: ref => {
        switch (ref.type.name) {
          case 'NewCommand': {
            const titleNode = ref.node.getChild('String')!;
            const title = text.slice(titleNode.from + 1, titleNode.to - 1);
            const withStatusNode = ref.node.getChild('WithStatus');
            const statusNode = withStatusNode?.getChild('TaskStatus');
            const status = statusNode != null ? (text.slice(statusNode.from, statusNode.to) as TaskStatus) : undefined;
            const durationNode = withStatusNode?.getChild('Duration');
            const duration =
              durationNode != null ? ms(text.slice(durationNode.from, durationNode.to) as StringValue) : undefined;
            command =
              status != null && duration != null
                ? {
                    name: 'task.create',
                    data: {
                      title,
                      schedule: {
                        at: Date.now() + duration,
                        status,
                      },
                    },
                  }
                : {
                    name: 'task.create',
                    data: {
                      title,
                      status,
                    },
                  };
            break;
          }
          case 'SetCommand':
            {
              const taskIdNode = ref.node.getChild('TaskId')!;
              const taskId = text.slice(taskIdNode.from, taskIdNode.to);
              const withTitleNode = ref.node.getChild('WithTitle');
              if (withTitleNode != null) {
                const titleNode = withTitleNode.getChild('String')!;
                const title = text.slice(titleNode.from + 1, titleNode.to - 1);
                command = {
                  name: 'task.updateTitle',
                  data: {
                    id: taskId,
                    title,
                  },
                };
              }
              const withStatusNode = ref.node.getChild('WithStatus');
              if (withStatusNode != null) {
                const statusNode = withStatusNode.getChild('TaskStatus')!;
                const status = text.slice(statusNode.from, statusNode.to) as TaskStatus;
                const durationNode = withStatusNode.getChild('Duration');
                const duration =
                  durationNode != null ? ms(text.slice(durationNode.from, durationNode.to) as StringValue) : undefined;
                console.log(
                  durationNode != null ? text.slice(durationNode.from, durationNode.to) : undefined,
                  duration
                );
                command =
                  duration != null
                    ? {
                        name: 'task.updateSchedule',
                        data: {
                          id: taskId,
                          schedule: {
                            at: Date.now() + duration,
                            status,
                          },
                        },
                      }
                    : {
                        name: 'task.updateStatus',
                        data: {
                          id: taskId,
                          status,
                        },
                      };
              }
            }
            break;
        }
      },
    });
    console.log('command', command);
    return command;
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
        runCommand(command)
          .then(() => clearEditor(view))
          .catch(e => {
            // TODO: tell user that command not worked
            console.error(e);
          });
      }
      return true;
    },
  },
]);
const theme = EditorView.theme(
  {
    '&': {
      fontSize: '0.875rem',
      fontWeight: 600,
      color: 'white',
      backgroundColor: 'transparent',
    },
    '.cm-cursor': {
      borderLeftColor: 'white',
    },
    '.cm-content': {
      caretColor: 'white',
    },
    '.cm-selectionBackground, ::selection': {
      backgroundColor: '#5B9DD9 !important',
    },
  },
  { dark: true }
);
const highlighting = HighlightStyle.define([
  { tag: tags.keyword, color: '#cf8e6d' },
  { tag: tags.string, color: '#6aab73' },
  { tag: tags.name, color: '#56a8f5' },
]);

export function createEditor(
  elem: HTMLElement,
  options?: {
    placeholder?: string;
  }
) {
  const state = EditorState.create({
    doc: '',
    extensions: [
      theme,
      drawSelection(),
      highlightSpecialChars(),
      syntaxHighlighting(highlighting, { fallback: true }),
      forceSingleLine,
      placeholder(options?.placeholder ?? 'Input command...'),
      commandLanguage,
      submit,
    ],
  });
  const view = new EditorView({ state, parent: elem });
  return view;
}
