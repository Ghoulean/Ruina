import {
    DecoratedAbnoPage,
    DecoratedCombatPage,
    DecoratedKeyPage,
    DecoratedPassive,
    Localization,
    LookupResult,
} from "@ghoulean/ruina-common";
import * as fastestLevenshtein from "fastest-levenshtein";
import { default as __DISAMBIGUATION_RESULTS } from "../data/ambiguousResults.json";
import { default as __AUTOCOMPLETE } from "../data/autocomplete.json";
import { default as __CN_ABNO } from "../data/cn/abno.json";
import { default as __CN_COMBAT } from "../data/cn/combat.json";
import { default as __CN_KEYPAGES } from "../data/cn/keypages.json";
import { default as __CN_PASSIVE } from "../data/cn/passive.json";
import { default as __EN_ABNO } from "../data/en/abno.json";
import { default as __EN_COMBAT } from "../data/en/combat.json";
import { default as __EN_KEYPAGES } from "../data/en/keypages.json";
import { default as __EN_PASSIVE } from "../data/en/passive.json";
import { default as __JP_ABNO } from "../data/jp/abno.json";
import { default as __JP_COMBAT } from "../data/jp/combat.json";
import { default as __JP_KEYPAGES } from "../data/jp/keypages.json";
import { default as __JP_PASSIVE } from "../data/jp/passive.json";
import { default as __KR_ABNO } from "../data/kr/abno.json";
import { default as __KR_COMBAT } from "../data/kr/combat.json";
import { default as __KR_KEYPAGES } from "../data/kr/keypages.json";
import { default as __KR_PASSIVE } from "../data/kr/passive.json";
import { default as __LOOKUP_RESULTS } from "../data/queryLookupResults.json";
import { default as __TRCN_ABNO } from "../data/trcn/abno.json";
import { default as __TRCN_COMBAT } from "../data/trcn/combat.json";
import { default as __TRCN_KEYPAGES } from "../data/trcn/keypages.json";
import { default as __TRCN_PASSIVE } from "../data/trcn/passive.json";
import { DisambiguationResults } from "../model/disambiguation_result";

type QueryToLookupResult = {
    [key: string]: LookupResult[];
};

type QueryToDecoratedAbnoPage = {
    [key: string]: DecoratedAbnoPage;
};

type LocalizationToQueryDecoratedAbnoPage = {
    [key in Localization]: QueryToDecoratedAbnoPage;
};

type QueryToDecoratedCombatPage = {
    [key: string]: DecoratedCombatPage;
};

type LocalizationToQueryDecoratedCombatPage = {
    [key in Localization]: QueryToDecoratedCombatPage;
};

type QueryToDecoratedKeyPage = {
    [key: string]: DecoratedKeyPage;
};

type LocalizationToQueryDecoratedKeyPage = {
    [key in Localization]: QueryToDecoratedKeyPage;
};

type QueryToDecoratedPassive = {
    [key: string]: DecoratedPassive;
};

type LocalizationToQueryDecoratedPassive = {
    [key in Localization]: QueryToDecoratedPassive;
};

type LevenshteinResults = {
    best: string;
    score: number;
};

const LOOKUP_RESULTS: QueryToLookupResult =
    __LOOKUP_RESULTS as QueryToLookupResult;

const CN_ABNO: QueryToDecoratedAbnoPage = __CN_ABNO as QueryToDecoratedAbnoPage;
const EN_ABNO: QueryToDecoratedAbnoPage = __EN_ABNO as QueryToDecoratedAbnoPage;
const JP_ABNO: QueryToDecoratedAbnoPage = __JP_ABNO as QueryToDecoratedAbnoPage;
const KR_ABNO: QueryToDecoratedAbnoPage = __KR_ABNO as QueryToDecoratedAbnoPage;
const TRCN_ABNO: QueryToDecoratedAbnoPage =
    __TRCN_ABNO as QueryToDecoratedAbnoPage;

const CN_COMBAT: QueryToDecoratedCombatPage =
    __CN_COMBAT as QueryToDecoratedCombatPage;
const EN_COMBAT: QueryToDecoratedCombatPage =
    __EN_COMBAT as QueryToDecoratedCombatPage;
const JP_COMBAT: QueryToDecoratedCombatPage =
    __JP_COMBAT as QueryToDecoratedCombatPage;
