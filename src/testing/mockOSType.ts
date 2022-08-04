import { vitest } from 'vitest';
import { OSType } from '../utils/osType';

export function mockOSType(os: OSType) {
  vitest.stubGlobal('__TAURI_OS_TYPE__', os);
}
