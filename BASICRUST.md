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

 * Note 
 Git គឺជា Commond Version Control System យើងអាចធ្វើការប្តូរ Cargo new ទៅដើម្បីប្រើ version control system ផ្សេងៗ ឬ មិនប្រើ Version control system ក៏បាន ដោយ ប្រើនូវ -- vcs flag ។ Run cargo new --help ដើម្បី ស្វែងយល់ពីជម្រើសផ្សេងៗ ទៀតរបស់ Cargo 

បើ *Cargo.toml* ក្នុង text editor ណាមួយ   អ្នកនឹងឃើញ 

```
[package]

name = "hello_cargo"

version = "0.1.0"

authors = ["Your Name <you@example.com>"]

[dependencies]
```
FIle នេះគឺជាទម្រង់ TOML (Tom’s Obvious, Minimal Language) 

ជាទម្រង់នៃការ Configuarationr របស់ Cargo 

 បន្ទាត់ទី១ [package] គឺក្បាលដំបូង នៃការជង្អុលបង្ហាញ ការធ្វើតាម(Following ) Statements មានការConfiguaring ក្នុង Package 
 បន្ទាត់ទី២ ពួកយើងអាចបន្ថែម ជម្រើសផ្សេងៗ ដែលចាំបាច់ ឬ ត្រូវកែប្រែ ​ ដែល ប្រាប់ទៅ  Compiler Compile កម្មវិធីរបស់យើង (name version authors ..)

បន្ទាត់ទី៣ [dependencies] ជាបន្ទាត់ចុងក្រោយ វាជាការាចាប់ផ្តើមនៃ ការជ្រើសរើសបញ្ជីរនៃ ចំនួនភាពអាស្រ័យនៃប្រូជែក(list Any of Project's dependencies) ក្នុង Rust ចំនួន Packages សំដៅទៅលើ *crates*  បង្ហាញ់នៅចំនុចបន្ទាប់

*src/main.rs*
```
fn main(){
    println!("Hello, world!");
}
```
Cargo បាន Generated​ កម្មវិធី Hello, world! មួយ ស្រដៀងអ្វីដែលយើងបានសរសេរពីមុន ភាពខុសគ្នាពីប្រជែកមុន គឺថា Cargo វាស្រួលជាងនឹងផ្តល់ឲ្យទីតាំងនៃកូដ ដូចជាក្នុង src directory មាន Cargo.toml confuguration file ក្នុង Top Directory 
និយាយទៅវាជួយជា Structure File និង Directory 

## Building and Running a Cargo Project ការកសាង និង តំនើរការ គម្រោង
កម្មវិធីដែលបង្កើតតាម Cargo ដែលចេញពី *hello_cargo* directory Build your project ដោយ ប្រើ Command : 

 ### cargo build
 
 Compiling hello_cargo 

```` 
Compiling hello_cargo  v0.1.0 (File:\\\Project\hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.20s
````
នេះជាពាក្យបញ្ជារ(command)ការបង្កើត ការ executable file​ មួយ ក្នុង (target/debug/hello_cargo​ target\debug\hello_cargo.exe on Windows) ដែលបានផ្ទុកក្នុង current directory 
You can run the executable with this command:
អាលូវយើងអាច Run The Executable ដូចនេះ 
### $ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
ផងដែលយើងអាច ប្រើ cargo run ដើម្បី compile the code(កូដដែលយើងបានសរសេរហើយ) ហើយ​ ។
### Cargo run 
```
$ cargo run

Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)

Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs

Running `target/debug/hello_cargò
```  

Cargo ផងដែល ផ្តោល ពាក្យបញ្ជារមួយដែលហៅថា cargo check. ពាក្យបញ្ជារនេះ ធ្វើការ 

 checks your code ដើម្បីប្រាកដថា​ វា compiles ប៉ុន្តែមិនសម្រាប់  produce an executable:

ហេតុអីបានយើងមិនចង់ប្រើ ការ executable(Cargo build) ? ជារឿយៗ ក៏ព្រោះថា Cargo Check វា លឿងជាង Cargo build (Executable )

We can build a project using cargo build or cargo check.
We can build and run a project in one step using cargo run.
Instead of saving the result of the build in the same directory as our

code, Cargo stores it in the target/debug directory.

## Building for Release ដាក់ចេញសម្រាប់ដំនើរការ
នៅពេល គម្រោងត្រូវបានរួចរាល់ អ្នកអាចប្រើ cargo build --release ដើម្បី compile វា ជាមួយ ការ optimizations 

executable ក្នុង target/release មាននូវ target/debug ការ optimizations ធ្វើឲ្យកូដរបស់យើងលឿន ប៉ុន្តែវាចំណាយពេលច្រើនជាងធម្មតា  ចឹងបានយើងមានពីវិធី មួយសម្រាប់ ការអវិវឌ្ឍន៍ (development) ពេលយើងចង់ rebuild អោយលឿន និងមួយទៀតសម្រាប់កម្មវិធីបញ្ជប់ (final Program) វាធ្វើឲ្យកម្មវិធីដើរឡើនតាមតែអាច 

