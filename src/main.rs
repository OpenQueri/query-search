use std::error::Error;

use rayon::prelude::*;

use cld2_lib::{detect_lang, is_ukrainian, is_russian, is_english};

fn main() -> Result<(), Box<dyn Error>> {

let test_strings = vec![
    // Українські
    ("Привіт мене зовуть софійко".to_string(), "uk".to_string()),
    ("Доброго ранку, як справи?".to_string(), "uk".to_string()),
    ("рецепт борщу з буряком".to_string(), "uk".to_string()),
    ("купити книжки онлайн дешево".to_string(), "uk".to_string()),
    ("погода в києві на завтра".to_string(), "uk".to_string()),
    ("як вивчити англійську мову швидко".to_string(), "uk".to_string()),
    ("фільми українською мовою дивитися онлайн".to_string(), "uk".to_string()),
    ("новини україни сьогодні останні".to_string(), "uk".to_string()),
    ("перекладач з англійської на українську".to_string(), "uk".to_string()),
    ("дешеві авіаквитки львів варшава".to_string(), "uk".to_string()),
    ("ліки від головного болю ціна".to_string(), "uk".to_string()),
    ("вакансії програміст київ віддалено".to_string(), "uk".to_string()),
    ("курс долара до гривні".to_string(), "uk".to_string()),
    ("мапа укриттів у львові".to_string(), "uk".to_string()),
    ("графік відключення світла".to_string(), "uk".to_string()),
    
    // Російські
    ("Привет меня зовут София".to_string(), "ru".to_string()),
    ("Доброе утро, как дела?".to_string(), "ru".to_string()),
    ("рецепт борща со свеклой".to_string(), "ru".to_string()),
    ("купить книги онлайн дешево".to_string(), "ru".to_string()),
    ("погода в москве на завтра".to_string(), "ru".to_string()),
    ("как выучить английский язык быстро".to_string(), "ru".to_string()),
    ("фильмы на русском смотреть онлайн".to_string(), "ru".to_string()),
    ("новости россии сегодня последние".to_string(), "ru".to_string()),
    ("переводчик с английского на русский".to_string(), "ru".to_string()),
    ("дешевые авиабилеты москва сочи".to_string(), "ru".to_string()),
    ("лекарство от головной боли цена".to_string(), "ru".to_string()),
    ("вакансии программист москва удаленно".to_string(), "ru".to_string()),
    ("курс доллара к рублю".to_string(), "ru".to_string()),
    ("карта бомбоубежищ в москве".to_string(), "ru".to_string()),
    ("график отключения света".to_string(), "ru".to_string()),
    
    // Англійські
    ("Hello my name is Sophia".to_string(), "en".to_string()),
    ("Good morning, how are you?".to_string(), "en".to_string()),
    ("borscht recipe with beets".to_string(), "en".to_string()),
    ("buy books online cheap".to_string(), "en".to_string()),
    ("weather in london tomorrow".to_string(), "en".to_string()),
    ("how to learn english fast".to_string(), "en".to_string()),
    ("watch movies online free".to_string(), "en".to_string()),
    ("latest news ukraine today".to_string(), "en".to_string()),
    ("translate english to ukrainian".to_string(), "en".to_string()),
    ("cheap flights to warsaw".to_string(), "en".to_string()),
    ("headache medicine price".to_string(), "en".to_string()),
    ("software engineer jobs remote".to_string(), "en".to_string()),
    ("dollar to euro exchange rate".to_string(), "en".to_string()),
    ("shelter map kyiv".to_string(), "en".to_string()),
    ("power outage schedule".to_string(), "en".to_string()),
    
    // Мікс (для перевірки)
    ("київ погода завтра".to_string(), "uk".to_string()),
    ("львів новини".to_string(), "uk".to_string()),
    ("ukraine news".to_string(), "en".to_string()),
    ("война в украине новости".to_string(), "ru".to_string()),
    ("boršč recept".to_string(), "cs".to_string()),  // чеська
    ("przepis na barszcz".to_string(), "pl".to_string()),  // польська
];
    

    let _: Vec<_> = test_strings.par_iter().map(|string| {
        let detect = detect_lang(&string.0);
        println!("Worsd: {} {}: {}", string.0,string.1,detect);
    }).collect();


    

    Ok(())
}
