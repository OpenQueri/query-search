
use cld3::NNetLanguageIdentifier;
use once_cell::sync::Lazy;
use std::error::Error;

use object_pool::Pool;
use std::cell::RefCell;

thread_local! {
    static CLD_POOL: RefCell<Pool<NNetLanguageIdentifier>> = RefCell::new(
        Pool::new(32, || NNetLanguageIdentifier::new(0, 500).unwrap())
    );
}

pub async fn cld3_main<'a>(text: &'a str) -> Result<String, Box<dyn Error>>{

    CLD_POOL.with(|pool_cell: &RefCell<Pool<NNetLanguageIdentifier>>| {
            let pool = pool_cell.borrow_mut();

            let mut detector = pool.pull(|| {
                NNetLanguageIdentifier::new(0, 1000).unwrap()
            });

            let rs = detector.find_language(&text);
            let language = rs.language;

            Ok(language)
        })
}