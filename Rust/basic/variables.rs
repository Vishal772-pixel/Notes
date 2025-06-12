//Printing a Statement in Rust 
fn main ()
{
print!(" Welcome to Rust and Web3 Backend "); 
}

//Run through cammand "Cargo run" 
//Run through Cammand "Cargo build" to compile


// Get about signed and unsigned integers 
//limit of variables i8,i16,i32 and all

fn main ()
{
int x:i8= 5 ;// signed integer can be negative or positive 
int y:u32=1000 // unsigned integers can't be negative 
int z:f32= 100.8 // floating value or decimals 

print!("x:{},y:{},z:{}", x,y,z);
}


//Comman Loop in Rust 
fn main ()
{
let mut x:i32= 100
for i:i32 in  0...1000 {
i=i+100;
}
print!("x={}",x)
}

//Booleans 
fn main ()
{
let is_male:bool=true;
let mut is_above_18:bool=true;

if is_male {
print!("is male");}
if is_above_18 {
print!("is above 18");}
if is_male && is_above_18 {
print!( " is male and above 18");}

}

//Conditionals
fn even(){
let is_even:bool= true;
if is_even {
print!("Even");
}
else if !is_even{
print!("Odd");
}}







