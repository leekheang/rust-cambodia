# មូលដ្ឋាន សម្រាប់អ្នកសរសេរភាសា Rust


##  បង្កើត  Project Directory មួយ

ក្នុងចំនុចនេះយើងនឹងបង្កើត គម្រោងមួយ ដែលអាចប្រើបាន​ក្នុងការងារជាក់ស្តែង  ការបង្កើតនេះមានលក្ខណៈ ងាយស្រួល ហើយយើងនឹងធ្វើវា ទៅ តាមប្រភេទ ប្រព័ន្ធប្រតិបត្តិការផ្សេងៗ ​


*  សម្រាប់លីនុច មេសអូអេស  For Linux and macOS, enter this:

    ```
    $ mkdir ~/projects
    $ cd ~/projects
    $ mkdir hello_world
    $ cd hello_world
    ```

*  សម្រាប់វិនដូរ​ CMD For Windows CMD, enter this:
   ```
    > mkdir "%USERPROFILE%\projects"
    > cd /d "%USERPROFILE%\projects"
    > mkdir hello_world
    > cd hello_world
    ```

* សម្រាប់វិនដូរ ប្រើ PowerShell For Windows PowerShell, enter this:

    ជាទូទៅយើងអាចជ្រើសរើសទីតាំង ដើម្បី បង្កើត Project Directory ក្នុងទីតាំងណាក៏បានដែរ អោយតែ សមស្រប់និងធានាថាវាមិនប៉ះពាល់ ចំពោះដំនើរការ ប្រព័ន្ធប្រតិបត្តិការរបស់យើង តែចំពោះការបង្ហាញខាងលើវាជាស្តង់ដារមួយដែលគេនិយាយប្រើប្រាស់

## ការសរសេរនិង ការដំនើរការ​កម្មវិធី RUST 

​ បង្កើត File មួយ ដែលមាន  extention rs  ឧ. main.rs 
បើក main.rs រួចសរសេរកូដ ចូល 

```
    fn main() {
        println!("Hello, world!");
    }
```
បន្ទាប់មក Save File នោះ ​រួច 
ចូលទៅការ Terminal (linux or macOS)
```
$ rustc main.rs
$ ./main
Hello, world!
```
 ចូលទៅការ CMD or Powershell (Window only)
```
> rustc main.rs
> .\main.exe
Hello, world!
```

## សួស្តី Cargo!
Cargo គឺជា Rust ដែលសម្រាប់បង្កើត System ឬ កញ្ចប់ Package manager ( កញ្ចប់កូដដែលអាចយកប្រើប្រាស់បាន)
Cargo មានប្រយោជន៍ខ្លាំង សម្រាប់ការបង្កើត គម្រោង និងជួយសម្រួល ចំពោះកិច្ចការផ្សេងៗ ដូចជា building កូដរបស់យើងដែលសរសេរហើយ ទាញយកដូចជា libraries​ ចំពោះការសរសេរកូដលក្ខណៈស្មុកស្មាញ យើងមិនអាច ខ្លះ Cargo បានឡើង ក៏ព្រោះថា យើងត្រូវការ នូវ ភាពអាស្រ័យ (dependencies) មួយចំនួនសម្រាប់ Project របស់យើង 
 
* ត្រួតពិនិត្យលើ ជំនាន់របស់ Cargo  
```
cargo --version
```


## បង្កើត គម្រោង ដោយ Cargo 

បន្តិចទៀតយើងនឹងអាចដឹងពីភាព ខុសគ្នារវាង ការបង្កើតកម្រោងពី មុន ហើយប្រៀបធៀបនិង Cargo ខុសគ្នាយ៉ាងណា​
គ្រប់ ប្រព័ន្ធប្រតិបត្តិការ ទាំងអស់ សរសេរដូចគ្នា៖ 

```
$ cargo new hello_cargo --bin
$ cd hello_cargo
```

បង្កើត binary executable​ ថ្មីមួយ ត្រូវបានហៅថា hello_cargo
ដែល --bin ជា argument(ប៉ារ៉ាមែទ័រ) ដាក់ ទៅហ្នឹង cargo ថ្មីហ្នឹងឯង​ រួចហើយបង្កើតបាន  executable application​ មួយ
(often just called a binary) as opposed to a library
ពួកយើងបាន គម្រោងមួយ​ដែលមានឈ្មោះថា​ សួស្តី៌ខារហ្គោ *hello_cargo*   ឥលូវ លូកពោះវា *hello_cargo*  ដើម្បីចង់ដឹងថានៅខាងក្នុង ប្រជេកមានអី ខ្លះ 
Cargo.toml file និង src ​directory មួយ
 main.rs file​ មួយ . មានការ initialized  Git ​repository​ ថ្មីមួយ ជាមួយ  .gitignore file