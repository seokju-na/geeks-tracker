import { useEffect, useState } from 'react';
import { app } from '@tauri-apps/api';

export function Titlebar() {
  const [version, setVersion] = useState<string>();
  useEffect(() => {
    app
      .getVersion()
      .then(v => `v${v}`)
      .then(setVersion);
  }, []);
  return (
    <div
      data-tauri-drag-region
      className="flex items-center justify-between h-6 px-2 bg-zinc-800 text-white text-xs font-me border-b border-b-zinc-700 cursor-default select-none"
    >
      <b>geeks-tracker</b>
      <span>{version}</span>
    </div>
  );
}