const KR_COMBAT: QueryToDecoratedCombatPage =
    __KR_COMBAT as QueryToDecoratedCombatPage;
const TRCN_COMBAT: QueryToDecoratedCombatPage =
    __TRCN_COMBAT as QueryToDecoratedCombatPage;

const CN_KEYPAGES: QueryToDecoratedKeyPage =
    __CN_KEYPAGES as QueryToDecoratedKeyPage;
const EN_KEYPAGES: QueryToDecoratedKeyPage =
    __EN_KEYPAGES as QueryToDecoratedKeyPage;
const JP_KEYPAGES: QueryToDecoratedKeyPage =
    __JP_KEYPAGES as QueryToDecoratedKeyPage;
const KR_KEYPAGES: QueryToDecoratedKeyPage =
    __KR_KEYPAGES as QueryToDecoratedKeyPage;
const TRCN_KEYPAGES: QueryToDecoratedKeyPage =
    __TRCN_KEYPAGES as QueryToDecoratedKeyPage;

const CN_PASSIVE: QueryToDecoratedPassive =
    __CN_PASSIVE as QueryToDecoratedPassive;
const EN_PASSIVE: QueryToDecoratedPassive =
    __EN_PASSIVE as QueryToDecoratedPassive;
const JP_PASSIVE: QueryToDecoratedPassive =
    __JP_PASSIVE as QueryToDecoratedPassive;
const KR_PASSIVE: QueryToDecoratedPassive =
    __KR_PASSIVE as QueryToDecoratedPassive;
const TRCN_PASSIVE: QueryToDecoratedPassive =
    __TRCN_PASSIVE as QueryToDecoratedPassive;

const LOCALIZATION_TO_DECORATED_ABNO_PAGE: LocalizationToQueryDecoratedAbnoPage =
    {
        [Localization.CHINESE_SIMPLIFIED]: CN_ABNO,
        [Localization.CHINESE_TRADITIONAL]: TRCN_ABNO,
        [Localization.ENGLISH]: EN_ABNO,
        [Localization.JAPANESE]: JP_ABNO,
        [Localization.KOREAN]: KR_ABNO,
    };

const LOCALIZATION_TO_DECORATED_COMBAT_PAGE: LocalizationToQueryDecoratedCombatPage =
    {
        [Localization.CHINESE_SIMPLIFIED]: CN_COMBAT,
        [Localization.CHINESE_TRADITIONAL]: TRCN_COMBAT,
        [Localization.ENGLISH]: EN_COMBAT,
        [Localization.JAPANESE]: JP_COMBAT,
        [Localization.KOREAN]: KR_COMBAT,
    };

const LOCALIZATION_TO_DECORATED_KEY_PAGE: LocalizationToQueryDecoratedKeyPage =
    {
        [Localization.CHINESE_SIMPLIFIED]: CN_KEYPAGES,
        [Localization.CHINESE_TRADITIONAL]: TRCN_KEYPAGES,
        [Localization.ENGLISH]: EN_KEYPAGES,
        [Localization.JAPANESE]: JP_KEYPAGES,
        [Localization.KOREAN]: KR_KEYPAGES,
    };

const LOCALIZATION_TO_DECORATED_PASSIVE: LocalizationToQueryDecoratedPassive = {
    [Localization.CHINESE_SIMPLIFIED]: CN_PASSIVE,
    [Localization.CHINESE_TRADITIONAL]: TRCN_PASSIVE,
    [Localization.ENGLISH]: EN_PASSIVE,
    [Localization.JAPANESE]: JP_PASSIVE,
    [Localization.KOREAN]: KR_PASSIVE,
};

const AUTOCOMPLETE: string[] = __AUTOCOMPLETE.data;
const DISAMBIGUATION_RESULTS: { [key: string]: DisambiguationResults } =
    __DISAMBIGUATION_RESULTS as { [key: string]: DisambiguationResults };

const FUZZY_MATCHING_DISTANCE = 2;

export class DataAccessor {
    constructor() {}

    public lookup(query: string, preferredLocale: Localization): LookupResult {
        let lookupResults: LookupResult[] | undefined = LOOKUP_RESULTS[query];
        if (!lookupResults) {
            const levenshteinResults: LevenshteinResults =
                this.levenshteinLookup(query);
            if (levenshteinResults.score > FUZZY_MATCHING_DISTANCE) {
                throw new Error(`Couldn't identify query result for ${query}.`);
            }
            lookupResults = LOOKUP_RESULTS[levenshteinResults.best];
        }
        for (const lookupResult of lookupResults) {
            if (lookupResult.locale == preferredLocale) {
                return lookupResult;
            }
        }
        return lookupResults[0];
    }

