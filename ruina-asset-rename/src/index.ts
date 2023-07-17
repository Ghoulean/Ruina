import * as fs from "fs";
import * as path from "path";
import * as _queryLookupResults from "./queryLookupResults.json";

const queryLookupResults = _queryLookupResults as {[key: string]: any};

function* walkSync(dir: string): Generator<string, any, unknown> {
    const files = fs.readdirSync(dir, { withFileTypes: true });
    for (const file of files) {
        if (file.isDirectory()) {
            yield* walkSync(path.join(dir, file.name));
        } else {
            yield path.join(dir, file.name);
        }
    }
}

function levenshtein(s: string, t: string): number {
    if (s === t) {
        return 0;
    }
    var n = s.length,
        m = t.length;
    if (n === 0 || m === 0) {
        return n + m;
    }
    var x = 0,
        y,
        a,
        b,
        c,
        d,
        g,
        h;
    var p = new Uint16Array(n);
    var u = new Uint32Array(n);
    for (y = 0; y < n; ) {
        u[y] = s.charCodeAt(y);
        p[y] = ++y;
    }

    for (; x + 3 < m; x += 4) {
        var e1 = t.charCodeAt(x);
        var e2 = t.charCodeAt(x + 1);
        var e3 = t.charCodeAt(x + 2);
        var e4 = t.charCodeAt(x + 3);
        c = x;
        b = x + 1;
        d = x + 2;
        g = x + 3;
        h = x + 4;
        for (y = 0; y < n; y++) {
            a = p[y];
            if (a < c || b < c) {
                c = a > b ? b + 1 : a + 1;
            } else {
                if (e1 !== u[y]) {
                    c++;
                }
            }

            if (c < b || d < b) {
                b = c > d ? d + 1 : c + 1;
            } else {
                if (e2 !== u[y]) {
                    b++;
                }
            }

            if (b < d || g < d) {
                d = b > g ? g + 1 : b + 1;
            } else {
                if (e3 !== u[y]) {
                    d++;
                }
            }

            if (d < g || h < g) {
                g = d > h ? h + 1 : d + 1;
            } else {
                if (e4 !== u[y]) {
                    g++;
                }
            }
            p[y] = h = g;
            g = d;
            d = b;
            b = c;
            c = a;
        }
    }

    for (; x < m; ) {
        var e = t.charCodeAt(x);
        c = x;
        d = ++x;
        for (y = 0; y < n; y++) {
            a = p[y];
            if (a < c || d < c) {
                d = a > d ? d + 1 : a + 1;
            } else {
                if (e !== u[y]) {
                    d = c + 1;
                } else {
                    d = c;
                }
            }
            p[y] = d;
            c = a;
        }
        h = d;
    }

    return h!;
}

const usedNames: { [key: string]: string } = {};

for (const filePath of walkSync("./Pages")) {
    // Ignore non-abno pages for now -- generate mapping later
    if (filePath.includes("EGO Pages") || filePath.includes("Combat Pages")) {
        continue;
    }
    // Ignore nonobtainable
    if (filePath.includes("Unused") || filePath.includes("Guests")) {
        continue;
    }

    const baseName: string = path.basename(filePath, ".png");
    let bestResult: string;
    let bestScore = 9999;
    for (const queryLookup of Object.keys(queryLookupResults)) {
        const score = levenshtein(baseName, queryLookup);
        if (score < bestScore) {
            bestResult = queryLookup;
            bestScore = score;
        }
    }
    if (bestScore >= 3) {
        console.log(`Rejecting ${filePath} due to poor scoring`);
        continue;
    }

    const bestResultId: string = queryLookupResults[bestResult!]!.pageId!;

    if (usedNames[bestResultId]) {
        console.log(
            `Duplicate spotted: (new filename) ${bestResultId}.png already used by (old filename) ${
                usedNames[bestResultId]
            }.png`
        );
    }
    // console.log(`Mapping (old filename) ${baseName}.png to (new filename) ${bestResultId}.png`);
    if (bestScore > 0) {
        console.log(
            `Mapping (old filename) ${baseName}.png to (new filename) ${bestResultId}.png. Score is greater than 0!`
        );
    }
    usedNames[bestResultId] = baseName;


    fs.copyFileSync(
        path.resolve(__dirname, "../", filePath),
        path.resolve(__dirname, "../out", `${bestResultId}.png`)
    );
}
