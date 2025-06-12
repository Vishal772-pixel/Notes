//Tricky Datatype of Rust ( string)


//First way to initialise a String 
fn main ()
{
let greetings:&str="Hello world";
print!("{}",greeting);

//second way to initialise the string

fn main()
let greetings:String=String::from("Hello world");
printf("{}",greeting);

//Ways to interate a String (2 ways as i know) 

//First way to iterate a string 
let char1:Option<char>=greeting.char().nth(0);
print!("char1:{}",char1.unwrap());
}

//Second way to iterate a String (better way)

let char1:Option<char>=greeting.char().nth(11);
match char1{
Some (c:char)=>print!("{}",c),
None =>println!("No character at index1");
}
