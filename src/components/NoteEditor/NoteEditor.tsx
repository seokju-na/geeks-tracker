import { useObservableCallback, useSubscription } from 'observable-hooks';
import { Suspense } from 'react';
import { concatMap, debounceTime, from } from 'rxjs';
import { useNote } from '../../hooks/useNote';
import { useNoteOperations } from '../../hooks/useNoteOperations';
import { useViewDate } from '../../hooks/useViewDate';
import { Category } from '../../types/Category';
import { makeNoteId } from '../../types/Note';
import { Editor } from '../Editor';

interface Props {
  category: Category;
}

export function NoteEditor(props: Props) {
  return (
    <Suspense fallback={null}>
      <Resolved {...props} />
    </Suspense>
  );
}

function Resolved({ category }: Props) {
  const { viewDate } = useViewDate();
  const noteId = makeNoteId({ category, date: viewDate });
  const { note } = useNote(noteId);
  const { createOrUpdateNote } = useNoteOperations();

  const [handleChange, changes$] = useObservableCallback<void, string>(input$ =>
    input$.pipe(
      debounceTime(200),
      concatMap(text =>
        from(
          createOrUpdateNote({
            id: noteId,
            categoryId: category.id,
            body: text,
          })
        )
      )
    )
  );
  useSubscription(changes$);

  return <Editor initialDoc={note?.body ?? category.template} onChange={handleChange} />;
}
