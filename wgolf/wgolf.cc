#include <vector>
#include <set>
#include <map>
#include <algorithm>
#include <iostream>
#include <fstream>
#include <numeric>
#include <cctype>
#include <climits>
using namespace std;
char ALPHABET[] = "abcdefghijklmnopqrstuvwxyz";
vector<string> get_neighbors(vector<string> &dict, string &src) {
    // neighbors are words whose letters differ alphabetically by one at a time
    // head -> dead, heal, etc
    vector<string> neighbors;
    for (int i = 0; i < src.size(); ++i) {
        for (auto letter : ALPHABET) {
            string swapped = src;
            swapped[i] = letter;
            if (find(dict.begin(), dict.end(), swapped) != end(dict) && swapped != src) {
                neighbors.push_back(swapped);
            }
        }
    }
    return neighbors;
}

bool visited(const vector<pair<string, string>> &v, string &s) {
    return any_of(begin(v), end(v), [&](auto &pair){
        auto [word, next] = pair;
        return s == next;
    });
}

void wgolf(map<string, set<string>> &links, vector<string> &dict, string &cur, string &dst) {
    if (cur == dst) return;
    for (auto &next : get_neighbors(dict, cur)) {
        if (links[cur])
        links[cur].insert(next);
        wgolf(links, dict, next, dst);
    }
}

vector<string> load_dict(int wordlen) {
    char path[] = "/usr/share/dict/words";
    ifstream file (path);
    if (!file.is_open()) {
        cout << "Missing '" << path << "'\n";
        exit(-1);
    }
    string line;
    vector<string> dict;
    while (getline(file, line)) {
        if (line.size() == wordlen) {
            line[0] = tolower(line[0]);
            dict.push_back(line);
        }
    }
    return dict;
}

int main() {
    map<string, set<string>> links;
    auto dict = load_dict(4);
    cout << dict[0] << "\n";
    string src = "warm";
    string dst = "cold";
    // test_get_neighbors(dict);
    wgolf(links, dict, src, dst);
    cout << "finished steps\n";
    int max = INT_MIN;
    int min = INT_MAX;
    for (auto [w, v] : links) {
        if (v.size() > max) max = v.size();
        if (v.size() < min) min = v.size();
    }
    printf("most steps: %d\n", max);
    printf("least steps: %d\n", min);
}
