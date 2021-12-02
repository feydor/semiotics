#include "markov/src/markov.h"
#include <algorithm>
#include <cstdint>
#include <cstring> 
#include <fstream>
#include <iostream>
#include <memory>
#include <set>
#include <sstream>
#include <random>
#include <ranges>

typedef std::vector<std::string> ngram_t;

/**
 * @brief random text generation with N chunks
 * 
 * @param text 
 * @param words 
 * @param N 
 * @return std::unique_ptr<std::string> 
 */
std::unique_ptr<std::string> markovn(const std::string &text, int words, int N) {
    // strip useless characters from text
    std::string formatted;
    std::remove_copy_if(text.begin(), text.end(), std::back_inserter(formatted), [](auto ch){
        return (isdigit(ch) || ispunct(ch) || ch == '\n');
    });

    // split by space
    auto tokens = split(formatted, ' ');

    // get ngrams
    auto ngrams = parse_ngrams<ngram_t::iterator>(tokens, N);

    // generate text
    auto generated = generate_text_from_ngrams<ngram_t::iterator>(ngrams, words, N);
    return std::make_unique<std::string>(generated);
}

/**
 * @brief get ngrams from tokens, an ngram is a sliding window of n size
 * 
 * @tparam It 
 * @param tokens 
 * @param n 
 * @return std::vector<std::pair<It, It>> 
 */
template <std::forward_iterator It>
std::vector<std::pair<It, It>> parse_ngrams(std::vector<std::string> &tokens, auto n) {
    std::vector<std::pair<It, It>> ngrams;
    for (auto itr = tokens.begin(); itr != tokens.end(); ++itr)
        ngrams.emplace_back(std::make_pair   (itr, itr+n));
    return ngrams;
}

template <std::forward_iterator It>
std::string generate_text_from_ngrams(const std::vector<std::pair<It, It>> &ngrams, int words, int N) {
    std::random_device dev;
    std::mt19937 rng(dev());
    std::uniform_int_distribution<> rand_idx(0, ngrams.size()-1);

    std::string generated;
    for (auto unused = 0; unused < words; unused += N) {
        auto idx = rand_idx(rng);
        auto random_first = *(ngrams[idx].first);

        std::string random_second;
        for (auto itr = ngrams[idx].first+1; itr != ngrams[idx].second; ++itr)
            random_second += *itr + " ";
        
        generated += random_first + " " + random_second;
    }
    return generated;
}

std::vector<std::string> split(const std::string &str, const char delim) {
    std::stringstream ss(str);
    std::string s;
    std::vector<std::string> tokens;

    while (std::getline(ss, s, delim)) {
        tokens.push_back(s);
    }

    std::remove_if(tokens.begin(), tokens.end(), [](auto str){ return str == " " || str.empty(); });

    return tokens;
}
