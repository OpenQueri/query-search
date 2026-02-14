# Query Search Engine (Mini Project)

For word stemming (reducing words to their root), this project uses the **Snowball / libstemmer_c** library.  
The C++ source code has been included directly in this project for convenience when using it from Rust via FFI.  

❗ Note: The stemmer code is **not our own**, we are just wrapping it for use in our pipeline.  

Original Snowball project: [https://snowballstem.org/](https://snowballstem.org/)

---

## Supported Languages

- English (built into Snowball)
- Russian (built into Snowball)  
- **Ukrainian** (added manually)

### Ukrainian Stemmer

Special thanks to **[Roman Kobzar (Tapkomet)](https://github.com/Tapkomet)** for creating the Ukrainian Snowball stemmer.  
His work makes it possible to support the Ukrainian language in this project.

The Ukrainian stemmer algorithm is based on:  
[https://github.com/amakukha/stemmers_ukrainian/blob/master/src/tapkomet_stem/stem_ukr.sbl](https://github.com/amakukha/stemmers_ukrainian/blob/master/src/tapkomet_stem/stem_ukr.sbl)  
(3-Clause BSD License)

---

## Language Detection

To determine the language of each word before stemming, this project uses the **Compact Language Detector 2 (CLD2)** library.

CLD2 is a fast, accurate language detection library developed by Google, capable of identifying 83+ languages (including Ukrainian, Russian, and English) even from very short texts.

- **License**: Apache License 2.0
- **Original project**: [https://github.com/CLD2Owners/cld2](https://github.com/CLD2Owners/cld2)
- **Integration**: The library is linked as a system dependency (`libcld2`) or compiled from source for maximum portability.

The language detection step ensures that each word is processed with the correct stemmer (e.g., Ukrainian words go through the Ukrainian stemmer, Russian through the Russian stemmer, etc.).

---

## Usage

- Russian, Ukrainian, and English are supported via libstemmer_c.
- Language detection is performed using CLD2 before stemming.
- Each thread can have its own stemmer instance.
- For multithreaded search, use one stemmer per thread, or a single worker thread with a task queue.

### Example Workflow

1. Input text: "машинами українськими program"
2. Detect language per word:
   - "машинами" → Ukrainian
   - "українськими" → Ukrainian
   - "program" → English
3. Apply appropriate stemmer:
   - Ukrainian stemmer → "машин", "україн"
   - English stemmer → "program"
4. Build search index with normalized forms