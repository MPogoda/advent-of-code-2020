#include <iostream>
#include <string>

using namespace std;
int main()
{
    auto count = 0;
    auto currentPos = 0;
    for (auto t = 0; t < 323 / 2; ++t) {
        string row;
        cin >> row;

        count += row[currentPos] == '#';
        currentPos = (currentPos + 1) % row.length();
        cin >> row;
    }

    cout << count << '\n';

    return 0;
}
