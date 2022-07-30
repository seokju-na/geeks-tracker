import * as Primitives from '@radix-ui/react-tabs';
import { styled } from '../../styles';

const Root = styled(Primitives.Root, {});

const List = styled(Primitives.List, {
  display: 'flex',
});

const Trigger = styled(Primitives.Trigger, {
  all: 'unset',
  backgroundColor: '$background',
  height: 28,
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  padding: '0 $xl',
  fontSize: '$md',
  color: '$text',
  userSelect: 'none',
  cursor: 'pointer',
  '& + &': {
    borderLeft: '1px solid $divider',
  },
  '&[aria-selected="true"]': {
    backgroundColor: '$backgroundHighlighted',
  },
  '&:focus': {
    backgroundColor: '$blue600',
  },
});

const Content = styled(Primitives.Content, {
  outline: 'none',
});

export const FlatTabs = {
  Root,
  List,
  Trigger,
  Content,
};
