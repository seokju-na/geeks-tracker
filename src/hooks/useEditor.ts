import { defaultKeymap } from '@codemirror/commands';
import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
import { bracketMatching, indentOnInput } from '@codemirror/language';
import { languages } from '@codemirror/language-data';
import { EditorState, Text } from '@codemirror/state';
import { EditorView, highlightActiveLine, highlightActiveLineGutter, keymap } from '@codemirror/view';
import { useEffect, useRef, useState } from 'react';
import { noop } from '../utils/noop';
import { usePreservedCallback } from './usePreservedCallback';

interface Props {
  initialDoc?: string | Text;
  onChange?: (state: EditorState) => void;
}

export function useEditor<T extends Element>({ initialDoc = '', onChange = noop }: Props = {}) {
  const elemRef = useRef<T>(null);
  const changeCallback = usePreservedCallback(onChange);
  const [view, setView] = useState<EditorView>();

  useEffect(() => {
    if (elemRef.current == null) {
      return;
    }

    const startState = EditorState.create({
      doc: initialDoc,
      extensions: [
        keymap.of([
          ...defaultKeymap,
          {
            key: 'Ctrl-s',
            mac: 'Cmd-s',
            run: view => {
              changeCallback(view.state);
              return true;
            },
          },
        ]),
        highlightActiveLineGutter(),
        indentOnInput(),
        bracketMatching(),
        highlightActiveLine(),
        markdown({
          base: markdownLanguage,
          codeLanguages: languages,
          addKeymap: true,
        }),
        EditorView.lineWrapping,
        EditorView.updateListener.of(update => {
          if (update.changes) {
            changeCallback(update.state);
          }
        }),
      ],
    });
    const editorView = new EditorView({
      state: startState,
      parent: elemRef.current,
    });
    setView(editorView);

    return () => {
      editorView.destroy();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return [elemRef, view] as const;
}
