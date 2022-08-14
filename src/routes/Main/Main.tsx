import { CategoryTabs } from '../../components/CategoryTabs';
import { NoteEditor } from '../../components/NoteEditor';
import { ViewDateNavigator } from '../../components/ViewDateNavigator';
import { styled } from '../../styles';

export function Main() {
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
      <Wrapper>
        <CategoryTabs.Content>{category => <NoteEditor category={category} />}</CategoryTabs.Content>
      </Wrapper>
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

const Wrapper = styled('main', {
  flex: 1,
  overflowY: 'auto',
});
