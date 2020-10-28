# Setting Up a New Project
បង្កើតដូច ជំពូកទីមួយដែរ 
```
$ cargo new guessing_game --bin
cd guessing_game
```

# Processing a Guess
ផ្នែកដំបូងនៃកម្មវិធីល្បែងទស្សន៍ទាយ គឺ ការស្នើសុំការបញ្ចូលពីអ្នកប្រើប្រាស់ ដំណើរការ ការបញ្ចូល ហើយពិនិត្យមើលថាការបញ្ចូលគឺស្ថិតនៅក្នុងទំរង់ដែលរំពឹងទុក។ ដើម្បីចាប់ផ្តើម យើងអនុញ្ញាតិឲ្យអ្នកលេង បញ្ចូល ការទាយ។​ 

*src/main.rs*
```
use std::io;

fn main() {

println!("Guess the number!");

println!("Please input your guess.");

let mut guess = String::new();

io::stdin().read_line(&mut guess)

.expect("Failed to read line");

println!("You guessed: {}", guess);

}
```
កូដទទួលបាន ការទាយមួយពី អ្នកលេង ហើយបង្ហាញវា
បើយើងក្រលែកទៅមើលបណ្តុំកូដទាំងនោះយើងឃើញថាមានពត៌មានច្រើន ដូចច្នោះមើលវាមួយៗ រហូតដល់ការបង្ហាញលទ្ធផល​ 
the io(input/output) ជា library ចូលទៅក្នុង scope. the io library មកពី standard library (ដោយដឹងថា std):

use std::io;
ដោយ default, Rust នាំមកនូវ ចំនួន២ ៣ ប្រភេទ ចូលទៅក្នុង scope នៃ គ្រប់ កម្មវិធី (in the prelude) 
ប្រសិនបើប្រភេទណាដែលអ្នកចង់ប្រើមិនមាននៅក្នុង prelude ដំបូងអ្នកត្រូវតែនាំយកប្រភេទនោះទៅដាក់ក្នុង scope explicitly ដោយប្រើ use statemen eg. ការប្រើប្រាស់ដូចជា *std::io* library ផ្តល់ឲ្យយើងនូវមុខងារមានប្រយោជន៍មួយចំនួន។

the main function ប្រែជាខេមរៈភាសាថា  មុខងារសំខាន់ វា ជាចំណុចចូលទៅក្នុង កម្មវិធី ៖
```
fn main() {

```

fn ជា syntax ប្រកាស់ a new function (មុខងារថ្មីមួយ) ជាមួយវង់ក្រចក() ការចង្អុលបង្ហាញនៅទីនោះ គ្មានប៉ារ៉ាម៉ែត្រទេ ហើយcurly bracket { ជាការចាប់ផ្តើម ជាខ្លួន របស់ function
ក្នុងជំពូកមុន println! គឺជា a macro ដែល prints a string ទៅកាន់ អេក្រង់ 

```
println!("Guess the number!");

println!("Please input your guess.");
```

ក្នុងកូដនេះ គឺ បង្ហាញ  prompt  ថាតើ Game នេះទាមទារការបញ្ចូលអ្វី​ពី អ្នកលេង 

# Storing Values with Variables
បន្ទាប់មកយើង បង្កើតកន្លែងសម្រាប់ផ្ទុក នូវការបញ្ចូលពីអ្នកប្រើប្រាស់ គឺបែបនេះ ៖ 
let mut guess = String::new();
ចំណាំ​  let statement សម្រាប់ការបង្កើត អថេរ ( variable)
ឩ let foo = bar ;
បន្ទាត់ហ្នឹងគេបង្កើត អថេរថ្មីមួយដោយឲ្យឈ្មោះថា foo by binds (ដោយការចង) វា ពី bar ក្នុង  Rust, variables(អថេរ) គឺ immutable(មិនអាចផ្លាស់ប្តូរបាន) ដោយ defaul