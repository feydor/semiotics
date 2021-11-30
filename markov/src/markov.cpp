#include "markov/src/markov.h"
#include <algorithm>
#include <cstring> 
#include <fstream>
#include <iostream>
#include <memory>
#include <set>
#include <sstream>
#include <random>
#include <ranges>

typedef std::pair<std::string, std::string> bigram_t;

/**
 * @brief random text generation with variable number of words
 * 
 * @param text 
 * @param words 
 * @return std::unique_ptr<std::string> 
 */
std::unique_ptr<std::string> markov(const std::string &text, int words) {
    // strip useless characters from text
    std::string formatted;
    std::remove_copy_if(text.begin(), text.end(), std::back_inserter(formatted), [](auto ch){
        return (isdigit(ch) || ispunct(ch) || ch == '\n');
    });

    // get bigrams
    auto bigrams = parse_bigrams(formatted);

    // for N iterations, pick random bigram.first and then random matching bigram.second
    std::random_device dev;
    std::mt19937 rng(dev());
    std::uniform_int_distribution<std::mt19937::result_type> rand_idx(0, bigrams.size() - 1);

    std::string generated;
    for (auto i = 0; i < words; ++i) {
        auto random_first = bigrams[rand_idx(rng)].first;
        auto random_second = get_random_matching_second(bigrams, random_first);
        generated += random_first + " " + random_second + " ";
    }

    return std::make_unique<std::string>(generated);
}

/**
 * @brief get a vector of bigrams from a string
 * 
 * @param text 
 * @return std::vector<bigram_t> 
 */
std::vector<bigram_t> parse_bigrams(const std::string &text) {
    std::vector<std::string> tokens = split(text, " ");
    std::vector<bigram_t> bigrams;

    if (!text.empty()) {
        for (auto itr = tokens.begin(); itr != tokens.end()-1; itr++)
            bigrams.emplace_back(*itr, *(itr + 1));
    }

    return bigrams;
}

std::vector<std::string> split(const std::string &str, const std::string &delim) {
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

std::string get_random_matching_second(const std::vector<bigram_t> &bigrams,
                                       const std::string &first) {
    
    // get the subset of bigrams whose first match
    std::vector<bigram_t> bigrams_matching_first;
    std::copy_if(bigrams.begin(), bigrams.end(), std::back_inserter(bigrams_matching_first),
        [first](auto bigram) {
            return bigram.first == first;
        });

    // from the subset of matching bigrams, pick a random second
    std::random_device dev;
    std::mt19937 rng(dev());
    std::uniform_int_distribution<std::mt19937::result_type> rand_idx(0, bigrams_matching_first.size() - 1);
    return bigrams_matching_first[rand_idx(rng)].second;
}
