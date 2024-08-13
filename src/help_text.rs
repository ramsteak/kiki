pub const AFTER_HELP: &'static str = color_print::cstr!(
    "<bold><underline>Methods list:</underline></bold>
    - LSB      Least significant bit. (lossless only)
<bold><underline>Method options list:</underline></bold>
  <underline>LSB Least Significant Bit:</underline>
    - SEQ : uses consecutive pixels, starting from the top left.
    - RNG : uses random pixels, determined by the passed key."
);

pub const EMBED_IMAGE: &'static str = "The path to the image to hide the data in.";

pub const EMBED_OUTPUT: &'static str = "Path of the output image.";

pub const EMBED_SECRET: &'static str =
    "Path to the file containing the secret. If unspecified or \"-\", read from stdin.";

pub const EXTRACT_IMAGE: &'static str = "The path to the image to extract data from.";

pub const EXTRACT_OUTPUT: &'static str =
    "The file path to write the data to. If unspecified or \"-\", write to stdout.";

pub const METHOD: &'static str =
    "The method to use for the operation. The list is available on the help menu.";

pub const KEY: &'static str =
    "The key to use for the operation. If unspecified, an empty string will be used.";

pub const VERBOSE: &'static str = "Gives additional output, useful for debugging.";

pub const OPTIONS: &'static str =
    "Additional method-specific options. The list is available on the help menu.";
