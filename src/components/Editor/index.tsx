import { useEditor } from '../../hooks/useEditor';
import { styled } from '../../styles';

interface Props {
  initialDoc?: string;
}

export function Editor(props: Props) {
  const [elemRef] = useEditor<HTMLDivElement>(props);

  return <Wrapper ref={elemRef} />;
}

const Wrapper = styled('div', {
  outline: 'none',
  '.cm-editor': {
    color: '$text',
    backgroundColor: '$background',
    fontFamily:
      'ui-sans-serif, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Inter", "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Microsoft YaHei Light", sans-serif',
    fontWeight: 500,
  },
  '.cm-content': {
    caretColor: '$text',
  },
  '.cm-focused .cm-selectionBackground, ::selection': {
    backgroundColor: '$blue100',
  },
  '.cm-gutters': {
    backgroundColor: '$background',
    borderRight: '1px solid $divider',
    color: '$text',
  },
  '.cm-activeLine': {
    backgroundColor: '$backgroundHighlighted',
    fontWeight: 400,
  },
  '.cm-activeLineGutter': {
    backgroundColor: '$backgroundHighlighted',
  },
});
