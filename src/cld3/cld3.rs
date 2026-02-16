use cld3::NNetLanguageIdentifier;
use std::error::Error;
use object_pool::Pool;
use std::cell::RefCell;

thread_local! {
    static CLD_POOL: RefCell<Option<Pool<NNetLanguageIdentifier>>> = RefCell::new(
        NNetLanguageIdentifier::new(0, 500).ok().map(|detector| {
            Pool::new(32, || {
                NNetLanguageIdentifier::new(0, 500)
                    .expect("Failed to create CLD3 detector")
            })
        })
    );
}

pub async fn cld3_main(text: &str) -> Result<String, Box<dyn Error>> {
    CLD_POOL.with(|pool_cell| {
        let mut pool_opt = pool_cell.borrow_mut();
        
        match pool_opt.as_mut() {
            Some(pool) => {
                let mut detector = pool.pull(|| {
                    NNetLanguageIdentifier::new(0, 500)
                        .expect("Failed to create CLD3 detector")
                });
                
                let rs = detector.find_language(text);
                Ok(rs.language)
            }
            None => Err("CLD3 not available".into())
        }
    })
}