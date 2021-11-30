#pragma once
#include <memory>
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
std::unique_ptr<std::string> markov(const std::string &text, int words);

std::vector<bigram_t> parse_bigrams(const std::string &text);

std::vector<std::string> split(const std::string &str, const std::string &delim);

std::string get_random_matching_second(const std::vector<bigram_t> &bigrams, const std::string &first);
