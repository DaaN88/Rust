fn main() {
    let text = r#"
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Count {
    lines_in_total: usize,
    lines_containing_code: usize,
    empty_lines: usize,
    code_symbols: usize,
}

impl Count {
    #[allow(dead_code)]
    pub fn calculate(text: &str) -> Count {
        todo!("Implement me!")
    }
}"#;

    let count = Count::calculate(text);

    println!("{:?}", count);
}

// ===

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Count {
    lines_in_total: usize,
    lines_containing_code: usize,
    empty_lines: usize,
    code_symbols: usize,
}

impl Count {
    #[allow(dead_code)]
    pub fn calculate(text: &str) -> Count {
        let lines: Vec<&str> = text.lines().collect();

        let mut lines_in_total = 0;
        let mut lines_containing_code = 0;
        let mut empty_lines = 0;
        let mut code_symbols = 0;

        for line in lines {
            lines_in_total += 1;

            let trimmed = line.trim();

            if trimmed.is_empty() {
                empty_lines += 1;
            } else {
                lines_containing_code += 1;

                code_symbols += line.len() + 1; // +1 для символа новой строки
            }
        }

        // Уменьшаем на 1, чтобы не учитывать trailling \n
        if code_symbols > 0 {
            code_symbols -= 1;
        }

        Count {
            lines_in_total,
            lines_containing_code,
            empty_lines,
            code_symbols,
        }
    }
}
