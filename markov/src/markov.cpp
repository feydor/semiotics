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

typedef std::pair<std::string, std::string> bigram_t;
typedef std::vector<std::string> ngram_t;

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
    auto generated = generate_text_from_ngrams<ngram_t::iterator>(ngrams, words);
    return std::make_unique<std::string>(generated);
}

/**
 * @brief divides the range into n chunks, where each chunk is a pair of It
 * 
 * @tparam It 
 * @param range_from 
 * @param range_to 
 * @param n 
 * @return std::vector<std::pair<It, It>> 
 */
template <std::forward_iterator It>
std::vector<std::pair<It, It>> chunk(It range_from, It range_to, const long total, const std::ptrdiff_t n) {
    const std::ptrdiff_t portion { n };
    std::vector<std::pair<It, It>> chunks(total / portion);

    It portion_end { range_from };

    std::generate(chunks.begin(), chunks.end(), [&portion_end, portion]() {
        It portion_start { portion_end };

        portion_end += portion;
        return std::make_pair(portion_start, portion_end);
    });

    // add in last
    chunks.back().second = range_to;

    return chunks;
}

/**
 * @brief get a vector of bigrams from a string
 * 
 * @param text 
 * @return std::vector<bigram_t> 
 */
std::vector<bigram_t> parse_bigrams(const std::string &text) {
    std::vector<std::string> tokens = split(text, ' ');
    std::vector<bigram_t> bigrams;

    if (!text.empty()) {
        for (auto itr = tokens.begin(); itr != tokens.end()-1; itr++)
            bigrams.emplace_back(*itr, *(itr + 1));
    }

    return bigrams;
}

template <std::forward_iterator It>
std::vector<std::pair<It, It>> parse_ngrams(std::vector<std::string> &tokens, auto n) {
    return chunk(tokens.begin(), tokens.end(), tokens.size(), n);
}

template <std::forward_iterator It>
std::string generate_text_from_ngrams(const std::vector<std::pair<It, It>> &ngrams, int words) {
    std::random_device dev;
    std::mt19937 rng(dev());
    std::uniform_int_distribution<std::mt19937::result_type> rand_idx(0, ngrams.size()-1);

    std::string generated;
    for (auto unused = 0; unused < words; ++unused) {
        auto idx = rand_idx(dev);
        auto random_first = *(ngrams[idx].first);

        // std::uniform_int_distribution<std::mt19937::result_type> rand_idx2(0,
        //     std::distance(ngrams[idx].first, ngrams[idx].second) - 1);
        // auto second_idx = rand_idx2(dev);

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
