import { Any } from './types';

export type OSType = 'linux' | 'windows' | 'macos';

export function getOSType() {
  return (window as Any).__TAURI_OS_TYPE__ as OSType;
}
