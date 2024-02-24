use std::io::Read;

/// GNU df, a tool to report file system space usage
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "df")]
pub struct DfCommand {}

impl DfCommand {
    pub fn execute(&self) -> serde_json::Value {
        let mut input = String::new();
        std::io::stdin()
            .lock()
            .read_to_string(&mut input)
            .expect("read stdin to string");

        let parsed = DfLine::parse(&input);
        serde_json::to_value(parsed).expect("convert parsed object to json value")
    }
}

#[derive(serde::Serialize, Default)]
struct DfLine<'input> {
    pub filesystem: Option<&'input str>,
    pub r#type: Option<&'input str>,
    pub inodes: Option<u64>,
    pub iused: Option<u64>,
    pub ifree: Option<u64>,
    pub iuse_percent: Option<u64>,
    #[serde(rename = "1k_blocks")]
    pub kibi_blocks: Option<u64>,
    pub size: Option<u64>,
    pub used: Option<u64>,
    pub available: Option<u64>,
    pub use_percent: Option<u64>,
    pub file: Option<&'input str>,
    pub mounted_on: Option<&'input str>,
}

impl<'input> DfLine<'input> {
    pub fn parse(input: &'input str) -> Vec<Self> {
        let all_lines = input.lines().filter(|l| !l.is_empty()).collect::<Vec<_>>();
        let first_line = all_lines.get(0).expect("get first line");
        let first_line_len = first_line.len();

        // find char offset where each line has a space
        // TODO: this is not ideal because this can happen by accident! ideally we'd use a hardcoded
        // list of allowed headers
        let mut split_indices = (0..first_line_len)
            .filter(|&idx| {
                idx == 0
                    || all_lines
                        .iter()
                        .all(|&l| l.as_bytes().get(idx) == Some(&b' '))
            })
            .collect::<Vec<_>>();
        split_indices.push(usize::MAX / 2);

        let mut ret = vec![];
        for (line_idx, line) in all_lines.iter().enumerate().skip(1) {
            let mut ret_val = DfLine::default();

            for index in split_indices.windows(2) {
                let start = index[0];
                let end = index[1].min(line.len());
                let key_end = index[1].min(first_line.len());

                let key = first_line[start..key_end].trim();
                let slice = line[start..end].trim();

                match key {
                    "Filesystem" => ret_val.filesystem = Some(slice),
                    "Type" => ret_val.r#type = Some(slice),
                    "Inodes" => {
                        ret_val.inodes = Some(parse_number(slice).unwrap_or_else(|e| {
                            panic!("parse value '{slice}' for `inodes` on line {line_idx}: {e}")
                        }))
                    }
                    "IUsed" => {
                        ret_val.iused = Some(parse_number(slice).unwrap_or_else(|e| {
                            panic!("parse value '{slice}' for `iused` on line {line_idx}: {e}")
                        }))
                    }
                    "IFree" => {
                        ret_val.ifree = Some(parse_number(slice).unwrap_or_else(|e| {
                            panic!("parse value '{slice}' for `ifree` on line {line_idx}: {e}")
                        }))
                    }
                    "IUse%" => {
                        ret_val.iuse_percent = Some(parse_number(slice).unwrap_or_else(|e| {
                            panic!(
                                "parse value '{slice}' for `iuse_percent` on line {line_idx}: {e}"
                            )
                        }))
                    }
                    "1K-blocks" | "1024-blocks" => {
                        ret_val.kibi_blocks = Some(parse_number(slice).unwrap_or_else(|e| {
                            panic!(
                                "parse value '{slice}' for `kibi_blocks` on line {line_idx}: {e}"
                            )
                        }))
                    }
                    "Size" => {
                        ret_val.size = Some(parse_number(slice).unwrap_or_else(|e| {
                            panic!("parse value '{slice}' for `size` on line {line_idx}: {e}")
                        }))
                    }
                    "Used" => {
                        ret_val.used = Some(parse_number(slice).unwrap_or_else(|e| {
                            panic!("parse value '{slice}' for `used` on line {line_idx}: {e}")
                        }))
                    }
                    // TODO: I highly doubt that this is correct, but jc doesn't fare much better
                    "Avail" | "Available" => {
                        ret_val.available = Some(parse_number(slice).unwrap_or_else(|e| {
                            panic!("parse value '{slice}' for `available` on line {line_idx}: {e}")
                        }))
                    }
                    "Use%" => {
                        ret_val.use_percent = Some(parse_number(slice).unwrap_or_else(|e| {
                            panic!(
                                "parse value '{slice}' for `use_percent` on line {line_idx}: {e}"
                            )
                        }))
                    }
                    "File" => ret_val.file = Some(slice),
                    "Mounted on" => ret_val.mounted_on = Some(slice),
                    _ => (),
                }
            }

            ret.push(ret_val);
        }

        ret
    }
}

// assume `-h` (--human-readable) is used over `-H` (--si)
const SUFFIXES: [(&str, u64); 6] = [
    ("B", 1),
    ("K", 1024),
    ("M", 1024 * 1024),
    ("G", 1024 * 1024 * 1024),
    ("T", 1024 * 1024 * 1024 * 1024),
    ("P", 1024 * 1024 * 1024 * 1024 * 1024),
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

    let num = num.trim_end_matches('%');

    Ok((num.parse::<f64>()? * mul as f64) as u64)
}
