import { useEditor } from '../../hooks/useEditor';

interface Props {
  initialDoc?: string;
}

export function Editor(props: Props) {
  const [elemRef] = useEditor<HTMLDivElement>(props);

  return <div ref={elemRef} />;
}
