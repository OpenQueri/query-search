#include <cstring>
#include "compact_lang_det.h"

extern "C" {

const char* detect_language(const char* text, int length) {
    bool is_reliable;
    
    CLD2::Language lang = CLD2::DetectLanguage(
        text, 
        length, 
        true, 
        &is_reliable
    );
    
    return CLD2::LanguageCode(lang);
}

int detect_language_detailed(
    const char* text, 
    int length, 
    char* lang_code, 
    float* confidence
) {
    bool is_reliable;
    
    CLD2::Language lang = CLD2::DetectLanguage(
        text, 
        length, 
        true, 
        &is_reliable
    );
    
    strcpy(lang_code, CLD2::LanguageCode(lang));
    *confidence = is_reliable ? 1.0f : 0.5f;
    
    return length;
}

}