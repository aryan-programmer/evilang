use clap::Parser;

use evilang_lib::types::string::StringT;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArguments {
	#[arg(long, short, help = "File to execute", value_name = "FILE")]
	pub file: Option<StringT>,
	#[arg(long = "src-debug-dnu", help = "Source Debug (Do Not Use)", action, hide = true)]
	pub src_debug: bool,
}
