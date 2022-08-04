import { ChevronLeftIcon, ChevronRightIcon } from '@radix-ui/react-icons';
import { format } from 'date-fns';
import { Suspense } from 'react';
import { useSettings } from '../../hooks/useSettings';
import { useViewDate } from '../../hooks/useViewDate';
import { styled } from '../../styles';
import { Accelerator } from '../Accelerator';
import { FlatButton } from '../FlatButton';
import { FlatIconButton } from '../FlatIconButton';

export function ViewDateNavigator() {
  return (
    <Suspense fallback={<Fallback />}>
      <Resolved />
    </Suspense>
  );
}

function Resolved() {
  const { viewDate } = useViewDate();
  const {
    settings: { viewDateFormat },
  } = useSettings('appearance');

  return (
    <Nav>
      <Title>{format(viewDate, viewDateFormat)}</Title>
      <Buttons>
        <Accelerator shortcut={['CmdOrCtrl', 'Shift', '[']}>
          <FlatIconButton aria-label="Previous date">
            <ChevronLeftIcon />
          </FlatIconButton>
        </Accelerator>
        <FlatButton>Today</FlatButton>
        <Accelerator shortcut={['CmdOrCtrl', 'Shift', ']']}>
          <FlatIconButton aria-label="Next date">
            <ChevronRightIcon />
          </FlatIconButton>
        </Accelerator>
      </Buttons>
    </Nav>
  );
}

function Fallback() {
  return <Nav />;
}

const Nav = styled('nav', {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  height: 29,
  borderBottom: '1px solid $divider',
});

const Title = styled('h1', {
  margin: 0,
  padding: '0 $lg',
  fontSize: '$md',
  fontWeight: '$semibold',
  color: '$text',
});

const Buttons = styled('div', {
  display: 'flex',
  alignItems: 'center',
  borderLeft: '1px solid $divider',
  '& > button + button': {
    borderLeft: '1px solid $divider',
  },
});
