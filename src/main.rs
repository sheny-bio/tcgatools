use clap::{Parser, Subcommand};
use rayon::ThreadPoolBuilder;

use tcgatools::xml_clinic_merge::xml_clinic_merge;

/// tcga数据处理工具
#[derive(Parser)]
#[command(name = "tcgatools")]
#[command(author = "shen yi")]
#[command(version = "0.1.0")]
#[command(about = "Call mutations from low-depth WGS data", long_about = None)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>,

}

#[derive(Subcommand)]
enum Commands {

    /// 合并tcga xml格式的临床数据
    #[command(name = "clinic_merge")]
    ClinicMerge {

        /// 需要输出的列，多个列用逗号分隔
        #[clap(short='c', long="cols")]
        cols: Option<String>,

        /// 输出文件路径
        #[clap(short='o', long="output")]
        output: String,

        /// 临床文件路径
        files: Vec<String>,

        /// 最大线程数
        #[clap(short='t', long="threads", default_value="1")]
        threads: usize,
    },
}


fn main() {
    // 解析命令行参数
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ClinicMerge { cols, output, files, threads }) => {
            ThreadPoolBuilder::new().num_threads(threads.clone()).build_global().unwrap();
            let cols = match cols {
                Some(cols) => Some(cols.split(",").map(|s| s.to_string()).collect::<Vec<String>>()),
                None => None
            };
            xml_clinic_merge(files.clone(), cols, output.clone());
        }
        None => {}
    }
}