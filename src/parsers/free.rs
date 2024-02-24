use std::io::Read;

/// Linux free, to display the amount of free and unused memory
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "free")]
pub struct FreeCommand {}

impl FreeCommand {
    pub fn execute(&self) -> serde_json::Value {
        let mut input = String::new();
        std::io::stdin()
            .lock()
            .read_to_string(&mut input)
            .expect("read stdin to string");

        let parsed = FreeLine::parse(&input);
        serde_json::to_value(parsed).expect("convert parsed object to json value")
    }
}

#[derive(serde::Serialize, Default)]
pub struct FreeLine<'input> {
    pub r#type: &'input str,
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub shared: Option<u64>,
    /// Mutually exclusive with [`buffers`] and [`cache`].
    pub buff_cache: Option<u64>,
    /// Mutually exclusive with [`buff_cache`].
    pub buffers: Option<u64>,
    /// Mutually exclusive with [`buff_cache`].
    pub cache: Option<u64>,
    pub available: Option<u64>,
}

impl<'input> FreeLine<'input> {
    pub fn parse(input: &'input str) -> Vec<Self> {
        let mut lines = input.lines();
        let first_line = lines.next().expect("get first input line");
        let columns = first_line
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let mut results = vec![];
        for (line_idx, line) in lines.enumerate() {
            let (r#type, rest) = line
                .split_once(':')
                .unwrap_or_else(|| panic!("find `:` in line {line_idx}"));

            let mut ret_val = FreeLine {
                r#type,
                ..Default::default()
            };

            let rest_iter = rest.split(' ').filter(|s| !s.is_empty());

            macro_rules! parse_arm {
                ($v:ident, $line_idx:ident, $lit:literal, $mem:ident) => {
                    ret_val.$mem = parse_number($v).unwrap_or_else(|e| {
                        panic!(
                            concat!("parse value '{}' for `", $lit, "` on line {}: {}"),
                            $v, $line_idx, e
                        )
                    })
                };
            }
            macro_rules! parse_arm_opt {
                ($v:ident, $line_idx:ident, $lit:literal, $mem:ident) => {
                    ret_val.$mem = Some(parse_number($v).unwrap_or_else(|e| {
                        panic!(
                            concat!("parse value '{}' for `", $lit, "` on line {}: {}"),
                            $v, $line_idx, e
                        )
                    }))
                };
            }

            for (&k, v) in columns.iter().zip(rest_iter) {
                match k {
                    "total" => parse_arm!(v, line_idx, "total", total),
                    "used" => parse_arm!(v, line_idx, "used", used),
                    "free" => parse_arm!(v, line_idx, "free", free),
                    "shared" => parse_arm_opt!(v, line_idx, "shared", shared),
                    "buff/cache" => parse_arm_opt!(v, line_idx, "buff/cache", buff_cache),
                    "buffers" => parse_arm_opt!(v, line_idx, "buffers", buffers),
                    "cache" => parse_arm_opt!(v, line_idx, "cache", cache),
                    "available" => parse_arm_opt!(v, line_idx, "available", available),
                    _ => (),
                }
            }

            results.push(ret_val);
        }

        results
    }
}

const SUFFIXES: [(&str, u64); 11] = [
    ("B", 1),
    ("K", 1000),
    ("M", 1000 * 1000),
    ("G", 1000 * 1000 * 1000),
    ("T", 1000 * 1000 * 1000 * 1000),
    ("P", 1000 * 1000 * 1000 * 1000 * 1000),
    ("Ki", 1024),
    ("Mi", 1024 * 1024),
    ("Gi", 1024 * 1024 * 1024),
    ("Ti", 1024 * 1024 * 1024 * 1024),
    ("Pi", 1024 * 1024 * 1024 * 1024 * 1024),
];

fn parse_number(num: &str) -> Result<u64, std::num::ParseFloatError> {
    let (mut num, mut mul) = (num, 1);
    for (suffix, suffix_mul) in SUFFIXES {
        if num.ends_with(suffix) {
            num = num.trim_end_matches(suffix);
            mul = suffix_mul;
            break;
        }
    }

    Ok((num.parse::<f64>()? * mul as f64) as u64)
}

#[cfg(test)]
mod tests {
    macro_rules! parse {
        ($file_name:literal) => {
            super::FreeLine::parse(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/test-cases/free/",
                $file_name,
            )))
        };
    }

    #[test]
    fn parse_no_args() {
        let _parsed = parse!("no-args.txt");
    }

    #[test]
    fn parse_total() {
        let _parsed = parse!("total.txt");
    }

    #[test]
    fn parse_wide() {
        let _parsed = parse!("wide.txt");
    }

    #[test]
    fn parse_human() {
        let _parsed = parse!("human.txt");
    }

    #[test]
    fn parse_human_si() {
        let _parsed = parse!("human-si.txt");
    }
}
