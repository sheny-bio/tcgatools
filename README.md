# tcgatools

github: https://github.com/sheny-bio/tcgatools

使用rust编写的，tcga数据批处理命令行工具。主要特点是方便，速度快。目前只实现了临床xml格式合并的功能，后面根据需求不断完善。



## 下载与使用

我已经将构建好的工具，放到了bin目录。只需clone该库，即可使用。如果有错误，自行构建一下即可。mac平台的需自行构建。



## 使用示例

### 合并xml合适的临床文件

+ 解析所有xml文件，并合并到一个文件中

  ```bash
  bin/tcgatools clinic_merge  -o ./123.tsv -t 3  demo/*
  ```

+ 指定所需列

  ```bash
  bin/tcgatools clinic_merge --cols "file_uuid,kras_gene_analysis_performed" -o ./123.tsv -t 3   demo/*
  ```

  

