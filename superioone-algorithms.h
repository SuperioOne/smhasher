#ifndef __SUPERIOONE_ALGORITHMS_H
#define __SUPERIOONE_ALGORITHMS_H

#include <cstdint>

extern "C" void rust_murmur3_128(const void *input, std::size_t len, std::uint64_t seed, const void *out);
extern "C" void rust_murmur3_32(const void *input, std::size_t len, std::uint32_t seed, const void *out);
extern "C" void rust_crc32c(const void *input, std::size_t len, std::uint32_t seed, const void *out);
extern "C" void rust_cityhash_32(const void *input, std::size_t len, const void *out);
extern "C" void rust_cityhash_64(const void *input, std::size_t len, std::uint64_t seed, const void *out);
extern "C" void rust_cityhash_64_no_seed(const void *input, std::size_t len, const void *out);
extern "C" void rust_cityhash_128(const void *input, std::size_t len, std::uint64_t seed0, std::uint64_t seed1, const void *out);
extern "C" void rust_cityhash_128_no_seed(const void *input, std::size_t len, const void *out);
extern "C" void rust_cityhash_128crc(const void *input, std::size_t len, std::uint64_t seed0, std::uint64_t seed1, const void *out);
extern "C" void rust_cityhash_256crc(const void *input, std::size_t len, const void *out);

inline void Rust_Test_murmur3_128(const void *input, const int len, const std::uint32_t seed, void *out) {
  rust_murmur3_128(input, len, seed, out);
}

inline void Rust_Test_murmur3_32(const void *input, const int len, const std::uint32_t seed, void *out) {
  rust_murmur3_32(input, len, seed, out);
}

inline void Rust_Test_crc32c(const void *input, const int len, const std::uint32_t seed, void *out) {
  if (!len) {
    *(uint32_t *)out = 0;
    return;
  }

  rust_crc32c(input, len, seed, out);
}

inline void Rust_Test_cityhash_32(const void *input, const int len, const std::uint32_t seed, void *out) {
  rust_cityhash_32(input, len, out);
}

inline void Rust_Test_cityhash_64(const void *input, const int len, const std::uint32_t seed, void *out) {
  rust_cityhash_64(input, len, seed, out);
}

inline void Rust_Test_cityhash_64_no_seed(const void *input, const int len, const std::uint32_t seed, void *out) {
  rust_cityhash_64_no_seed(input, len, out);
}

inline void Rust_Test_cityhash_128(const void *input, const int len, const std::uint32_t seed, void *out) {
  rust_cityhash_128(input, len, seed, 0, out);
}

inline void Rust_Test_cityhash_128_no_seed(const void *input, const int len, const std::uint32_t seed, void *out) {
  rust_cityhash_128_no_seed(input, len, out);
}

inline void Rust_Test_cityhash_128crc(const void *input, const int len, const std::uint32_t seed, void *out) {
  rust_cityhash_128crc(input, len, seed, 0, out);
}

inline void Rust_Test_cityhash_256crc(const void *input, const int len, const std::uint32_t seed, void *out) {
  rust_cityhash_256crc(input, len, out);
}
#endif
