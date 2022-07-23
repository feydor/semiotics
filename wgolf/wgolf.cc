#include <vector>
#include <algorithm>
#include <iostream>
#include <fstream>
#include <numeric>
#include <cctype>
using namespace std;
char ALPHABET[] = "abcdefghijklmnopqrstuvwxyz";
vector<pair<string, string>> get_neighbors(vector<string> &dict, string &src) {
    // neighbors are words whose letters differ alphabetically by one at a time
    // head -> dead, heal, etc
    vector<pair<string, string>> neighbors;
    for (int i = 0; i < src.size(); ++i) {
        for (auto letter : ALPHABET) {
            string swapped = src;
            swapped[i] = letter;
            if (find(dict.begin(), dict.end(), swapped) != end(dict) && swapped != src) {
                neighbors.push_back(make_pair(src, swapped));
            }
        }
    }
    return neighbors;
}

void test_get_neighbors(vector<string> &dict) {
    string src = "head";
    auto neighbors = get_neighbors(dict, src);
    for (auto [word, next] : neighbors) {
        cout << word << " -> " << next << "\n"; 
    }
}

bool visited(const vector<pair<string, string>> &v, string &s) {
    return any_of(begin(v), end(v), [&](auto &pair){
        auto [word, next] = pair;
        return s == next;
    });
}

int gsteps;
void wgolf(vector<pair<string, string>> &links, vector<string> &dict, string &prev, string &src, string &dst) {
    if (src == dst) {
        cout << prev << " :: " << src << " :: " << dst << "\n";
        return;
    }
    links.push_back(make_pair(prev, src));
    for (auto &[word, dword] : get_neighbors(dict, src)) {
        if (!visited(links, dword)) {
            gsteps++;
            wgolf(links, dict, word, dword, dst);
        }
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
    vector<pair<string, string>> links;
    auto dict = load_dict(4);
    cout << dict[0] << "\n";
    string src = "warm";
    string dst = "cold";
    // test_get_neighbors(dict);
    wgolf(links, dict, src, src, dst);
    cout << "finished steps\n";
    printf("gsteps: %d\n", gsteps);
}
