#include <iostream>
#include <algorithm>

using namespace std;

int main()
{
    unsigned good = 0;
    for (auto t = 0; t < 1000; ++t) {
        int min, max;
        char ch;
        string s;

        cin >> min;
        cin.get();
        cin >> max;
        cin.get();
        cin.get(ch);
        cin.get();
        cin >> s;

        const bool has_lhs = s.length() >= min && s[min - 1] == ch;
        const bool has_rhs = s.length() >= max && s[max - 1] == ch;

        if (has_lhs != has_rhs) ++good;
        // const auto counted = count(begin(s), end(s), ch);
        // if (counted >= min && counted <= max) ++good;
    }
    cout << good << '\n';
    return 0;
}
