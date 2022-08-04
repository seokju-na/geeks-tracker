import { GearIcon, MagnifyingGlassIcon } from '@radix-ui/react-icons';
import { Accelerator } from './components/Accelerator';
import { Editor } from './components/Editor';
import { FlatIconButton } from './components/FlatIconButton';
import { FlatTabs } from './components/FlatTabs';
import { useGlobalEscKeydown } from './hooks/useGlobalEscKeydown';
import { styled } from './styles';

export default function App() {
  useGlobalEscKeydown();

  return (
    <FlatTabs.Root
      defaultValue="Tasks"
      css={{
        display: 'flex',
        flexDirection: 'column',
        width: '100%',
        height: '100%',
      }}
    >
      <Header data-tauri-drag-region>
        <FlatTabs.List aria-label="Categories">
          <FlatTabs.Trigger value="Tasks">Tasks</FlatTabs.Trigger>
          <FlatTabs.Trigger value="Idea">Idea</FlatTabs.Trigger>
          <FlatTabs.Trigger value="Notes">Notes</FlatTabs.Trigger>
        </FlatTabs.List>
        <Buttons>
          <Accelerator shortcut={['CmdOrCtrl', 'F']}>
            <FlatIconButton aria-label="Search">
              <MagnifyingGlassIcon />
            </FlatIconButton>
          </Accelerator>
          <Accelerator shortcut={['CmdOrCtrl', ',']}>
            <FlatIconButton aria-label="Preferences">
              <GearIcon />
            </FlatIconButton>
          </Accelerator>
        </Buttons>
      </Header>
      <Main>
        <FlatTabs.Content value="Tasks">
          <Editor initialDoc="Tasks" />
        </FlatTabs.Content>
        <FlatTabs.Content value="Idea">
          <Editor initialDoc="Idea" />
        </FlatTabs.Content>
        <FlatTabs.Content value="Notes">
          <Editor initialDoc="Notes" />
        </FlatTabs.Content>
      </Main>
    </FlatTabs.Root>
  );
}

const Header = styled('header', {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  width: '100%',
  borderBottom: '1px solid $divider',
  userSelect: 'none',
});

const Main = styled('main', {
  flex: 1,
  overflowY: 'auto',
});

const Buttons = styled('div', {
  display: 'flex',
  alignItems: 'center',
  borderLeft: '1px solid $divider',
  '> button + button': {
    borderLeft: '1px solid $divider',
  },
});
