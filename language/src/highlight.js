import { styleTags, tags } from '@lezer/highlight';

export const highlighting = styleTags({
  new: tags.keyword,
  set: tags.keyword,
  delete: tags.keyword,
  title: tags.keyword,
  status: tags.keyword,
  Number: tags.number,
  String: tags.string,
  TaskId: tags.name,
  TaskStatus: tags.literal,
  Duration: tags.literal,
});
