import { useQuery } from 'react-query';
import { queryClient } from '../queryClient';
import { getNote } from '../remotes/getNote';
import { Note } from '../types/Note';

const queryKey = (id: string) => ['notes', id];

export function useNote(id: string) {
  const { data: note } = useQuery(queryKey(id), () => getNote(id));

  return { note: note as Note | null };
}

useNote.queryKey = queryKey;
useNote.invalidate = (id: string) => {
  return queryClient.invalidateQueries(queryKey(id));
};
