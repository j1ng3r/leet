// s is the string, r is the regex
// the regex can be split into "chunks", which are either a chararcter match or a star match.
bool isMatch(char* s, char* r) {
    // bitboard representing which chunks are "lit up"
    unsigned int bitlit = 0;
    char b = 0;
    char i = 0;
    // Initialize bitboard
    while (r[i] != 0) {
        bitlit |= 1 << b;
        if (r[i + 1] == '*') {
            i += 2;
            b ++;
        } else {
            break;
        }
    }

    for (char j = 0; s[j] != 0; j ++) {
        // For every new character, keep track of which chunks of the regex are now lit up, based on the previous state
        unsigned int newbitlit = 0;
        b = 0;
        for (i = 0; r[i] != 0;) {
            if (r[i + 1] == '*') {
                // Star chunk
                if (bitlit & (1 << b)) {
                    // If this particular chunk is lit up
                    if (r[i] == '.' || r[i] == s[j]) {
                        // Make sure to re-light the star chunk we're at
                        // And light the next one and so on until we either hit the end or a character chunk
                        newbitlit |= 1 << b;
                        do {
                            b ++;
                            i += 2;
                            newbitlit |= 1 << b;
                        } while (r[i] != 0 && r[i + 1] == '*');
                    } else {
                        b ++;
                        i += 2;
                    }
                } else {
                    b ++;
                    i += 2;
                }
            } else {
                // Character chunk
                if (bitlit & (1 << b)) {
                    // If this particular chunk is lit up
                    if (r[i] == '.' || r[i] == s[j]) {
                        // Make sure to not re-light the character chunk we're at
                        // But light the next one and so on until we either hit the end or a character chunk
                        b ++;
                        i += 1;
                        newbitlit |= 1 << b;
                        while (r[i] != 0 && r[i + 1] == '*') {
                            b ++;
                            i += 2;
                            newbitlit |= 1 << b;
                        }
                    } else {
                        b ++;
                        i += 1;
                    }
                } else {
                    b ++;
                    i += 1;
                }
            }
        }

        // If no chunks are lit up, exit early
        if (newbitlit == 0) {
            return false;
        }
        bitlit = newbitlit;
    }

    // Shift down to the last bit, representing successful termination of the regex
    i = 0;
    do {
        bitlit >>= 1;
        if (r[i + 1] == '*') {
            i += 2;
        } else {
            i += 1;
        }
    } while (r[i] != 0);
    return (bool) bitlit;
}
