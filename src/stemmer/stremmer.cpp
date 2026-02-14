#include <iostream>
#include <string>
#include <map>
#include "include/libstemmer.h"


int main() {
    // Створюємо стеммер для української
    sb_stemmer* stemmer = sb_stemmer_new("ukrainian", "UTF_8");
    if (!stemmer) {
        std::cerr << "Error stemmer \n";
        return 1;
    }

    // Масив тестових слів
    std::string words[] = {"очі", "очей", "очам", "очах"};
    
    for (const auto& word : words) {
        // Отримуємо стем
        const sb_symbol* stem = sb_stemmer_stem(
            stemmer, 
            (const sb_symbol*)word.c_str(), 
            word.size()
        );
        
        std::string stem_str((const char*)stem);
        
        
        std::cout << "word: " << word 
                  << " -> stremm: " << stem_str << std::endl;
    }
    

    sb_stemmer_delete(stemmer);
    return 0;
}