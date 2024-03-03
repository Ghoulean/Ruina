# Ruina Reparser

Hack scripts to convert Library of Ruina XML files into an easily digestable format. ~~These scripts are one-off. Don't expect good code here.~~

EDIT: I expected this to be 1-off but considering how important data parsing is, I think working on this package is actually a long-term thing now. So eventually I will need to completely refactor this, since my current adhoc abstractions are showing its weaknesses. Also, both "data parsing" and "Ruina Reparser" are misnomers at this point.

First: set up Node 18. Not tested with other versions of node. You can use Node version manager.

1. Download XMLs from BaseMod
2. Put BaseMod folder into root
3. Build `@ghoulean/ruina-common` (should be in the same repo)
4. Build this package via `npm run build`

### Scratch paper

There are three things that need to be represented:

1. **Game objects** - The entities of Library of Ruina. Examples are: combat pages, key pages, abno pages, and passives.
2. **Localizations** - Descriptions of game object in a given locale
3. **Query mappings** - Determining the referenced game object in a specific localization given an unlabeled query string

(1) and (2) are easy. They're literally just pulling the relevant data from the XMLs. It's 3 that needs a lot of work.

Some basic guidelines for (1) and (2):

- **Be a reference, not a guide**: Do not add annotations, notes, tips, or any other information not included in the original game. This includes bugs such as Eternal Rest taking ally HP into account.
- **Despaghetti over faithfulness**: Ruina data models should be remodeled into an ideal state. Spaghetti on how some game objects in Library of Ruina need to be smoothed over. To illustrate, 333...1973 and ranged collectable pages before SotC relies on the `Options` tag to exhaust, while Field Mods does not.

Features I didn't know I wanted for query mappings until way too late:

- **Combine Lookups**: On lookup, silently combine closely-related game objects when reasonable to do so (e.g. Shimmering)
- **Reasonable Defaults**: Add reasonable defaults where combination isn't appropriate but one particular entry still stands out (e.g. Weight of Sin; Xiao (passive) + Xiao's Page)
- **Disambiguate**: When neither combination nor reasonable defaults are appropriate on lookup, map to an explicit disambiguation page instead (e.g. Coffin)
- **Aliasing**: Permit adding aliases to lookups (e.g. Distorted Yan's Page)
Da