    public getDecoratedAbnoPage(
        pageId: string,
        locale: Localization
    ): DecoratedAbnoPage {
        const decoratedAbnoPage: DecoratedAbnoPage | undefined =
            this.getLocaledAbnoMapping(locale)[pageId];
        if (!decoratedAbnoPage) {
            throw new Error(
                `Abno page ${pageId} not found in ${locale} locale`
            );
        }
        return decoratedAbnoPage;
    }

    public getDecoratedCombatPage(
        pageId: string,
        locale: Localization
    ): DecoratedCombatPage {
        const decoratedCombatPage: DecoratedCombatPage | undefined =
            this.getLocaledCombatMapping(locale)[pageId];
        if (!decoratedCombatPage) {
            throw new Error(
                `Combat page ${pageId} not found in ${locale} locale`
            );
        }
        return decoratedCombatPage;
    }

    public getDecoratedKeyPage(
        pageId: string,
        locale: Localization
    ): DecoratedKeyPage {
        const decoratedKeyPage: DecoratedKeyPage | undefined =
            this.getLocaledKeyPageMapping(locale)[pageId];
        if (!decoratedKeyPage) {
            throw new Error(`Key page ${pageId} not found in ${locale} locale`);
        }
        return decoratedKeyPage;
    }

    public getDecoratedPassive(
        pageId: string,
        locale: Localization
    ): DecoratedPassive {
        const decoratedPassive: DecoratedPassive | undefined =
            this.getLocaledPassiveMapping(locale)[pageId];
        if (!decoratedPassive) {
            throw new Error(`Passive ${pageId} not found in ${locale} locale`);
        }
        return decoratedPassive;
    }

    public getDisambiguationResult(pageId: string): DisambiguationResults {
        const disambiguation: DisambiguationResults | undefined =
            DISAMBIGUATION_RESULTS[pageId];
        if (!disambiguation) {
            throw new Error(`Disambiguation ${pageId} not found`);
        }
        return disambiguation;
    }

    public autocomplete(query: string): string[] {
        const retVal: string[] = [];
        const cleanQuery = this.cleanQuery(query);
        for (const q of AUTOCOMPLETE) {
            if (
                this.autocompleteDistance(cleanQuery, q) <=
                FUZZY_MATCHING_DISTANCE
            ) {
                retVal.push(q);
            }
        }
        return retVal.sort((a, b) => {
            return (
                this.autocompleteDistance(cleanQuery, a) -
                    this.autocompleteDistance(cleanQuery, b) ||
                a.length - b.length ||
                a.localeCompare(b)
            );
        });
    }

    private getLocaledAbnoMapping(
        locale: Localization
    ): QueryToDecoratedAbnoPage {
        return LOCALIZATION_TO_DECORATED_ABNO_PAGE[locale];
    }

    private getLocaledCombatMapping(
        locale: Localization
    ): QueryToDecoratedCombatPage {
        return LOCALIZATION_TO_DECORATED_COMBAT_PAGE[locale];
    }

    private getLocaledKeyPageMapping(
        locale: Localization
    ): QueryToDecoratedKeyPage {
        return LOCALIZATION_TO_DECORATED_KEY_PAGE[locale];
    }

    private getLocaledPassiveMapping(
        locale: Localization
    ): QueryToDecoratedPassive {
        return LOCALIZATION_TO_DECORATED_PASSIVE[locale];
    }

    private cleanQuery(s: string): string {
        return s.replace(/\s/g, "").toLowerCase();
    }

    private autocompleteDistance(query: string, lookup: string): number {
        return fastestLevenshtein.distance(
            query,
            this.cleanQuery(lookup).substring(0, query.length)
        );
    }

    private levenshteinLookup(s: string): LevenshteinResults {
        const closest: string = fastestLevenshtein.closest(s, AUTOCOMPLETE);
        return {
            best: closest,
            score: fastestLevenshtein.distance(
                this.cleanQuery(s),
                this.cleanQuery(closest)
            ),
        };
    }
}
