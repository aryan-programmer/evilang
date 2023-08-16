use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArguments {
	#[arg(long, short, help = "File to execute", value_name = "FILE")]
	pub file: Option<String>,
}
