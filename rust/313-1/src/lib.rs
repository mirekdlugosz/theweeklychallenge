pub fn broken_keys(intended: &str, typed: &str) -> bool {
    let binding: Vec<_> = intended.chars().collect();
    let mut intended_chunks = binding.chunk_by(|a, b| a == b);
    let binding: Vec<_> = typed.chars().collect();
    let mut typed_chunks = binding.chunk_by(|a, b| a == b);

    loop {
        let intended_chunk = intended_chunks.next();
        let typed_chunk = typed_chunks.next();
        let mut intended_char = 'i';
        let mut typed_char = 't';
        let mut intended_len = 0;
        let mut typed_len = 0;

        if intended_chunk.is_none() && typed_chunk.is_none() {
            break;
        }

        // one is shorter than the other
        if intended_chunk.is_none() || typed_chunk.is_none() {
            return false;
        }

        // this is a bit awkward - we know both chunks contain a value, but compiler does not
        if let Some(chunk) = intended_chunk {
            intended_char = *chunk.first().unwrap_or(&'i');
            intended_len = chunk.len();
        }

        if let Some(chunk) = typed_chunk {
            typed_char = *chunk.first().unwrap_or(&'t');
            typed_len = chunk.len();
        }

        if intended_char != typed_char {
            return false;
        }

        if intended_len > typed_len {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = broken_keys("perl", "perrrl");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = broken_keys("raku", "rrakuuuu");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = broken_keys("python", "perl");
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = broken_keys("coffeescript", "cofffeescccript");
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = broken_keys("pythonista", "ppythhoooon");
        assert_eq!(result, false);
    }

    #[test]
    fn test6() {
        let result = broken_keys("coffeescript", "cofffescccript");
        assert_eq!(result, false);
    }
}
