import { Suspense } from 'react';
import { useCategories } from '../../hooks/useCategories';
import { Editor } from '../Editor';

export function NoteEditor() {
  return (
    <Suspense fallback={null}>
      <Resolved />
    </Suspense>
  );
}

function Resolved() {
  const { selectedCategory } = useCategories();

  if (selectedCategory === undefined) {
    return null;
  }

  return <Editor initialDoc={selectedCategory.template} />;
}
