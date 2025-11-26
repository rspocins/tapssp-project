
use mysql::*;
use mysql::prelude::*;
use webbrowser;
use std::path::Path;
mod upload;
use std::io;


#[derive(Debug, PartialEq, Eq)]
struct Book {
    title: String,
    path: String,
    history: bool,
    fiction: bool,
    education: bool,
}
// TO DO 

// STAY OPEN AFTER OPENING BOOK
// TRACK SECONDS READ



fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:Spoc979899))@127.0.0.1:3306/rustdb";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let mut customerQuery: String = String::new();
    let path = "C:";

    upload::main()?;


 println!(" welcome to the Library of Tamriel! ");

// list all books in the library
while true {


println!(" here are your options: \n (1) browser by genre \n (2) enter the title of the book you would like to read\n (3) QUIT ");
println!(" please enter genre or title or quit");
let mut input:String = readStdIN();



//mini split 
    //mini split 

    //mini split 

        //mini split 

            //mini split 

                //mini split 

    //mini split 


if (input.trim().contains("1"))|| (input.trim().to_lowercase().contains("genre" )  )    {

    // genre selection

    println!("Please select a genre: \n  History \n  Fiction \n  Education ");
    let mut genre_input:String = readStdIN();
     customerQuery = match genre_input.trim().to_lowercase().as_str() {
        "history" => "SELECT Title FROM library WHERE History = true ORDER BY Title".to_string(),
        "fiction" => "SELECT Title FROM library WHERE Fiction = true ORDER BY Title".to_string(),
        "education" => "SELECT Title FROM library WHERE Education = true ORDER BY Title".to_string(),
        _ => {
            println!(" invalid genre selection ");
        
        }
    };

    // Map each row (single column) to String
    let mut titles: Vec<String> = conn.query_map(
        &customerQuery,
        |title: String| title,
    )?;
    for title in &mut titles {
        *title = title.replace("-"," ");
    }
    // Print one per line
    println!("{}", titles.join(" ||  "));


    let mut inLoopInput:String = readStdIN();

     inLoopInput = inLoopInput.trim().to_lowercase();
    inLoopInput = inLoopInput.replace(" ", "-");

    let customer_query2 = format!("SELECT Path FROM library WHERE Title = '{}'", inLoopInput);
    let p1:String = match conn.exec_first(&customer_query2, ()){
        Ok(opt) => match opt {
            Some(path) => path,
            None => {
                println!("Book not found");
                return Ok(());
            }
        },
        Err(e) => {
            eprintln!("Error executing query: {}", e);
            return Ok(());
        }
    };
    println!(" selected path: {}", p1);
    customerQuery = customer_query2;
    



    // just got QUERY
// nect 
    // get path of selected book
//    outside of if else

} else if (input.trim().contains("2") ||  (input.trim().to_lowercase().contains("title"))) {

    // title selection
    println!(" please enter the title of the book you would like to read: ");
    let mut title_input:String = readStdIN();
    title_input = title_input.trim().to_lowercase();
    title_input = title_input.replace(" ", "-");
    
 let path_opt: Option<(String,)> = conn.exec_first(
        "SELECT Path FROM library WHERE Title = :title",
        params! { "title" => title_input},
    )?;
    let path = path_opt.map(|(p,)| p);
    println!("{:?}", path);

}else if input.trim() == "3" || input.trim().to_lowercase() == "quit" {
    println!("Quitting the program.");
    return Ok(());
}else{
    println!("  invalid selection ");}


// and were back /
// and were back /
    //and were back /
        // and were back /
            // and were back /
                // and were back /


// execute query to get path


if !(customerQuery.is_empty()){
    println!("{}", customerQuery);
let selected_path: String = conn.query_first(&customerQuery)?.ok_or("Book not found")?;


  


println!(" opening book at path: {}", selected_path);


    //  open the file in the default web browser
     //  open the file in the default web browser
      //  open the file in the default web browser
       //  open the file in the default web browser
        //  open the file in the default web browser
         //  open the file in the default web browser
          //  open the file in the default web browser
           //  open the file in the default web browser


     let url = format!("file:///{}", selected_path.replace("\\", "/"));

     println!("Opening: {}", url);

     match webbrowser::open(&url) {
         Ok(_) => println!("Opened HTML file in your browser."),
         Err(e) => eprintln!("Failed to open file: {}", e),
    }

}
}// end while true
    Ok(())

}


fn readStdIN() -> (String) {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("You typed: {}", input.trim());

    return input.trim().to_string();
}
