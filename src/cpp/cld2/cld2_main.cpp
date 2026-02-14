#include <iostream>
#include <string>
#include <cstring>
#include "compact_lang_det.h"

int main() {
    // Тестові слова
    std::string words[] = {
        // Українські
        "машинами їхали по дорозі",
        "книжки лежали на столі",
        "читання книг розвиває мозок",
        "унікальний випадок в історії",
        "сонячне проміння зігріває землю",
        
        // Російські
        "машины ехали по дороге",
        "книги лежали на столе",
        "чтение книг развивает мозг",
        "уникальный случай в истории",
        "солнечный свет согревает землю",
        
        // Англійські
        "cars were driving on the road",
        "books were lying on the table",
        "reading develops the brain",
        "unique case in history",
        "sunlight warms the earth",
        
        // Мікс (для перевірки)
        "привіт як справи",
        "hello world",
        "как дела"
    };
    
    std::cout << "==================================\n";
    
    for (const auto& word : words) {
        bool is_reliable;
        
        CLD2::Language lang = CLD2::DetectLanguage(
            word.c_str(),
            word.length(),
            true,  // plain text
            &is_reliable
        );
        
        std::cout << "Слово: \"" << word << "\"\n";
        std::cout << "  Мова: " << CLD2::LanguageCode(lang) << "\n";
        std::cout << "  Надійно: " << (is_reliable ? "так" : "ні") << "\n";
    }
    
    return 0;
}