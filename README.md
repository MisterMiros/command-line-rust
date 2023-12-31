# Command-Line Rust exercises
Exercises from the *"Command-Line Rust" by Ken Youens-Clark* ([link](https://www.oreilly.com/library/view/command-line-rust/9781098109424/))

## Implemented tools

### True

* Location: [hello/src/bin/true.rs](hello/src/bin/true.rs)
* Description: Returns exit code of 0 (success).

### False

* Location: [hello/src/bin/false.rs](hello/src/bin/false.rs)
* Description: Returns exit code of 1 (failure).

### Echor

* Location: [echor/src/main.rs](echor/src/main.rs), [echor/src/lib.rs](echor/src/lib.rs)
* Description: Prints given strings.

### Catr
* Location: [catr/src/main.rs](catr/src/main.rs), [catr/src/lib.rs](catr/src/lib.rs)
* Description: Outputs contents of given files or stdin.

### Headr

* Location: [headr/src/main.rs](headr/src/main.rs), [headr/src/lib.rs](headr/src/lib.rs)
* Description: Outputs the top lines or bytes of given files or stdin.

### Wcr

* Location: [wcr/src/main.rs](wcr/src/main.rs), [wcr/src/lib.rs](wcr/src/lib.rs)
* Description: Counts the number of lines, words and bytes/chars of given files or stdin.

### Uniqr

* Location: [uniqr/src/main.rs](uniqr/src/main.rs), [uniqr/src/lib.rs](uniqr/src/lib.rs)
* Description: Shrunks duplicate lines with optional counting of given files or of stdin.

### Cutr

* Location: [cutr/src/main.rs](cutr/src/main.rs), [cutr/src/lib.rs](cutr/src/lib.rs)
* Description: Cuts out selected portions of each line of a file.

### Grepr

* Location: [grepr/src/main.rs](grepr/src/main.rs), [grepr/src/lib.rs](grepr/src/lib.rs)
* Description: Searches any given input files, selecting lines that match the pattern.

### Tailr

* Location: [tailr/src/main.rs](tailr/src/main.rs), [tailr/src/lib.rs](tailr/src/lib.rs)
* Description: Outputs the bottom lines or bytes of given files or stdin.

### Fortuner

* Location: [fortuner/src/main.rs](fortuner/src/main.rs), [fortuner/src/lib.rs](fortuner/src/lib.rs)
* Description: Outputs a random line from given files or stdin or all lines that match a pattern.

### Calr

* Location: [calr/src/main.rs](calr/src/main.rs), [calr/src/lib.rs](calr/src/lib.rs)
* Description: Prints a calendar of the given month and year or of the current month and year.

### Lsr

* Location: [lsr/src/main.rs](lsr/src/main.rs), [lsr/src/lib.rs](lsr/src/lib.rs)
* Description: Displays contents of given directories
* Note: I made all the development on Windows Subsystem for Linux platform and it doesn't support permissions by default. So I decided to change the tests to always expect full set of permissions, but I'm sure it'll work correctly on the Linux/Unix systems just fine. 