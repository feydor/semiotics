#pragma once
#include <cstdint>
#include <memory>
#include <string>
#include <vector>
#include <iterator>


std::unique_ptr<std::string> markovn(const std::string &text, int words, int N);

template <std::forward_iterator It>
std::vector<std::pair<It, It>> chunk(It range_from, It range_to, const long total, const std::ptrdiff_t n);

template <std::forward_iterator It>
std::vector<std::pair<It, It>> parse_ngrams(std::vector<std::string> &tokens, auto n);

template <std::forward_iterator It>
std::string generate_text_from_ngrams(const std::vector<std::pair<It, It>> &ngrams, int words, int N);

std::vector<std::string> split(const std::string &str, const char delim);
