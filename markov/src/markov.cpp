#include <cstring> 
#include <fstream>
#include <iostream>
#include <set>
#include <sstream>
#include <random>
#include "markov.h"

auto split(const std::string &str, const std::string &delim) {
    std::vector<std::string> tokens;
    int start = 0;
    int end = str.find(delim);
    while (end != -1) {
        tokens.push_back(str.substr(start, end - start));
        start = end + delim.size();
        end = str.find(delim, start);
    }
    return tokens;
}

std::vector<bigram_t> parse_bigrams(const std::string &text) {
    std::vector<std::string> tokens = split(text, " ");
    std::vector<bigram_t> bigrams;

    auto first = tokens.begin();
    for (auto second = first + 1; second != tokens.end(); second++) {
        bigrams.push_back(std::make_pair(*first, *second));
        first++;
    }
    return bigrams;
}

auto get_random_matching_second(const std::vector<bigram_t> &bigrams, const std::string &first) {
    std::random_device dev;
    std::mt19937 rng(dev());

    std::vector<bigram_t> distinct = [](auto bigrams, auto first) {
        std::vector<bigram_t> distinct;
        for (auto b : bigrams) {
            if (b.first == first)
                distinct.push_back(b);
        }
        return distinct;
    } (bigrams, first);

    std::uniform_int_distribution<std::mt19937::result_type> rand_idx(0, distinct.size() - 1);

    return distinct[rand_idx(rng)].second;
}

std::string markov(const std::string &text, int words) {
    std::string generated;
    std::string formatted;
    for (auto ch : text) {
        if (isdigit(ch) || ispunct(ch) || ch == '\n')
            continue;
        formatted.push_back(ch);
    }

    // get bigrams
    auto bigrams = parse_bigrams(formatted);

    // for N iterations, pick random bigram.first and then random matching bigram.second
    std::random_device dev;
    std::mt19937 rng(dev());
    std::uniform_int_distribution<std::mt19937::result_type> rand_idx(0, bigrams.size() - 1);
    for (size_t i = 0; i < words; ++i) {
        auto random_first = bigrams[rand_idx(rng)].first;
        auto random_second = get_random_matching_second(bigrams, random_first);
        generated += random_first + " ";
        generated += random_second + " ";
    }

    return generated;
}

int main(int argc, char **argv) {

    // validate filename
    if (argc < 2) {
        std::cerr << "File missing.\n";
        return 1;
    }

    auto filename = argv[1];

    std::fstream file;
    std::stringstream text;
    file.exceptions(std::fstream::failbit | std::fstream::badbit);

    try {
        file.open(filename);
        text << file.rdbuf();
    } catch(std::fstream::failure &error) {
        std::cerr << "Exception opening/reading file.";
    }

    auto generated = markov(text.str(), 100);
    std::cout << generated;

    return 0;
}