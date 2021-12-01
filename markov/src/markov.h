#pragma once
#include <cstdint>
#include <memory>
#include <string>
#include <vector>
#include <iterator>

typedef std::pair<std::string, std::string> bigram_t;

/**
 * @brief generates text using a markov chain
 * 
 * @param text 
 * @param words 
 * @return std::string 
 */
std::unique_ptr<std::string> markov(const std::string &text, int words);

std::unique_ptr<std::string> markovn(const std::string &text, int words, int N);

template <std::forward_iterator It>
std::vector<std::pair<It, It>> chunk(It range_from, It range_to, const long total, const std::ptrdiff_t n);

template <std::forward_iterator It>
std::vector<std::pair<It, It>> parse_ngrams(std::vector<std::string> &tokens, auto n);

template <std::forward_iterator It>
std::string generate_text_from_ngrams(const std::vector<std::pair<It, It>> &ngrams, int words);

std::vector<bigram_t> parse_bigrams(const std::string &text);

std::vector<std::string> split(const std::string &str, const char delim);

std::string get_random_matching_second(const std::vector<bigram_t> &bigrams, const std::string &first);
