// digital garden module has 
mod write;

//publicly expose the function write from the module write
pub use write::write;
//very similar to 
//export { write } from './write';

