#include <iostream>
#include <unordered_set>

using namespace std;

int main()
{
    unordered_set<int> set;
    for (int i = 0; i < 200; ++i) {
        int a;
        cin >> a;
        set.insert(a);
    }
    for (const auto& a: set) {
        for (const auto& b: set) {
            if (a == b) continue;
            const auto c = 2020-a-b;
            if (a == c || b == c) continue;
            if (set.count(c)) {
                cout << a * b * c << ' ' << a << ' ' << b << ' ' << c << '\n';
                return 0;
            }
        }
    }
    // while (true) {
    //     int a;
    //     cin >> a;
    //     if (set.count(2020-a)) {
    //         cout << a * (2020 -a) << ' ' << a << 2020-a << '\n';
    //         return 0;
    //     }
    //     set.insert(a);
    // }
    return 0;
}
