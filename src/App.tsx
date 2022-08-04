import { CategoryTabs } from './components/CategoryTabs';
import { NoteEditor } from './components/NoteEditor';
import { ViewDateNavigator } from './components/ViewDateNavigator';
import { useGlobalEscKeydown } from './hooks/useGlobalEscKeydown';
import { styled } from './styles';

export default function App() {
  useGlobalEscKeydown();

  return (
    <CategoryTabs.Root
      css={{
        display: 'flex',
        flexDirection: 'column',
        width: '100%',
        height: '100%',
      }}
    >
      <Header data-tauri-drag-region>
        <CategoryTabs.List />
      </Header>
      <ViewDateNavigator />
      <Main>
        <CategoryTabs.Content>
          <NoteEditor />
        </CategoryTabs.Content>
      </Main>
    </CategoryTabs.Root>
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
