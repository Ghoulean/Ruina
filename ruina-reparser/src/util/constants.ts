import * as path from 'path';

export const PROJECT_ROOT_DIR = path.resolve(__dirname, "../../");

export const BASEMOD_DIR = path.resolve(PROJECT_ROOT_DIR, "BaseMod");
export const OUTPUT_DIR = path.resolve(PROJECT_ROOT_DIR, "out");

export const STATICINFO_DIR = path.resolve(OUTPUT_DIR, "StaticInfo");
export const LOCALIZE_DIR = path.resolve(OUTPUT_DIR, "Localize");
export const DATA_DIR = path.resolve(OUTPUT_DIR, "Data");

export const ABNO_DIR = path.resolve(STATICINFO_DIR, "EmotionCard");
export const COMBAT_PAGE_DIR = path.resolve(STATICINFO_DIR, "Card");
export const KEY_PAGE_DIR = path.resolve(STATICINFO_DIR, "EquipPage");
export const PASSIVE_DIR = path.resolve(STATICINFO_DIR, "PassiveList");

export const NO_IMAGE_IMAGE_PATH = "/no_image.png";