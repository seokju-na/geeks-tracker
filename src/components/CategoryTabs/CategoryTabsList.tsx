import { PlusIcon } from '@radix-ui/react-icons';
import { CSS } from '@stitches/react';
import { Suspense } from 'react';
import { useCategories } from '../../hooks/useCategories';
import { FlatButton } from '../FlatButton';
import { FlatTabs } from '../FlatTabs';

interface Props {
  css?: CSS;
}

export function CategoryTabsList(props: Props) {
  return (
    <Suspense fallback={<Fallback {...props} />}>
      <Resolved {...props} />
    </Suspense>
  );
}

function Fallback(props: Props) {
  return <FlatTabs.List aria-label="Categories" {...props} />;
}

function Resolved(props: Props) {
  const { categories } = useCategories();

  return (
    <>
      <FlatTabs.List aria-label="Categories" {...props}>
        {categories.map(category => (
          <FlatTabs.Trigger key={category.id} value={category.id}>
            {category.title}
          </FlatTabs.Trigger>
        ))}
      </FlatTabs.List>
      <FlatButton>
        <PlusIcon />
        Add Category
      </FlatButton>
    </>
  );
}
