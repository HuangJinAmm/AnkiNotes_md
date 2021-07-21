### AnkiNotes

---

通过md生成anki记忆卡片



### 使用

---

```sh
USAGE:
    akmd.exe [FILE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>...    需要处理的文件，没有则默认当前文件夹下面所有以md结尾的文件 生成同名的apkg文件
```





### 设置

---

和*本程序同路径*下的`setting.toml`。**没有或者字段为空**，则使用默认的设置

```toml
#问题和答案的分割符号，默认---
question_seperator = "---"
#卡片之间的分割符号，默认连续3个换行
note_seperator = "\r\n\r\n\r\n"
#css样式。有默认样式
css = ""
```





### 效果示例

---

<img src="https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231005564.png" alt="image-20210721231005564" style="zoom: 50%;" />![image-20210721231035512](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231035512.png)<img src="https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231005564.png" alt="image-20210721231005564" style="zoom: 50%;" />![image-20210721231035512](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231035512.png)![image-20210721231052965](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231052965.png)<img src="https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231005564.png" alt="image-20210721231005564" style="zoom: 50%;" />![image-20210721231035512](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231035512.png)![image-20210721231052965](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231052965.png)

<img src="https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231110394.png" alt="image-20210721231110394" style="zoom:50%;" />![image-20210721231130470](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231130470.png)<img src="https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231110394.png" alt="image-20210721231110394" style="zoom:50%;" />![image-20210721231130470](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231130470.png)<img src="https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231110394.png" alt="image-20210721231110394" style="zoom:50%;" />![image-20210721231130470](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231130470.png)<img src="https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231110394.png" alt="image-20210721231110394" style="zoom:50%;" />![image-20210721231130470](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231130470.png)![image-20210721231319894](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231319894.png)<img src="https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231110394.png" alt="image-20210721231110394" style="zoom:50%;" />![image-20210721231130470](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231130470.png)![image-20210721231319894](https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231319894.png)<img src="https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231110394.png" alt="image-20210721231110394" style="zoom:50%;" /><img src="https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231130470.png" alt="image-20210721231130470" /><img src="https://github.com/HuangJinAmm/AnkiNotes_md/image/README/image-20210721231110394.png" alt="image-20210721231110394" style="zoom:50%;" /><img src="E:\code\rust\AnkiNotes_md\image\README\image-20210721231130470.png" alt="image-20210721231130470" /><img src="E:\code\rust\AnkiNotes_md\image\README\image-20210721231130470.png" alt="image-20210721231130470" style="zoom:50%;" /><img src="E:\code\rust\AnkiNotes_md\image\README\image-20210721231202988.png" alt="image-20210721231202988" />
