#[derive(Debug)]
enum MyError{
    NotPositive,
}

fn is_par(number: i32) -> Option<i32>{
    if number % 2 ==0 
    {
       return Some(number);
    }
    None
}

fn is_positive(number: i32) ->Result<i32, MyError>{
   if number < 0 
   {
    return Err(MyError::NotPositive);
   }
   Ok(number)
}

fn main(){
    let vector = [-1, 0, 11];
    let mut index = 0;

    while index < vector.len(){
        let number = vector[index];
        let result = is_positive(number);
        while let Ok(r) = result {
            if let Some(_) = is_par(r) {
                println!("{} este par.", r);
            } else {
                println!("{} nu este par.", r);
            }
            break;

        }
        if let Err(i) = result
        {
            println!("{:?}", i);
        }
        index+=1;
    }
}