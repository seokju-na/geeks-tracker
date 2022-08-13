export type NoteCommandName = 'NoteCommand.CreateOrUpdate';

interface Base {
  name: NoteCommandName;
}

interface CreateOrUpdateCommand extends Base {
  name: 'NoteCommand.CreateOrUpdate';
  id: string;
  categoryId: string;
  body: string;
}

export type NoteCommand = CreateOrUpdateCommand;
export type NoteCommandOf<T extends NoteCommandName> = Extract<NoteCommand, { name: T }>;

export function isNoteCommand(command: unknown): command is NoteCommand {
  return command != null && (command as NoteCommand)?.name?.startsWith('NoteCommand');
}
