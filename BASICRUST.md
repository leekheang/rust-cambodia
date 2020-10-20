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
