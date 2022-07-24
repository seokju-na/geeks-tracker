import { GearIcon, MagnifyingGlassIcon } from '@radix-ui/react-icons';
import { FlatTabs } from './components/FlatTabs';
import { styled } from './styles';

export default function App() {
  return (
    <FlatTabs.Root defaultValue="Tasks" css={{ width: '100vw', height: '100vh' }}>
      <Header>
        <FlatTabs.List aria-label="Categories">
          <FlatTabs.Trigger value="Tasks">Tasks</FlatTabs.Trigger>
          <FlatTabs.Trigger value="Idea">Idea</FlatTabs.Trigger>
          <FlatTabs.Trigger value="Notes">Notes</FlatTabs.Trigger>
        </FlatTabs.List>
        <Buttons>
          <IconButton aria-label="Search (⌘+F)">
            <MagnifyingGlassIcon />
          </IconButton>
          <IconButton aria-label="Preferences (⌘+,)">
            <GearIcon />
          </IconButton>
        </Buttons>
      </Header>
      <main>
        <FlatTabs.Content value="Tasks">Tasks</FlatTabs.Content>
        <FlatTabs.Content value="Idea">Idea</FlatTabs.Content>
        <FlatTabs.Content value="Notes">Notes</FlatTabs.Content>
      </main>
    </FlatTabs.Root>
  );
}

const Header = styled('header', {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  width: '100%',
  borderBottom: '1px solid $divider',
});

const Buttons = styled('div', {
  display: 'flex',
  alignItems: 'center',
  borderLeft: '1px solid $divider',
});

const IconButton = styled('button', {
  all: 'unset',
  display: 'inline-flex',
  alignItems: 'center',
  justifyContent: 'center',
  height: 28,
  padding: '0 $md',
  color: '$icon',
  '& + &': {
    borderLeft: '1px solid $divider',
  },
});
