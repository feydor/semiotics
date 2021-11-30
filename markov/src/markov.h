#pragma once
#include <string>
#include <vector>

typedef std::pair<std::string, std::string> bigram_t;

/**
 * @brief generates text using a markov chain
 * 
 * @param text 
 * @param words 
 * @return std::string 
 */
std::string markov(const std::string &text, int words);

std::vector<bigram_t> parse_bigrams(const std::string &text);

auto get_random_matching_second(const std::vector<bigram_t> &bigrams, const std::string &first);

auto split(const std::string &str, const std::string &delim);