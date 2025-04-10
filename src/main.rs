use vizia::{Application, ApplicationError};
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|_cx|{
       
    }).title("First Vizia").min_inner_size(Some((800, 480))).run()    
}
