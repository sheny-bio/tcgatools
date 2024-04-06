use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use polars::functions::concat_df_diagonal;

use xml::reader::{EventReader, XmlEvent};
use polars::prelude::*;
use rayon::prelude::*;

/// 读取xml临床结果文件，并转换成dataframe格式
pub fn read_xml(file: &str, cols: Option<Vec<String>>) -> Result<DataFrame, Box<dyn std::error::Error>> {

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let parser = EventReader::new(reader);
    let mut rslt: HashMap<String, String> = HashMap::new();

    // 遍历每一条信息，得到结构化数据
    let mut key = String::new();
    let mut value = String::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, ..}) => {
                key = name.local_name;
            }
            Ok(XmlEvent::Characters(data))  => {
                value = data;
            }
            Ok(XmlEvent::EndElement { name: _ }) => {
                rslt.insert(key.clone(), value.clone());
                key = String::new();
                value = String::new();
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    };


    // 将数据转换成dataframe
    let mut series = Vec::new();
    for (k, v) in rslt.iter() {

        if k == &"".to_string() {continue}

        if cols.is_none() || cols.as_ref().unwrap().contains(k) {
            series.push(Series::new(k, vec![v.as_str()]));
        }
    };
    let df_rslt = DataFrame::new(series)?;

    Ok(df_rslt)


}

/// 合并多个xml临床结果文件
pub fn xml_clinic_merge(files: Vec<String>, cols: Option<Vec<String>>, output: String) {

    let rslt = files.par_iter().map(|file| {
        read_xml(file, cols.clone()).unwrap()
    }).collect::<Vec<DataFrame>>();

    let df_rslt = concat_df_diagonal(&rslt).unwrap();  // 对角线合并

    // 把id列放到第一列
    let mut df_rslt = if cols.is_none() {
        let col_id = df_rslt.column("file_uuid").unwrap().clone();
        let mut df_rslt = df_rslt.drop("file_uuid").unwrap();
        df_rslt.insert_column(0, col_id).unwrap();
        df_rslt
    } else {df_rslt};

    // 保存结果
    let mut file = File::create(output).unwrap();
    CsvWriter::new(&mut file)
        .with_separator(b'\t')
        .finish(&mut df_rslt).unwrap();

}