export type CategoryCommandName =
  | 'CategoryCommand.Create'
  | 'CategoryCommand.UpdateTitle'
  | 'CategoryCommand.UpdateTemplate';

interface Base {
  name: CategoryCommandName;
}

interface CreateCommand extends Base {
  name: 'CategoryCommand.Create';
  id: string;
  title: string;
  template: string;
}

interface UpdateTitleCommand extends Base {
  name: 'CategoryCommand.UpdateTitle';
  id: string;
  title: string;
}

interface UpdateTemplateCommand extends Base {
  name: 'CategoryCommand.UpdateTemplate';
  id: string;
  template: string;
}

export type CategoryCommand = CreateCommand | UpdateTitleCommand | UpdateTemplateCommand;
export type CategoryCommandOf<T extends CategoryCommandName> = Extract<CategoryCommand, { name: T }>;

export function isCategoryCommand(command: unknown): command is CategoryCommand {
  return command != null && (command as CategoryCommand)?.name?.startsWith('CategoryCommand');
}
