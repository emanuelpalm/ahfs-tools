use std::collections::HashMap;

pub struct Duplicate<'a, E> {
    pub original: &'a E,
    pub duplicate: &'a E,
}

pub fn find_duplicate<E>(items: &[E]) -> Option<Duplicate<E>>
    where E: AsRef<str>,
{
    let len = items.len();
    if len < 16 {
        for a in items {
            let a0 = a.as_ref();
            let mut count = 0;
            for b in items {
                if a0 == b.as_ref() {
                    count += 1;
                    if count > 1 {
                        return Some(Duplicate {
                            original: a,
                            duplicate: b,
                        });
                    }
                }
            }
        }
    } else {
        let mut map = HashMap::with_capacity(items.len());
        for item in items {
            if let Some(original) = map.insert(item.as_ref(), item) {
                return Some(Duplicate {
                    original,
                    duplicate: item,
                });
            }
        }
    }
    None
}