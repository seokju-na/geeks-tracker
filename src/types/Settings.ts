export interface AllSettings {
  appearance: {
    viewDateFormat: string;
  };
}

export type SettingsName = keyof AllSettings;
export type Settings<Name extends SettingsName> = AllSettings[Name];

export const defaultAllSettings: AllSettings = {
  appearance: {
    viewDateFormat: 'yyyy-MM-dd',
  },
};
