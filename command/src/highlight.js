import { styleTags, tags } from '@lezer/highlight';

export const geeksTrackerHighlight = styleTags({
  new: tags.keyword,
  set: tags.keyword,
  delete: tags.keyword,
  title: tags.keyword,
  status: tags.keyword,
  Number: tags.number,
  String: tags.string,
  TaskId: tags.literal,
  TaskStatus: tags.literal,
  Duration: tags.operatorKeyword,
});
